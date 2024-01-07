use super::utils::{create_procedure, define_special_forms};
use crate::{
    evaluator::{
        error::runtime_error,
        eval::{self, EvalResult},
        procedure::ApplyProcedure,
        EnvRef,
    },
    expr::{
        exprs, proc_result_tailcall, proc_result_value, Arity, Body, Expr, Exprs, ListKind,
        ProcedureResult, ProcedureReturn,
    },
    utils::debug,
};
use std::collections::VecDeque;

define_special_forms! {
    define = ("define", define_fn, Arity::AtLeast(2)),
    set = ("set!", set_fn, Arity::Exact(2)),
    lambda = ("lambda", lambda_fn, Arity::Exact(2)),
    let_ = ("let", let_fn, Arity::AtLeast(1)),
    letrec = ("letrec", letrec_fn, Arity::AtLeast(1)),
    if_ = ("if", if_fn, Arity::Range(2, 3)),
    cond = ("cond", cond_fn, Arity::AtLeast(1)),
    begin = ("begin", begin_fn, Arity::Any),
    quote = ("quote", quote_fn, Arity::Exact(1)),
    quasiquote = ("quasiquote", quasiquote_fn, Arity::Exact(1)),
}

enum ModifyEnv {
    Add,
    Set,
}

fn modify_env(
    symbol_expr: Expr,
    body_expr: Expr,
    env: &mut EnvRef,
    mod_env: ModifyEnv,
) -> ProcedureResult {
    let symbol = symbol_expr.into_symbol().map_err(|expr| {
        runtime_error!(
            "expected symbol as first argument of define, got {}",
            expr.kind()
        )
    })?;
    let body = eval::eval_expr(body_expr, env)?;

    match mod_env {
        ModifyEnv::Add => {
            env.add(symbol, body);
            proc_result_value!(Expr::Void)
        }
        ModifyEnv::Set => env
            .set(symbol, body)
            .map(|_| ProcedureReturn::Value(Expr::Void))
            .map_err(|err| runtime_error!("{}", err)),
    }
}

fn define_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let first_arg = args.pop_front().unwrap();

    if let Expr::Symbol(_) = first_arg {
        // define a variable
        let value_expr = args.pop_front().unwrap();
        return modify_env(first_arg, value_expr, env, ModifyEnv::Add);
    }

    // if first arg is not a symbol (define variable)
    // it must be a list (define procedure)
    let name_and_params = first_arg.into_list().map_err(|expr| {
        runtime_error!(
            "expected symbol or list as the first argument for define, got {}",
            expr
        )
    })?;

    // check that all elements of define procedure formals list are symbols
    name_and_params
        .iter()
        .zip(1..)
        .try_for_each(|expr| match expr {
            (Expr::Symbol(_), _) => Ok(()),
            (expr, idx) => Err(runtime_error!(
                "expected symbols in define procedure formals list, got {} at position {}",
                expr.kind(),
                idx
            )),
        })?;

    let (name_expr, mut params) = name_and_params.split_first().map_err(|_| {
        runtime_error!("expected at least 1 argument for define procedure formals list, got 0")
    })?;
    // unwrap is safe since we checked `name_and_params` above
    let name = name_expr.into_symbol().unwrap();

    // define a procedure with variadic params
    let params_expr = match (params.kind(), params.len()) {
        (ListKind::Dotted, 1) => {
            // safe to unwrap since params.len() == 1
            params.pop_front().unwrap()
        }
        _ => Expr::List(params),
    };

    let body: Body = args.into();
    let procedure = create_procedure(Some(name.to_string()), params_expr, body, env)?;
    env.add(name.to_string(), procedure.into());

    proc_result_value!(Expr::Void)
}

fn set_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let symbol_expr = args.pop_front().unwrap();
    let body_expr = args.pop_front().unwrap();

    modify_env(symbol_expr, body_expr, env, ModifyEnv::Set)
}

fn let_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let first_arg = args.pop_front().unwrap();

    if first_arg.is_symbol() {
        let name = first_arg.into_symbol().unwrap();
        return named_let_form(name, args, env);
    }

    let mut bindings = first_arg.into_list().map_err(|expr| {
        runtime_error!(
            "expected bindings list as first argument of let, got {}",
            expr
        )
    })?;

    let mut eval_env = env.extend();
    while !bindings.is_empty() {
        let mut binding = bindings
            .pop_front()
            .unwrap()
            .into_list()
            .map_err(|expr| runtime_error!("expected list as binding in let, got {}", expr))?;

        if binding.len() != 2 {
            return Err(runtime_error!(
                "expected 2 elements in binding in let, got {}",
                binding.len()
            ));
        }

        let symbol = binding.pop_front().unwrap().into_symbol().map_err(|expr| {
            runtime_error!(
                "expected symbol as first element of binding in let, got {}",
                expr.kind()
            )
        })?;
        let value = binding.pop_front().unwrap();
        // use `env` here to disallow recursion
        let value = eval::eval_expr(value, env)?;
        eval_env.add(symbol, value);
    }

    eval::eval_exprs_with_tailcall(args, &mut eval_env)
}

// named let (let <symbol> <bindings> <body>)
fn named_let_form(name: String, mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    debug!(
        "evaluating named let form - name: \"{}\", args: {:?}",
        name, args
    );

    let mut bindings = args.pop_front().unwrap().into_list().map_err(|expr| {
        runtime_error!(
            "expected bindings list as first argument of let, got {}",
            expr
        )
    })?;

    let mut lambda_args = VecDeque::new();
    let mut lambda_params = Exprs::new();
    while !bindings.is_empty() {
        let mut binding = bindings
            .pop_front()
            .unwrap()
            .into_list()
            .map_err(|expr| runtime_error!("expected list as binding in let, got {}", expr))?;

        if binding.len() != 2 {
            return Err(runtime_error!(
                "expected 2 elements in binding in let, got {}",
                binding.len()
            ));
        }

        let symbol = binding.pop_front().unwrap().into_symbol().map_err(|expr| {
            runtime_error!(
                "expected symbol as first element of binding in let, got {}",
                expr.kind()
            )
        })?;
        lambda_args.push_back(symbol.clone());
        let value = binding.pop_front().unwrap();
        let value = eval::eval_expr(value, env)?;
        lambda_params.push_back(value);
    }

    let mut eval_env = env.extend();
    let proc = create_procedure(
        Some(name.clone()),
        Expr::new_proper_list(lambda_args.into_iter().map(Expr::Symbol).collect()),
        args.into(),
        &eval_env,
    )?;
    eval_env.add(name.clone(), proc.clone().into());
    proc.apply(lambda_params, &mut eval_env)
}

fn letrec_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let mut bindings = args.pop_front().unwrap().into_list().map_err(|expr| {
        runtime_error!(
            "expected bindings list as first argument of letrec, got {}",
            expr
        )
    })?;

    let mut eval_env = env.extend();
    while !bindings.is_empty() {
        let mut binding =
            bindings.pop_front().unwrap().into_list().map_err(|expr| {
                runtime_error!("expected list as binding in letrec, got {}", expr)
            })?;
        if binding.len() != 2 {
            return Err(runtime_error!(
                "expected 2 elements in binding in letrec, got {}",
                binding.len()
            ));
        }
        let symbol = binding.pop_front().unwrap().into_symbol().map_err(|expr| {
            runtime_error!(
                "expected symbol as first element of binding in letrec, got {}",
                expr.kind()
            )
        })?;
        let value = binding.pop_front().unwrap();
        // use `eval_env` here to allow recursion
        let value = eval::eval_expr(value, &mut eval_env)?;
        eval_env.add(symbol, value);
    }

    eval::eval_exprs_with_tailcall(args, &mut eval_env)
}

fn lambda_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let params = args.pop_front().unwrap();
    let body: Body = args.into();
    let proc = create_procedure(None, params, body, env)?;

    proc_result_value!(proc.into())
}

fn if_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let test = args.pop_front().unwrap();
    let then = args.pop_front().unwrap();
    let else_ = args.pop_front();

    let test = eval::eval_expr(test, env)?;
    if test.is_truthy() {
        proc_result_tailcall!(then, env.clone())
    } else {
        match else_ {
            Some(expr) => proc_result_tailcall!(expr, env.clone()),
            None => proc_result_value!(Expr::Void),
        }
    }
}

enum TestResult {
    Normal(Expr),
    Else,
}

impl TestResult {
    fn is_truthy(&self) -> bool {
        match self {
            Self::Normal(expr) => expr.is_truthy(),
            Self::Else => true,
        }
    }
}

fn cond_fn(args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let args_len = args.len();
    for (clause, idx) in args.into_iter().zip(1..) {
        let mut clause = clause
            .into_list()
            .map_err(|expr| runtime_error!("expected list as clause in cond, got {}", expr))?;

        if clause.is_empty() {
            return Err(runtime_error!("expected at least 1 element in clause",));
        }

        let test = clause.pop_front().unwrap();
        // check if clause test is special `else` clause
        let test_result = if test.is_specific_symbol("else") {
            // `else` clause should be last in cond
            if idx != args_len {
                return Err(runtime_error!("else clause must be last in cond"));
            }
            TestResult::Else
        } else {
            // if its normal clause test then evaluate it and check if it is a true value
            TestResult::Normal(eval::eval_expr(test, env)?)
        };

        if test_result.is_truthy() {
            return match test_result {
                TestResult::Normal(result_expr) => {
                    if clause.car().is_some_and(|e| e.is_specific_symbol("=>")) {
                        clause.pop_front(); // pop "=>" symbol
                        let proc_expr = clause.pop_front().ok_or(runtime_error!(
                            "expected expression after `=>` in cond clause"
                        ))?;
                        // if clause uses `=>` then next expr should evaluate to a procedure
                        let proc =
                            eval::eval_expr(proc_expr, env)?
                                .into_procedure()
                                .map_err(|expr| {
                                    runtime_error!(
                                        "expected procedure after `=>` in cond clause, got {}",
                                        expr.kind()
                                    )
                                })?;

                        // procedure should accept 1 argument
                        if !matches!(proc.arity(), Arity::Exact(1)) {
                            return Err(runtime_error!(
                                "expected procedure with 1 argument after `=>` in cond clause"
                            ));
                        }

                        // evaluate procedure with test result as argument
                        proc.apply(exprs![result_expr], env)
                    } else {
                        // normal clause, so just evaluates expressions in clause
                        let exprs = clause.into_exprs();
                        if exprs.is_empty() {
                            // if clause contains only `test` then check which type of clause it is
                            // for normal clause return value of `test` as result
                            return proc_result_value!(result_expr);
                        }
                        // if clause test is truthy and it has at least 1 expression
                        // then just evaluate expressions and return result of the last one
                        eval::eval_exprs_with_tailcall(exprs, env)
                    }
                }
                TestResult::Else => {
                    let exprs = clause.into_exprs();
                    if exprs.is_empty() {
                        // `else` clause should contain at least 1 expression
                        return Err(runtime_error!(
                            "else clause should contain at least 1 expression"
                        ));
                    }
                    // if clause test is truthy and it has at least 1 expression
                    // then just evaluate expressions and return result of the last one
                    eval::eval_exprs_with_tailcall(exprs, env)
                }
            };
        }
    }

    proc_result_value!(Expr::Void)
}

fn begin_fn(args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    eval::eval_exprs_with_tailcall(args, env)
}

fn quote_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    proc_result_value!(args.pop_front().unwrap())
}

fn quasiquote_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    match args.pop_front().unwrap() {
        Expr::List(list) => quasiquote_list(list.into(), env).map(ProcedureReturn::Value),
        expr => proc_result_value!(expr),
    }
}

fn quasiquote_list(list: Exprs, env: &mut EnvRef) -> EvalResult {
    let mut new_list = Exprs::new();

    for expr in list {
        if expr.is_list() {
            // if expr is a list, check if it's unquote call
            let mut expr_list = expr.into_list().unwrap();
            match expr_list.car() {
                Some(expr) if expr.is_specific_symbol("unquote") => {
                    expr_list.pop_front(); // pop "unquote" symbol
                    let expr = expr_list
                        .pop_front()
                        .ok_or(runtime_error!("expected expression after unquote"))?;
                    new_list.push_back(eval::eval_expr(expr, env)?);
                }
                Some(expr) if expr.is_specific_symbol("unquote-splicing") => {
                    expr_list.pop_front(); // pop "unquote-splicing" symbol
                    let expr = expr_list
                        .pop_front()
                        .ok_or(runtime_error!("expected expression after unquote"))?;

                    let list = eval::eval_expr(expr, env)?.into_list().map_err(|expr| {
                        runtime_error!("expected list after unquote-splicing, got {}", expr.kind())
                    })?;
                    new_list.extend(list);
                }
                _ => new_list.push_back(quasiquote_list(expr_list.into(), env)?),
            }
        } else {
            new_list.push_back(expr)
        }
    }

    Ok(Expr::new_proper_list(new_list))
}
