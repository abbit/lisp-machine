use lispdm::{exprs, Engine, Expr};

#[test]
fn eval_number() {
    let source = "1";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(1));
}

#[test]
fn eval_sum() {
    let source = "(+ 1 2)";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(3));
}

#[test]
fn eval_sum_multiple() {
    let source = "(+ 1 2 3 4 5)";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(15));
}

#[test]
fn eval_mult_multiple() {
    let source = "(* 1 2 3 4 5)";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(120));
}

#[test]
fn eval_sum_no_args() {
    let source = "(+)";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(0));
}

#[test]
fn eval_mult_no_args() {
    let source = "(*)";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(1));
}

#[test]
fn eval_complex_arithmethic() {
    let source = "(+ (- 1 (* 3 (/ 3 (- 2 1)))) (* 3 (+ 2 (- 1 2))))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(-5));
}

#[test]
fn eval_anon_lambda() {
    let source = "((lambda (x) (* x x)) 3)";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(9));
}

#[test]
fn eval_if() {
    let source = "(if (< 1 2) 1 2)";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(1));
}

#[test]
fn eval_begin() {
    let source = "(begin (define x 1) (+ x 1))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(2));
}

#[test]
fn eval_list() {
    let source = "(list 1 (list 2 (+ 3 4)))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

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
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();

    assert_eq!(result, Expr::Integer(3));
}

#[test]
fn eval_define() {
    let source = "(define x 1)";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    let env = engine.env();

    assert_eq!(result, Expr::Void);
    assert_eq!(env.get("x"), Some(Expr::Integer(1)));
}

// ========================================================================
//                           `let` tests
// ========================================================================

#[test]
fn eval_let_simple() {
    let source = "(let ((x 2) (y 3)) (* x y))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(6));
}

#[test]
fn eval_let_nested() {
    let source = "(let ((x 2) (y 3)) (let ((x 7) (z (+ x y))) (* z x)))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(35));
}

#[test]
fn eval_let_named() {
    let source = "(let fac ((n 10)) (if (= n 0) 1 (* n (fac (- n 1)))))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(3628800));
}

#[test]
fn eval_letrec_mutual() {
    let source = "(letrec ((zero? (lambda (n) (= n 0)))
                               (even? (lambda (n) (if (zero? n) #t (odd? (- n 1)))))
                               (odd? (lambda (n) (if (zero? n) #f (even? (- n 1))))))
                          (even? 100))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Boolean(true));
}

// ========================================================================
//                           `cond` tests
// ========================================================================

#[test]
fn eval_cond_simple() {
    let source = "(cond ((> 3 2) 'greater) ((< 3 2) 'less))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::new_symbol("greater"));
}

#[test]
fn eval_cond_no_else() {
    let source = "(cond ((> 3 3) 'greater) ((< 3 3) 'less))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Void);
}

#[test]
fn eval_cond_else() {
    let source = "(cond ((> 3 3) 'greater) ((< 3 3) 'less) (else 'equal))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::new_symbol("equal"));
}

#[test]
fn eval_cond_with_arrow() {
    let source = "(cond (1 => (lambda (x) x)))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(1));
}

#[test]
fn eval_cond_no_clauses() {
    let source = "(cond)";
    let mut engine = Engine::default();
    let result = engine.eval(source);
    assert!(result.is_err());
}

#[test]
fn eval_cond_else_with_no_expr() {
    let source = "(cond (else))";
    let mut engine = Engine::default();
    let result = engine.eval(source);
    assert!(result.is_err());
}

#[test]
fn eval_cond_with_arrow_not_procedure() {
    let source = "(cond (1 => 1))";
    let mut engine = Engine::default();
    let result = engine.eval(source);
    assert!(result.is_err());
}

#[test]
fn eval_cond_with_arrow_incorrect_args() {
    let source = "(cond (1 => (lambda (x y) x)))";
    let mut engine = Engine::default();
    let result = engine.eval(source);
    assert!(result.is_err());
}

// ========================================================================
//                           `do` tests
// ========================================================================

#[test]
fn eval_do_simple() {
    let source = "(do ((i 0 (+ i 1))
                             (sum 0 (+ sum i)))
                            ((> i 10) sum))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(55));
}

#[test]
fn eval_do_from_standard() {
    let source = "
            (let ((x '(1 3 5 7 9)))
             (do ((x x (cdr x))
                  (sum 0 (+ sum (car x))))
              ((null? x) sum)))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(25));
}

#[test]
fn eval_do_with_mutation() {
    let source = "(do ((i 0 (+ i 1))
                             (sum 0 (+ sum i)))
                            ((> i 10) sum)
                            (set! sum (+ sum i)))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(110));
}

#[test]
fn eval_do_with_no_step() {
    let source = "(do ((i 1)
                             (sum 0 (+ sum i)))
                            ((> sum 10) sum))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(11));
}

#[test]
fn eval_do_with_no_expr() {
    let source = "(do ((i 0 (+ i 1))
                             (sum 0 (+ sum i)))
                            ((> i 10) sum)
                            ())";
    let mut engine = Engine::default();
    let result = engine.eval(source);
    assert!(result.is_err());
}

#[test]
fn eval_do_with_no_test() {
    let source = "(do ((i 0 (+ i 1))
                             (sum 0 (+ sum i)))
                            ()
                            (display sum))";
    let mut engine = Engine::default();
    let result = engine.eval(source);
    assert!(result.is_err());
}

// ========================================================================
//                           simple program tests
// ========================================================================

#[test]
fn eval_program_circle_area() {
    let program = "
                (define pi 314)
                (define r0 10)
                (define sqr (lambda (r) (* r r)))
                (define area (lambda (r) (* pi (sqr r))))
                (area r0)
        ";
    let mut engine = Engine::default();

    let result = engine.eval(program).unwrap();
    assert_eq!(result, Expr::Integer((314 * 10 * 10) as i64));
}

#[test]
fn eval_program_factorial() {
    let program = "
                (define fact (lambda (n) (if (= n 0) 1 (* n (fact (- n 1))))))
                (fact 5)
        ";

    let mut engine = Engine::default();

    let result = engine.eval(program).unwrap();
    assert_eq!(result, Expr::Integer(120));
}

#[test]
fn eval_program_fibonacci() {
    let program = "
                (define fib (lambda (n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))
                (fib 10)
        ";
    let mut engine = Engine::default();

    let result = engine.eval(program).unwrap();
    assert_eq!(result, Expr::Integer(55));
}

// ========================================================================
//                           quotation tests
// ========================================================================

#[test]
fn eval_quoted_symbol() {
    let source = "'x";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Symbol("x".to_string()));
}

#[test]
fn eval_quoted_list() {
    let source = "'(lambda (x) (* x x))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
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
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
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
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Symbol("x".to_string()));
}

#[test]
fn eval_quasiquoted_list() {
    let source = "`(lambda (x) (* x x))";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
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
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
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
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
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
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
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
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(
        result,
        Expr::new_proper_list(exprs![
            Expr::Integer(1),
            Expr::new_proper_list(exprs![Expr::Integer(2), Expr::Integer(3)])
        ])
    )
}

// ========================================================================
//                            macros tests
// ========================================================================

#[test]
fn eval_macro_simple() {
    let source = "
            (define-macro (unless test . body) (list 'if test '#f (cons 'begin body)))
            (unless #f 1 2 3)
        ";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(3));
}

#[test]
fn eval_macro_quasiquoted() {
    let source = "
            (define-macro (unless test . body) `(if ,test #f (begin ,@body)))

            (unless #f 1 2 3)
        ";
    let mut engine = Engine::default();

    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(3));
}

#[test]
fn eval_macro_nested() {
    let source = "
            (define-macro (when test . body) `(if ,test (begin ,@body) #f))
            (define-macro (unless test . body) `(if ,test #f (begin ,@body)))
            (when #t (unless #f 1 2 3))
        ";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(3));
}

#[test]
fn eval_macro_nested_define() {
    let source = "
            (define-macro (when test . body) `(if ,test (begin ,@body) #f))
            (define (not x) (if x #f #t))
            (define-macro (unless test . body) `(when (not ,test) ,@body))
            (unless #f 1 2 3)
        ";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(3));
}

#[test]
fn eval_macro_with_list_call() {
    let source = "
            (define-macro (unless test . body) `(if ,test #f (begin ,@body)))
            (define-macro (ret str) (list 'unless '#f str))
            (ret 1)
        ";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(1));
}

#[test]
fn eval_macro_recursive() {
    let source = "
            (define (caar x) (car (car x)))
            (define (cdar x) (cdr (car x)))
            (define-macro (rec . clauses)
              (if (null? clauses)
                1
               `(rec ,@(cdr clauses))))
            (rec ((1 1) (2 2) (3 3)))
        ";
    let mut engine = Engine::default();
    let result = engine.eval(source).unwrap();
    assert_eq!(result, Expr::Integer(1));
}

// ========================================================================
//                      proper tail call tests
// use `cargo test --features test_tailcall` to run these tests
// disabled by default because it takes a long time to run them
// ========================================================================
#[cfg(feature = "test_tailcall")]
mod proper_tail_call {
    use super::*;

    const ITERATIONS: i64 = 345000;

    #[test]
    fn if_tco() {
        let source = format!(
            "
            (define (f x) (if (= x 0) 0 (f (- x 1))))
            (f {})
            ",
            ITERATIONS
        );

        let mut engine = Engine::default();
        // should not stack overflow
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Integer(0));
    }

    #[test]
    fn begin_tco() {
        let source = format!(
            "
            (begin (define (f x) (if (= x 0) 0 (f (- x 1)))) (f {}))
            ",
            ITERATIONS
        );
        let mut engine = Engine::default();
        // should not stack overflow
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Integer(0));
    }

    #[test]
    fn define_tco() {
        let source = format!(
            "
            (define (f x) (if (= x 0) 0 (f (- x 1))))
            (f {})
            ",
            ITERATIONS
        );
        let mut engine = Engine::default();
        // should not stack overflow
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Integer(0));
    }

    #[test]
    fn eval_tco() {
        let source = format!(
            "
            (define (f x) (if (= x 0) 0 (f (- x 1))))
            (eval '(f {}))
            ",
            ITERATIONS
        );
        let mut engine = Engine::default();
        // should not stack overflow
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Integer(0));
    }

    #[test]
    fn apply_tco() {
        let source = format!(
            "
            (define (get-f x)
             (if (= x 0)
               (lambda (x) x)
               (get-f (- x 1))))
            (apply (get-f {}) '(1))
            ",
            ITERATIONS
        );
        let mut engine = Engine::default();
        // should not stack overflow
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Integer(1));
    }

    #[test]
    fn mutual_recursion() {
        let source = format!(
            "
            (define (even? n) (if (= n 0) #t (odd? (- n 1))))
            (define (odd? n) (if (= n 0) #f (even? (- n 1))))
            (even? {})
            ",
            ITERATIONS
        );
        let mut engine = Engine::default();
        // should not stack overflow
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Boolean(ITERATIONS % 2 == 0));
    }

    #[test]
    fn named_let_tco() {
        let source = format!("(let f ((x {})) (if (= x 0) 0 (f (- x 1))))", ITERATIONS);
        let mut engine = Engine::default();
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Integer(0));
    }

    #[test]
    fn letrec_tco() {
        let source = format!(
            "(letrec ((zero? (lambda (n) (= n 0)))
                               (even? (lambda (n) (if (zero? n) #t (odd? (- n 1)))))
                               (odd? (lambda (n) (if (zero? n) #f (even? (- n 1))))))
                        (even? {}))",
            ITERATIONS
        );
        let mut engine = Engine::default();
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Boolean(ITERATIONS % 2 == 0));
    }

    #[test]
    fn do_loop() {
        let source = format!(
            "(do ((i 0 (+ i 1))
                         (sum 0 (+ sum i)))
                        ((> i {}) sum))",
            ITERATIONS
        );
        let mut engine = Engine::default();
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Integer(ITERATIONS * (ITERATIONS + 1) / 2));
    }

    #[test]
    fn do_tco() {
        let source = format!(
            "(define (f x) (if (= x 0)
                0
                (do ((i 0 (+ i 1)))
                 ((> i 5) (f (- x 1))))))
                (f {})",
            ITERATIONS / 5
        );
        let mut engine = Engine::default();
        let result = engine.eval(&source).unwrap();
        assert_eq!(result, Expr::Integer(0));
    }
}