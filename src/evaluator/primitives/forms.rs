use super::utils::define_special_forms;
use crate::{
    evaluator::{
        error::runtime_error,
        eval::{self, EvalResult},
        EnvRef, EvalError,
    },
    expr::{
        proc_result_value, Arity, Body, Expr, Exprs, ListKind, ProcedureResult, ProcedureReturn,
        {Procedure, ProcedureParams},
    },
};

define_special_forms! {
    define = ("define", define_fn, Arity::AtLeast(2)),
    set = ("set!", set_fn, Arity::Exact(2)),
    lambda = ("lambda", lambda_fn, Arity::Exact(2)),
    if_ = ("if", if_fn, Arity::Range(2, 3)),
    begin = ("begin", begin_fn, Arity::Any),
    quote = ("quote", quote_fn, Arity::Exact(1)),
    quasiquote = ("quasiquote", quasiquote_fn, Arity::Exact(1)),
    define_macro = ("define-macro", define_macro_fn, Arity::AtLeast(2)),
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

fn create_procedure(
    name: Option<String>,
    params: Expr,
    body: Body,
    env: &mut EnvRef,
) -> EvalResult {
    let params = match params {
        Expr::List(list) => {
            let kind = list.kind();
            let mut params = list
                .into_iter()
                .map(|expr| {
                    expr.into_symbol().map_err(|expr| {
                        runtime_error!("expected symbols in lambda params, got {}", expr.kind())
                    })
                })
                .collect::<Result<Vec<String>, EvalError>>()?;

            match kind {
                ListKind::Proper => ProcedureParams::Fixed(params),
                ListKind::Dotted => {
                    // safe to unwrap since improper list must have at least two elements
                    let variadic_param = params.pop().unwrap();
                    ProcedureParams::Mixed(params, variadic_param)
                }
            }
        }
        Expr::Symbol(param) => ProcedureParams::Variadic(param),
        _ => {
            return Err(runtime_error!(
                "expected list, symbol or dotted list as first argument for lambda"
            ))
        }
    };

    let procedure = Expr::Procedure(Procedure::new_compound(name, params, body, env));

    Ok(procedure)
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
    env.add(name.to_string(), procedure);

    proc_result_value!(Expr::Void)
}

fn set_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let symbol_expr = args.pop_front().unwrap();
    let body_expr = args.pop_front().unwrap();

    modify_env(symbol_expr, body_expr, env, ModifyEnv::Set)
}

fn lambda_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let params = args.pop_front().unwrap();
    let body: Body = args.into();

    create_procedure(None, params, body, env).map(ProcedureReturn::Value)
}

fn if_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let cond = args.pop_front().unwrap();
    let then = args.pop_front().unwrap();
    let else_ = args.pop_front();

    let cond = eval::eval_expr(cond, env)?;
    let ret = if cond.is_truthy() {
        ProcedureReturn::TailCall(then, env.clone())
    } else {
        match else_ {
            Some(expr) => ProcedureReturn::TailCall(expr, env.clone()),
            None => ProcedureReturn::Value(Expr::Void),
        }
    };

    Ok(ret)
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
                Some(Expr::Symbol(symbol)) if symbol == "unquote" => {
                    expr_list.pop_front(); // pop "unquote" symbol
                    let expr = expr_list
                        .pop_front()
                        .ok_or(runtime_error!("expected expression after unquote"))?;
                    new_list.push_back(eval::eval_expr(expr, env)?);
                }
                Some(Expr::Symbol(symbol)) if symbol == "unquote-splicing" => {
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

fn define_macro_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let name_and_params = args.pop_front().unwrap().into_list().map_err(|expr| {
        runtime_error!(
            "expected list as the first argument for define-macro, got {}",
            expr
        )
    })?;

    // check that all elements define-macro formals list are symbols
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
        runtime_error!("expected at least 1 argument for define-macro formals list, got 0")
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
    env.add_macro(name.to_string(), procedure.into_procedure().unwrap());

    proc_result_value!(Expr::Void)
}
