use lispdm::{exprs, Engine, Expr};

macro_rules! assert_absent_in_env {
    ($env:expr, $name:expr) => {
        assert!(!$env.has($name));
        assert!(!$env.has_macro($name));
    };
}

macro_rules! assert_present_in_env {
    ($env:expr, $name:expr) => {
        assert!($env.has($name) || $env.has_macro($name));
    };
}

#[test]
fn ensure_special_forms_defined_in_scheme_prelude() {
    let engine = Engine::new_without_prelude();
    let env = engine.env();
    assert_absent_in_env!(env, "let*");
    assert_absent_in_env!(env, "letrec*");
    assert_absent_in_env!(env, "case");
    assert_absent_in_env!(env, "when");
    assert_absent_in_env!(env, "unless");
    assert_absent_in_env!(env, "and");
    assert_absent_in_env!(env, "or");

    let engine = Engine::default();
    let env = engine.env();
    assert_present_in_env!(env, "let*");
    assert_present_in_env!(env, "letrec*");
    assert_present_in_env!(env, "case");
    assert_present_in_env!(env, "when");
    assert_present_in_env!(env, "unless");
    assert_present_in_env!(env, "and");
    assert_present_in_env!(env, "or");
}

#[test]
fn ensure_lexical_scope() {
    let source = "
        (define x 1)
        (define (foo) x)
        (define (bar) (define x 2) (foo))
        (bar)
    ";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 1);
}

#[test]
fn eval_number() {
    let source = "1";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 1);
}

#[test]
fn eval_sum() {
    let source = "(+ 1 2)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 3);
}

#[test]
fn eval_sum_multiple() {
    let source = "(+ 1 2 3 4 5)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 15);
}

#[test]
fn eval_mult_multiple() {
    let source = "(* 1 2 3 4 5)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 120);
}

#[test]
fn eval_sum_no_args() {
    let source = "(+)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 0);
}

#[test]
fn eval_mult_no_args() {
    let source = "(*)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 1);
}

#[test]
fn eval_complex_arithmethic() {
    let source = "(+ (- 1 (* 3 (/ 3 (- 2 1)))) (* 3 (+ 2 (- 1 2))))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, -5);
}

#[test]
fn eval_anon_lambda() {
    let source = "((lambda (x) (* x x)) 3)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 9);
}

#[test]
fn eval_if() {
    let source = "(if (< 1 2) 1 2)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 1);
}

#[test]
fn eval_begin() {
    let source = "(begin (define x 1) (+ x 1))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 2);
}

#[test]
fn eval_list() {
    let source = "(list 1 (list 2 (+ 3 4)))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();

    assert_eq!(
        result,
        Expr::new_proper_list(exprs![
            Expr::Integer(1),
            Expr::new_proper_list(exprs![Expr::Integer(2), Expr::Integer(7)])
        ])
    );
}

#[test]
fn eval_nested_empty_lits() {
    let source = "'(() . (() . ()))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(
        result,
        Expr::new_proper_list(exprs![
            Expr::new_proper_list(exprs![]),
            Expr::new_proper_list(exprs![])
        ])
    );
}

#[test]
fn eval_apply() {
    let source = "(apply + (list 1 2))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();

    assert_eq!(result, 3);
}

#[test]
fn eval_define() {
    let source = "(define x 1)";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    let env = engine.env();

    assert_eq!(result, Expr::Void);
    assert_eq!(env.get_expr("x"), Some(Expr::Integer(1)));
}

// ========================================================================
//                           `let` tests
// ========================================================================

#[test]
fn eval_let_simple() {
    let source = "(let ((x 2) (y 3)) (* x y))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 6);
}

#[test]
fn eval_let_nested() {
    let source = "(let ((x 2) (y 3)) (let ((x 7) (z (+ x y))) (* z x)))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 35);
}

#[test]
fn eval_let_named() {
    let source = "(let fac ((n 10)) (if (= n 0) 1 (* n (fac (- n 1)))))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 3628800);
}

#[test]
fn eval_letrec_mutual() {
    let source = "(letrec ((zero? (lambda (n) (= n 0)))
                               (even? (lambda (n) (if (zero? n) #t (odd? (- n 1)))))
                               (odd? (lambda (n) (if (zero? n) #f (even? (- n 1))))))
                          (even? 100))";
    let mut engine = Engine::default();
    let result = engine.eval::<bool>(source).unwrap().unwrap();
    assert!(result);
}

// ========================================================================
//                           `cond` tests
// ========================================================================

#[test]
fn eval_cond_simple() {
    let source = "(cond ((> 3 2) 'greater) ((< 3 2) 'less))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("greater"));
}

#[test]
fn eval_cond_no_else() {
    let source = "(cond ((> 3 3) 'greater) ((< 3 3) 'less))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::Void);
}

#[test]
fn eval_cond_else() {
    let source = "(cond ((> 3 3) 'greater) ((< 3 3) 'less) (else 'equal))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("equal"));
}

#[test]
fn eval_cond_with_arrow() {
    let source = "(cond (1 => (lambda (x) x)))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 1);
}

#[test]
fn eval_cond_no_clauses() {
    let source = "(cond)";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
    assert!(result.is_err());
}

#[test]
fn eval_cond_else_with_no_expr() {
    let source = "(cond (else))";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
    assert!(result.is_err());
}

#[test]
fn eval_cond_with_arrow_not_procedure() {
    let source = "(cond (1 => 1))";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
    assert!(result.is_err());
}

#[test]
fn eval_cond_with_arrow_incorrect_args() {
    let source = "(cond (1 => (lambda (x y) x)))";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
    assert!(result.is_err());
}

// ========================================================================
//                           `case` tests
// ========================================================================

#[test]
fn eval_case_simple() {
    let source = "(case (* 2 3) ((2 3 5 7) 'prime)
                             ((1 4 6 8 9) 'composite))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("composite"));
}

#[test]
fn eval_case_no_else() {
    let source = "(case (car '(c d))
        ((a) 'a)
        ((b) 'b))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::Void);
}

#[test]
fn eval_case_with_arrow() {
    let source = "(case (car '(2 1))
        ((1 3 5 7 9) => (lambda (x) (+ x 1)))
        ((2 4 6 8) => (lambda (x) (* x 2))))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 4);
}

#[test]
fn eval_case_else() {
    let source = "(case 10 ((2 3 5 7) 'prime)
                            ((1 4 6 8 9) 'composite)
                            (else 'other))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("other"));
}

#[test]
fn eval_case_else_single() {
    let source = "(case (car '(c d))
        (else => (lambda (x) x)))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("c"));
}

#[test]
fn eval_case_else_with_arrow() {
    let source = "(case (car '(c d))
        ((a e i o u) 'vowel)
        ((w y) 'semivowel)
        (else => (lambda (x) x)))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("c"));
}

// ========================================================================
//                           `match` tests
// ========================================================================

#[test]
fn eval_match_empty() {
    let source = "(match)";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
    assert!(result.is_err());
}

#[test]
fn eval_match_no_clauses() {
    let source = "(match 1)";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
    assert!(result.is_err());
}

#[test]
fn eval_match_unkown_pattern() {
    let source = "(match 1
        (,(unknown _) 'unknown))";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
    assert!(result.is_err());
}

#[test]
fn eval_match_no_match() {
    let source = "(match 1
        (0 'zero)
        (2 'two))";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
    println!("{:?}", result);
    assert!(result.is_err());
}

#[test]
fn eval_match_simple() {
    let source = "(match 1
        (0 'zero)
        (1 'one)
        (2 'two))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("one"));
}

#[test]
fn eval_match_exact() {
    let source = "(match '(1 2 3)
        ((1 2) 'one-two)
        ((1 2 3) 'one-two-three)
        ((1 2 3 4) 'one-two-three-four))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("one-two-three"));
}

#[test]
fn eval_match_any() {
    let source = "(match 23
        (0 'zero)
        (1 'one)
        (,_ 'something-else))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("something-else"));
}

#[test]
fn eval_match_any_bind() {
    let source = "(match 23
        (0 'zero)
        (1 'one)
        (,x x))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 23);
}

#[test]
fn eval_match_type() {
    let source = "(match 23
        (,(symbol _) 'symbol)
        (,(string _) 'string)
        (,(number _) 'number))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("number"));
}

#[test]
fn eval_match_type_bind() {
    let source = "(match 23
        (,(symbol sym) (list 'symbol sym))
        (,(string str) (list 'string str))
        (,(number num) (list 'number num)))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(
        result,
        Expr::new_proper_list(exprs![Expr::new_symbol("number"), Expr::Integer(23)])
    );
}

#[test]
fn eval_match_literal_string() {
    let source = r#"(match "hello"
        ("hello" 'hello)
        ("world" 'world))"#;
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("hello"));
}

#[test]
fn eval_match_literal_symbol() {
    let source = "(match 'hello
        (hello 1)
        (world 0))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 1);
}

#[test]
fn eval_match_literal_list() {
    let source = r#"(match '(1 ("2" 'three) ((4) 5))
        ((1 (2 3) ((4) 5)) 'int-list)
        ((1 ("2" 'three) ((4) 5)) 'mixed-list)
        (,_ 'no-match))"#;
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("mixed-list"));
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
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 55);
}

#[test]
fn eval_do_from_standard() {
    let source = "
            (let ((x '(1 3 5 7 9)))
             (do ((x x (cdr x))
                  (sum 0 (+ sum (car x))))
              ((null? x) sum)))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 25);
}

#[test]
fn eval_do_with_mutation() {
    let source = "(do ((i 0 (+ i 1))
                             (sum 0 (+ sum i)))
                            ((> i 10) sum)
                            (set! sum (+ sum i)))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 110);
}

#[test]
fn eval_do_with_no_step() {
    let source = "(do ((i 1)
                             (sum 0 (+ sum i)))
                            ((> sum 10) sum))";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 11);
}

#[test]
fn eval_do_with_no_expr() {
    let source = "(do ((i 0 (+ i 1))
                             (sum 0 (+ sum i)))
                            ((> i 10) sum)
                            ())";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
    assert!(result.is_err());
}

#[test]
fn eval_do_with_no_test() {
    let source = "(do ((i 0 (+ i 1))
                             (sum 0 (+ sum i)))
                            ()
                            (display sum))";
    let mut engine = Engine::default();
    let result = engine.eval::<()>(source);
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

    let result = engine.eval::<i64>(program).unwrap().unwrap();
    assert_eq!(result, 314 * 10 * 10);
}

#[test]
fn eval_program_factorial() {
    let program = "
                (define fact (lambda (n) (if (= n 0) 1 (* n (fact (- n 1))))))
                (fact 5)
        ";

    let mut engine = Engine::default();

    let result = engine.eval::<i64>(program).unwrap().unwrap();
    assert_eq!(result, 120);
}

#[test]
fn eval_program_fibonacci() {
    let program = "
                (define fib (lambda (n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))
                (fib 10)
        ";
    let mut engine = Engine::default();

    let result = engine.eval::<i64>(program).unwrap().unwrap();
    assert_eq!(result, 55);
}

// ========================================================================
//                           quotation tests
// ========================================================================

#[test]
fn eval_quoted_symbol() {
    let source = "'x";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("x"));
}

#[test]
fn eval_quoted_list() {
    let source = "'(lambda (x) (* x x))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
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
    let result = engine.eval::<Vec<i64>>(source).unwrap().unwrap();
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn eval_quasiquoted_symbol() {
    let source = "`x";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(result, Expr::new_symbol("x"));
}

#[test]
fn eval_quasiquoted_list() {
    let source = "`(lambda (x) (* x x))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
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
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(
        result,
        Expr::new_proper_list(exprs![
            Expr::new_symbol("+"),
            Expr::Integer(1),
            Expr::Integer(5)
        ])
    )
}

#[test]
fn eval_quasiquoted_list_with_unquote_splicing() {
    let source = "`(+ 1 ,@(list 2 3))";
    let mut engine = Engine::default();
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(
        result,
        Expr::new_proper_list(exprs![
            Expr::new_symbol("+"),
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
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
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
    let result = engine.eval::<Expr>(source).unwrap().unwrap();
    assert_eq!(
        result,
        Expr::new_proper_list(exprs![
            Expr::Integer(1),
            Expr::new_proper_list(exprs![Expr::Integer(2), Expr::Integer(3)])
        ])
    )
}

#[test]
fn eval_lambda_with_multiple_body_exprs() {
    let source = "((lambda (x) (define y 1) (set! y 10) (+ x y)) 1)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 11)
}

// ========================================================================
//                            `and` tests
// ========================================================================

#[test]
fn eval_and_simple() {
    let source = "(and #t #t #t)";
    let mut engine = Engine::default();
    let result = engine.eval::<bool>(source).unwrap().unwrap();
    assert!(result);
}

#[test]
fn eval_and_empty() {
    let source = "(and)";
    let mut engine = Engine::default();
    let result = engine.eval::<bool>(source).unwrap().unwrap();
    assert!(result);
}

#[test]
fn eval_and_single() {
    let source = "(and 1)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 1);
}

#[test]
fn eval_and_evaluation() {
    let source = "
        (define x 1)
        (and #t (set! x 10) 2)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 2);
    assert_eq!(engine.env().get::<i64>("x").unwrap().unwrap(), 10);
}

#[test]
fn eval_and_no_evaluation() {
    let source = "
        (define x 1)
        (and #f (set! x 10) 2)";
    let mut engine = Engine::default();
    let result = engine.eval::<bool>(source).unwrap().unwrap();
    assert!(!result);
    assert_eq!(engine.env().get::<i64>("x").unwrap().unwrap(), 1);
}

// ========================================================================
//                            `or` tests
// ========================================================================

#[test]
fn eval_or_simple() {
    let source = "(or #t #t #t)";
    let mut engine = Engine::default();
    let result = engine.eval::<bool>(source).unwrap().unwrap();
    assert!(result);
}

#[test]
fn eval_or_empty() {
    let source = "(or)";
    let mut engine = Engine::default();
    let result = engine.eval::<bool>(source).unwrap().unwrap();
    assert!(!result);
}

#[test]
fn eval_or_single() {
    let source = "(or 1)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 1);
}

#[test]
fn eval_or_no_evaluation() {
    let source = "
        (define x 1)
        (or 2 (set! x 10) 1)";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 2);
    assert_eq!(engine.env().get::<i64>("x").unwrap().unwrap(), 1);
}

#[test]
fn eval_or_evaluation() {
    let source = "
        (define x 1)
        (or #f (set! x 10) 3)";
    let mut engine = Engine::default();
    engine.eval::<()>(source).unwrap().unwrap();
    assert_eq!(engine.env().get::<i64>("x").unwrap().unwrap(), 10);
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
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 3);
}

#[test]
fn eval_macro_quasiquoted() {
    let source = "
            (define-macro (unless test . body) `(if ,test #f (begin ,@body)))

            (unless #f 1 2 3)
        ";
    let mut engine = Engine::default();

    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 3);
}

#[test]
fn eval_macro_nested() {
    let source = "
            (define-macro (when test . body) `(if ,test (begin ,@body) #f))
            (define-macro (unless test . body) `(if ,test #f (begin ,@body)))
            (when #t (unless #f 1 2 3))
        ";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 3);
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
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 3);
}

#[test]
fn eval_macro_with_list_call() {
    let source = "
            (define-macro (unless test . body) `(if ,test #f (begin ,@body)))
            (define-macro (ret str) (list 'unless '#f str))
            (ret 1)
        ";
    let mut engine = Engine::default();
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 1);
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
    let result = engine.eval::<i64>(source).unwrap().unwrap();
    assert_eq!(result, 1);
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
        let result = engine.eval::<i64>(&source).unwrap().unwrap();
        assert_eq!(result, 0);
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
        let result = engine.eval::<i64>(&source).unwrap().unwrap();
        assert_eq!(result, 0);
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
        let result = engine.eval::<i64>(&source).unwrap().unwrap();
        assert_eq!(result, 0);
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
        let result = engine.eval::<i64>(&source).unwrap().unwrap();
        assert_eq!(result, 0);
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
        let result = engine.eval::<i64>(&source).unwrap().unwrap();
        assert_eq!(result, 1);
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
        let result = engine.eval::<bool>(&source).unwrap().unwrap();
        assert_eq!(result, ITERATIONS % 2 == 0);
    }

    #[test]
    fn named_let_tco() {
        let source = format!("(let f ((x {})) (if (= x 0) 0 (f (- x 1))))", ITERATIONS);
        let mut engine = Engine::default();
        let result = engine.eval::<i64>(&source).unwrap().unwrap();
        assert_eq!(result, 0);
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
        let result = engine.eval::<bool>(&source).unwrap().unwrap();
        assert_eq!(result, ITERATIONS % 2 == 0);
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
        let result = engine.eval::<i64>(&source).unwrap().unwrap();
        assert_eq!(result, ITERATIONS * (ITERATIONS + 1) / 2);
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
        let result = engine.eval::<i64>(&source).unwrap().unwrap();
        assert_eq!(result, 0);
    }
}
