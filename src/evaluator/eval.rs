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
    debug!("expand_macros: {}", expr);

    if !expr.is_list() {
        return Ok(expr);
    }

    let original_expr = expr.clone();

    let list = expr.into_list().unwrap();
    if !list.is_proper() || list.is_empty() {
        return Ok(original_expr);
    }

    // safe to unwrap because we just checked that list is not empty
    let (macro_name_expr, args) = list.split_first().unwrap();
    match macro_name_expr {
        Expr::Symbol(macro_name) => match env.get_macro(&macro_name) {
            Some(macro_) => match macro_.apply(args.into(), env)? {
                ProcedureReturn::Value(expr) => Ok(expr),
                ProcedureReturn::TailCall(expr, mut eval_env) => eval_expr(expr, &mut eval_env),
            },
            None => Ok(original_expr),
        },
        _ => Ok(original_expr),
    }
}

pub fn eval_exprs<I: Iterator<Item = Expr>>(exprs: I, env: &mut EnvRef) -> EvalResult {
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
    let original_expr = expr.clone();
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
            Expr::Boolean(boolean) => return Ok(Expr::Boolean(boolean)),
            Expr::Integer(int) => return Ok(Expr::Integer(int)),
            Expr::Float(float) => return Ok(Expr::Float(float)),
            Expr::Char(char) => return Ok(Expr::Char(char)),
            Expr::String(str) => return Ok(Expr::String(str)),
            Expr::Symbol(symbol) => return eval_symbol(symbol, &mut env),
            Expr::List(list) => match list.kind() {
                ListKind::Proper => match eval_list(list.into(), &mut env)? {
                    ProcedureReturn::Value(e) => return Ok(e),
                    ProcedureReturn::TailCall(e, eval_env) => {
                        expr = e;
                        env = eval_env;
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
        .get(&symbol)
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
        expr::{exprs, Body, NamedProcedure, Procedure, ProcedureParams},
        parser,
    };

    fn eval_str(source: &str, env: &mut EnvRef) -> Result<Expr, EvalError> {
        let exprs = parser::parse_str(source).unwrap();
        eval_exprs(exprs.into_iter(), env)
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

    // ========================================================================
    //                           quotation tests
    // ========================================================================

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

    // ========================================================================
    //                     procedure params types tests
    // ========================================================================

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

    // ========================================================================
    //                            macros tests
    // ========================================================================

    #[test]
    fn eval_simple_macro() {
        let source = "
            (define-macro (infix infixed)
                (list (car (cdr infixed)) (car infixed) (car (cdr (cdr infixed)))))

            (infix (1 + 1))
        ";
        let mut env = env::new_root_env();

        assert!(env.get_macro("infix").is_none());
        let result = eval_str(source, &mut env).unwrap();
        assert!(env.get_macro("infix").is_some());
        assert_eq!(result, Expr::Integer(2));
    }

    #[test]
    fn eval_unless_macro() {
        let source = "
            (define-macro (unless test . body) `(if ,test #f (begin ,@body)))

            (unless #f 1 2 3)
        ";
        let mut env = env::new_root_env();

        let result = eval_str(source, &mut env).unwrap();
        assert_eq!(result, Expr::Integer(3));
    }

    // ========================================================================
    //                      proper tail call tests
    // use `cargo test --features test_tailcall` to run these tests
    // disabled by default because it takes a long time to run them
    // ========================================================================
    #[cfg(feature = "test_tailcall")]
    mod proper_tail_call {
        use super::*;

        #[test]
        fn if_tco() {
            let source = "
            (define (f x) (if (= x 0) 0 (f (- x 1))))
            (f 1000000)
            ";

            let mut env = env::new_root_env();
            // should not stack overflow
            let result = eval_str(source, &mut env).unwrap();
            assert_eq!(result, Expr::Integer(0));
        }

        #[test]
        fn begin_tco() {
            let source = "
            (begin (define (f x) (if (= x 0) 0 (f (- x 1)))) (f 1000000))
            ";
            let mut env = env::new_root_env();
            // should not stack overflow
            let result = eval_str(source, &mut env).unwrap();
            assert_eq!(result, Expr::Integer(0));
        }

        #[test]
        fn define_tco() {
            let source = "
            (define (f x) (if (= x 0) 0 (f (- x 1))))
            (f 1000000)
            ";
            let mut env = env::new_root_env();
            // should not stack overflow
            let result = eval_str(source, &mut env).unwrap();
            assert_eq!(result, Expr::Integer(0));
        }

        #[test]
        fn eval_tco() {
            let source = "
            (define (f x) (if (= x 0) 0 (f (- x 1))))
            (eval '(f 1000000))
            ";
            let mut env = env::new_root_env();
            // should not stack overflow
            let result = eval_str(source, &mut env).unwrap();
            assert_eq!(result, Expr::Integer(0));
        }

        #[test]
        fn apply_tco() {
            let source = "
            (define (get-f x)
             (if (= x 0)
               (lambda (x) x)
               (get-f (- x 1))))
            (apply (get-f 1000000) '(1000000))
            ";
            let mut env = env::new_root_env();
            // should not stack overflow
            let result = eval_str(source, &mut env).unwrap();
            assert_eq!(result, Expr::Integer(1000000));
        }

        #[test]
        fn mutual_recursion() {
            let source = "
            (define (even? n) (if (= n 0) #t (odd? (- n 1))))
            (define (odd? n) (if (= n 0) #f (even? (- n 1))))
            (even? 1000000)
            ";
            let mut env = env::new_root_env();
            // should not stack overflow
            let result = eval_str(source, &mut env).unwrap();
            assert_eq!(result, Expr::Boolean(true));
        }
    }
}
