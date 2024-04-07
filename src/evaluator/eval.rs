use super::{
    env::EnvRef,
    error::{runtime_error, EvalError},
};
use crate::{
    evaluator::procedure::ApplyProcedure,
    expr::{
        proc_result_tailcall, proc_result_value, AsExprs, Expr, Exprs, ListKind, ProcedureResult,
        ProcedureReturn,
    },
    utils::debug,
};

pub type EvalResult = Result<Expr, EvalError>;

pub fn expand_macros(expr: Expr, env: &mut EnvRef) -> EvalResult {
    if !expr.is_list() {
        return Ok(expr);
    }

    let original_expr = expr.clone();

    let list = expr.into_list().unwrap();
    if !list.is_proper() || list.is_empty() {
        return Ok(original_expr);
    }

    // safe to unwrap because we just checked that list is not empty
    let (first_expr, cdr_list) = list.split_first().unwrap();
    // list is proper, so list layout is same as exprs list
    let mut expanded_cdr_list = Exprs::new();
    for expr in cdr_list {
        expanded_cdr_list.push_back(expand_macros(expr, env)?);
    }

    match first_expr {
        Expr::Symbol(proc_name) => match env.get_macro(&proc_name) {
            // expr is a macro call
            // evaluate a macro and return result
            Some(macro_proc) => {
                debug!("expand_macros: {}", original_expr);
                let expanded_expr = match macro_proc.apply(expanded_cdr_list, env)? {
                    ProcedureReturn::Value(expr) => expr,
                    ProcedureReturn::TailCall(expr, mut eval_env) => {
                        debug!("expand_macro_tailcall_expr: {}", expr);
                        // using eval_expanede_expr here
                        // so we do not call make unnecessary expand_macros
                        eval_expanded_expr(expr, &mut eval_env)?
                    }
                };
                // return with expand_macros call
                // so we also expand macro calls from expanded macro
                expand_macros(expanded_expr, env)
            }
            // expr is a call, but not a macro call
            // so apply it and return result
            None => {
                expanded_cdr_list.push_front(Expr::new_symbol(proc_name));
                Ok(Expr::new_proper_list(expanded_cdr_list))
            }
        },
        // expr is a call, but not a macro call
        // so construct expr back and return it
        first_expr => {
            expanded_cdr_list.push_front(first_expr);
            Ok(Expr::new_proper_list(expanded_cdr_list))
        }
    }
}

pub fn eval_exprs<I: IntoIterator<Item = Expr>>(exprs: I, env: &mut EnvRef) -> EvalResult {
    let res = exprs
        .into_iter()
        .try_fold(Expr::Void, |_, expr| eval_expr(expr, env))?;

    Ok(res)
}

pub fn eval_exprs_with_tailcall(exprs: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let (exprs_but_tail, expr_tail) = match exprs.split_tail() {
        Some(split) => split,
        None => return proc_result_value!(Expr::Void),
    };

    for expr in exprs_but_tail {
        eval_expr(expr, env)?;
    }

    proc_result_tailcall!(expr_tail, env)
}

pub fn eval_expr(expr: Expr, env: &mut EnvRef) -> EvalResult {
    #[cfg(debug_assertions)]
    let original_expr = expr.clone(); // for debug purposes
    debug!("eval_expr: {}", expr);
    let expr = expand_macros(expr, env)?;
    let evaluated = eval_expanded_expr(expr, env)?;
    debug!("evaluated: {} -> {}", original_expr, evaluated);
    Ok(evaluated)
}

fn eval_expanded_expr(mut expr: Expr, env: &mut EnvRef) -> EvalResult {
    debug!("eval_expanded_expr: {}", expr);
    let mut env = env.clone();
    loop {
        match expr {
            Expr::Boolean(_) => return Ok(expr),
            Expr::Integer(_) => return Ok(expr),
            Expr::Float(_) => return Ok(expr),
            Expr::Char(_) => return Ok(expr),
            Expr::String(_) => return Ok(expr),
            Expr::InputPort(_) => return Ok(expr),
            Expr::OutputPort(_) => return Ok(expr),
            Expr::Symbol(symbol) => return eval_symbol(symbol, &mut env),
            Expr::List(list) => match list.kind() {
                ListKind::Proper => match eval_list(list.into(), &mut env)? {
                    ProcedureReturn::Value(e) => return Ok(e),
                    ProcedureReturn::TailCall(e, eval_env) => {
                        expr = e;
                        env = eval_env;
                        debug!("tailcall: {} in {:?}", expr, env);
                    }
                },
                ListKind::Dotted => return Err(runtime_error!("dotted list cannot be evaluated")),
            },
            Expr::Void => return Err(runtime_error!("void object cannot be evaluated")),
            Expr::Procedure(_) => {
                return Err(runtime_error!("procedure object cannot be evaluated"))
            }
        }
    }
}

fn eval_symbol(symbol: String, env: &mut EnvRef) -> EvalResult {
    debug!("eval_symbol: {}", symbol);
    let value = env
        .get_expr(&symbol)
        .ok_or(runtime_error!("undefined symbol: {}", symbol))?;
    debug!("symbol {} resolved to {}", symbol, value);
    Ok(value)
}

fn eval_list(mut list: Exprs, env: &mut EnvRef) -> ProcedureResult {
    debug!("eval_list: {:?}", list);

    let proc = list
        .pop_front()
        .ok_or(runtime_error!("empty list cannot be evaluated"))
        .and_then(|expr| eval_expr(expr, env))?
        .into_procedure()
        .map_err(|expr| {
            runtime_error!("expected procedure as first element of call, got {}", expr)
        })?;

    let mut args = list;
    if !proc.is_special_form() {
        // if procedure is not a special form, evaluate arguments first
        args = args
            .into_iter()
            .map(|arg| eval_expr(arg, env))
            .collect::<Result<Exprs, EvalError>>()?;
    }

    proc.apply(args, env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        evaluator::env,
        expr::{Body, NamedProcedure, Procedure, ProcedureParams},
        exprs, parser,
    };

    fn eval_str(source: &str, env: &mut EnvRef) -> EvalResult {
        let exprs = parser::parse_str(source).unwrap();
        eval_exprs(exprs, env)
    }

    #[test]
    fn create_lambda() {
        let source = "(lambda (x) (* x x))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        match result {
            Expr::Procedure(proc) => match proc {
                Procedure::Compound(compound) => {
                    assert_eq!(
                        compound.params,
                        ProcedureParams::Fixed(vec!["x".to_string()])
                    );
                    assert_eq!(
                        compound.body,
                        Box::new(Body::new_single(Expr::new_proper_list(exprs![
                            Expr::Symbol("*".to_string()),
                            Expr::Symbol("x".to_string()),
                            Expr::Symbol("x".to_string())
                        ])))
                    );
                }
                _ => panic!("expected compound procedure"),
            },
            _ => panic!("expected procedure"),
        }
    }

    #[test]
    fn define_lambda() {
        let source = "(define square (lambda (x) (* x x)))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);
        match env.get_expr("square") {
            Some(Expr::Procedure(proc)) => match proc {
                Procedure::Compound(compound) => {
                    assert_eq!(
                        compound.params,
                        ProcedureParams::Fixed(vec!["x".to_string()])
                    );
                    assert_eq!(
                        compound.body,
                        Box::new(Body::new_single(Expr::new_proper_list(exprs![
                            Expr::Symbol("*".to_string()),
                            Expr::Symbol("x".to_string()),
                            Expr::Symbol("x".to_string())
                        ])))
                    );
                }
                _ => panic!("expected compound procedure"),
            },
            _ => panic!("expected procedure"),
        }
    }

    #[test]
    fn define_proc_with_fixed_param() {
        let source = "(define (square x) (* x x))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);
        match env.get_expr("square") {
            Some(Expr::Procedure(proc)) => match proc {
                Procedure::Compound(compound) => {
                    assert_eq!(compound.name(), "square");
                    assert_eq!(
                        compound.params,
                        ProcedureParams::Fixed(vec!["x".to_string()])
                    );
                    assert_eq!(
                        compound.body,
                        Box::new(Body::new_single(Expr::new_proper_list(exprs![
                            Expr::Symbol("*".to_string()),
                            Expr::Symbol("x".to_string()),
                            Expr::Symbol("x".to_string())
                        ])))
                    );
                }
                _ => panic!("expected compound procedure"),
            },
            _ => panic!("expected procedure"),
        }
    }

    #[test]
    fn define_proc_with_variadic_param() {
        let source = "(define (f . x) x)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);
        match env.get_expr("f") {
            Some(Expr::Procedure(proc)) => match proc {
                Procedure::Compound(compound) => {
                    assert_eq!(compound.name(), "f");
                    assert_eq!(compound.params, ProcedureParams::Variadic("x".to_string()));
                    assert_eq!(
                        compound.body,
                        Box::new(Body::new_single(Expr::Symbol("x".to_string())))
                    );
                }
                _ => panic!("expected compound procedure"),
            },
            _ => panic!("expected procedure"),
        }
    }

    #[test]
    fn define_proc_with_mixed_params() {
        let source = "(define (f x . y) (list x y))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);
        match env.get_expr("f") {
            Some(Expr::Procedure(proc)) => match proc {
                Procedure::Compound(compound) => {
                    assert_eq!(compound.name(), "f");
                    assert_eq!(
                        compound.params,
                        ProcedureParams::Mixed(vec!["x".to_string()], "y".to_string())
                    );
                    assert_eq!(
                        compound.body,
                        Box::new(Body::new_single(Expr::new_proper_list(exprs![
                            Expr::Symbol("list".to_string()),
                            Expr::Symbol("x".to_string()),
                            Expr::Symbol("y".to_string())
                        ])))
                    );
                }
                _ => panic!("expected compound procedure"),
            },
            _ => panic!("expected procedure"),
        }
    }

    #[test]
    fn define_macro() {
        let source = "
            (define-macro (infix infixed)
                (list (car (cdr infixed)) (car infixed) (car (cdr (cdr infixed)))))
        ";
        let mut env = env::new_root_env();

        assert!(env.get_macro("infix").is_none());
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(result, Expr::Void);
        match env.get_macro("infix") {
            Some(Procedure::Compound(macro_proc)) => {
                assert_eq!(
                    macro_proc.params,
                    ProcedureParams::Fixed(vec!["infixed".to_string()])
                );
                assert_eq!(
                    macro_proc.body,
                    Box::new(Body::new_single(Expr::new_proper_list(exprs![
                        Expr::Symbol("list".to_string()),
                        Expr::new_proper_list(exprs![
                            Expr::Symbol("car".to_string()),
                            Expr::new_proper_list(exprs![
                                Expr::Symbol("cdr".to_string()),
                                Expr::Symbol("infixed".to_string())
                            ])
                        ]),
                        Expr::new_proper_list(exprs![
                            Expr::Symbol("car".to_string()),
                            Expr::Symbol("infixed".to_string())
                        ]),
                        Expr::new_proper_list(exprs![
                            Expr::Symbol("car".to_string()),
                            Expr::new_proper_list(exprs![
                                Expr::Symbol("cdr".to_string()),
                                Expr::new_proper_list(exprs![
                                    Expr::Symbol("cdr".to_string()),
                                    Expr::Symbol("infixed".to_string())
                                ])
                            ])
                        ])
                    ])))
                );
            }
            _ => panic!("expected macro procedure"),
        }
    }
}
