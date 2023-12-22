use super::{
    env::EnvRef,
    error::{runtime_error, EvalError},
};
use crate::{
    evaluator::procedure::ApplyProcedure,
    expr::{Expr, Exprs, ListKind},
    utils::debug,
};

pub type EvalResult = Result<Expr, EvalError>;

pub fn eval_exprs<I: Iterator<Item = Expr>>(mut exprs: I, env: &mut EnvRef) -> EvalResult {
    exprs.try_fold(Expr::Void, |_, expr| eval_expr(expr, env))
}

pub fn eval_expr(expr: Expr, env: &mut EnvRef) -> EvalResult {
    debug!("eval_expr: {}", expr);
    match expr {
        Expr::Boolean(boolean) => Ok(Expr::Boolean(boolean)),
        Expr::Integer(int) => Ok(Expr::Integer(int)),
        Expr::Float(float) => Ok(Expr::Float(float)),
        Expr::Char(char) => Ok(Expr::Char(char)),
        Expr::String(str) => Ok(Expr::String(str)),
        Expr::Symbol(symbol) => eval_symbol(symbol, env),
        Expr::List(list) => match list.kind() {
            ListKind::Proper => eval_list(list.into(), env),
            ListKind::Dotted => Err(runtime_error!("dotted list cannot be evaluated")),
        },
        Expr::Void => Err(runtime_error!("void object cannot be evaluated")),
        Expr::Procedure(_) => Err(runtime_error!("procedure object cannot be evaluated")),
    }
}

fn eval_symbol(symbol: String, env: &mut EnvRef) -> EvalResult {
    debug!("eval_symbol: {}", symbol);
    let value = env
        .get(&symbol)
        .ok_or(runtime_error!("undefined symbol: {}", symbol))?;
    debug!("symbol {} resolved to {}", symbol, value);
    Ok(value)
}

fn eval_list(mut list: Exprs, env: &mut EnvRef) -> EvalResult {
    debug!("eval_call: {:?}", list);

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
        expr::{exprs, Body, NamedProcedure, Procedure, ProcedureParams},
        parser,
    };

    fn eval_str(source: &str, env: &mut EnvRef) -> Result<Expr, EvalError> {
        let exprs = parser::parse_str(source).unwrap();
        exprs
            .into_iter()
            .try_fold(Expr::Void, |_, expr| eval_expr(expr, env))
    }

    #[test]
    fn eval_number() {
        let source = "1";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(1));
    }

    #[test]
    fn eval_sum() {
        let source = "(+ 1 2)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(3));
    }

    #[test]
    fn eval_sum_multiple() {
        let source = "(+ 1 2 3 4 5)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(15));
    }

    #[test]
    fn eval_mult_multiple() {
        let source = "(* 1 2 3 4 5)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(120));
    }

    #[test]
    fn eval_sum_no_args() {
        let source = "(+)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(0));
    }

    #[test]
    fn eval_mult_no_args() {
        let source = "(*)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(1));
    }

    #[test]
    fn eval_complex_arithmethic() {
        let source = "(+ (- 1 (* 3 (/ 3 (- 2 1)))) (* 3 (+ 2 (- 1 2))))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(-5));
    }

    #[test]
    fn eval_anon_lambda() {
        let source = "((lambda (x) (* x x)) 3)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(9));
    }

    #[test]
    fn eval_complex_expr() {
        let source = "(define   square\r\n(lambda\t(x)\n\n(* x x)\n)\n) (square 3)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        match env.get("square") {
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

        assert_eq!(result, Expr::Integer(9));
    }

    #[test]
    fn eval_if() {
        let source = "(if (< 1 2) 1 2)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(1));
    }

    #[test]
    fn eval_begin() {
        let source = "(begin (define x 1) (+ x 1))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(2));
    }

    #[test]
    fn eval_define() {
        let source = "(define x 1)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);
        assert_eq!(env.get("x"), Some(Expr::Integer(1)));
    }

    #[test]
    fn eval_lambda() {
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
    fn eval_define_lambda() {
        let source = "(define square (lambda (x) (* x x)))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);

        match env.get("square") {
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
    fn eval_define_procedure() {
        let source = "(define (square x) (* x x))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);

        match env.get("square") {
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
    fn eval_list() {
        let source = "(list 1 (list 2 (+ 3 4)))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(
            result,
            Expr::new_proper_list(exprs![
                Expr::Integer(1),
                Expr::new_proper_list(exprs![Expr::Integer(2), Expr::Integer(7)])
            ])
        );
    }

    #[test]
    fn eval_apply() {
        let source = "(apply + (list 1 2))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(3));
    }

    #[test]
    fn eval_program_circle_area() {
        let program = "
                (define pi 314)
                (define r0 10)
                (define sqr (lambda (r) (* r r)))
                (define area (lambda (r) (* pi (sqr r))))
                (area r0)
        ";
        let mut env = env::new_root_env();

        let result = eval_str(program, &mut env).unwrap();
        assert_eq!(result, Expr::Integer((314 * 10 * 10) as i64));
    }

    #[test]
    fn eval_program_factorial() {
        let program = "
                (define fact (lambda (n) (if (= n 0) 1 (* n (fact (- n 1))))))
                (fact 5)
        ";

        let mut env = env::new_root_env();

        let result = eval_str(program, &mut env).unwrap();
        assert_eq!(result, Expr::Integer(120));
    }

    #[test]
    fn eval_program_fibonacci() {
        let program = "
                (define fib (lambda (n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))
                (fib 10)
        ";
        let mut env = env::new_root_env();

        let result = eval_str(program, &mut env).unwrap();
        assert_eq!(result, Expr::Integer(55));
    }

    #[test]
    fn eval_quoted_symbol() {
        let source = "'x";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(result, Expr::Symbol("x".to_string()));
    }

    #[test]
    fn eval_quoted_list() {
        let source = "'(lambda (x) (* x x))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::new_proper_list(exprs![
                Expr::Symbol("lambda".to_string()),
                Expr::new_proper_list(exprs![Expr::Symbol("x".to_string())]),
                Expr::new_proper_list(exprs![
                    Expr::Symbol("*".to_string()),
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("x".to_string())
                ])
            ])
        );
    }

    #[test]
    fn eval_quoted_proper_list_with_dot() {
        let source = "'(1 . (2 3 4))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::new_proper_list(exprs![
                Expr::Integer(1),
                Expr::Integer(2),
                Expr::Integer(3),
                Expr::Integer(4)
            ])
        );
    }

    #[test]
    fn eval_quasiquoted_symbol() {
        let source = "`x";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(result, Expr::Symbol("x".to_string()));
    }

    #[test]
    fn eval_quasiquoted_list() {
        let source = "`(lambda (x) (* x x))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::new_proper_list(exprs![
                Expr::Symbol("lambda".to_string()),
                Expr::new_proper_list(exprs![Expr::Symbol("x".to_string())]),
                Expr::new_proper_list(exprs![
                    Expr::Symbol("*".to_string()),
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("x".to_string())
                ])
            ])
        );
    }

    #[test]
    fn eval_quasiquoted_list_with_unquote() {
        let source = "`(+ 1 ,(+ 2 3))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::new_proper_list(exprs![
                Expr::Symbol("+".to_string()),
                Expr::Integer(1),
                Expr::Integer(5)
            ])
        )
    }

    #[test]
    fn eval_quasiquoted_list_with_unquote_splicing() {
        let source = "`(+ 1 ,@(list 2 3))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::new_proper_list(exprs![
                Expr::Symbol("+".to_string()),
                Expr::Integer(1),
                Expr::Integer(2),
                Expr::Integer(3)
            ])
        )
    }

    #[test]
    fn eval_lambda_with_variadic_param() {
        let source = "((lambda x x) '(1 2 3))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::new_proper_list(exprs![Expr::new_proper_list(exprs![
                Expr::Integer(1),
                Expr::Integer(2),
                Expr::Integer(3)
            ])])
        )
    }

    #[test]
    fn eval_lambda_with_mixed_params() {
        let source = "((lambda (x . y) (list x y)) 1 2 3)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::new_proper_list(exprs![
                Expr::Integer(1),
                Expr::new_proper_list(exprs![Expr::Integer(2), Expr::Integer(3)])
            ])
        )
    }

    #[test]
    fn eval_define_with_variadic_param() {
        let source = "(define (f . x) x)";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(result, Expr::Void);

        match env.get("f") {
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
    fn eval_define_with_mixed_params() {
        let source = "(define (f x . y) (list x y))";
        let mut env = env::new_root_env();
        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(result, Expr::Void);

        match env.get("f") {
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
}