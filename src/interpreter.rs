use crate::{
    ast::{Expr, Procedure},
    debug,
    environment::EnvRef,
    lexer::Lexer,
    parser::parse,
};

#[derive(Debug, PartialEq)]
pub enum EvalError {
    ParseError(String),
    RuntimeError(String),
}

impl std::error::Error for EvalError {}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EvalError::ParseError(err) => write!(f, "parse error: {}", err),
            EvalError::RuntimeError(err) => write!(f, "runtime error: {}", err),
        }
    }
}

pub type EvalResult = Result<Expr, EvalError>;

pub fn eval(source: &str, env: &mut EnvRef) -> EvalResult {
    let lexer = Lexer::new(source);
    let tokens: Vec<_> = lexer.collect();
    match parse(&tokens) {
        Ok((expr, _)) => eval_expr(&expr, env),
        Err(err) => Err(EvalError::ParseError(err.to_string())),
    }
}

pub fn eval_expr(expr: &Expr, env: &mut EnvRef) -> EvalResult {
    debug!("eval_expr: {}", expr);
    match expr {
        Expr::Void => Ok(Expr::Void),
        Expr::Procedure(_) => Ok(expr.clone()),
        Expr::Integer(int) => Ok(Expr::Integer(*int)),
        Expr::Float(float) => Ok(Expr::Float(*float)),
        Expr::Symbol(symbol) => eval_symbol(symbol, env),
        Expr::String(string) => eval_symbol(string, env),
        Expr::Char(char) => Ok(Expr::Char(*char)),
        Expr::Boolean(bool) => Ok(Expr::Boolean(*bool)),
        Expr::List(list) => {
            // evaluate the first expression in the list
            let operator = eval_expr(&list[0], env)?;
            // collect the remaining expressions in the list
            let operands: Vec<_> = list.iter().skip(1).cloned().collect();

            eval_call(&operator, &operands, env)
        }
        Expr::Quote(_) => todo!(),
    }
}

fn eval_symbol(symbol: &str, env: &mut EnvRef) -> EvalResult {
    match env.get(symbol) {
        Some(val) => {
            let result = val.clone();
            debug!("eval_symbol: {} -> {}", symbol, result);
            Ok(result)
        }
        None => Err(EvalError::RuntimeError(format!(
            "undefined symbol: {}",
            symbol
        ))),
    }
}

fn eval_call(operator: &Expr, args: &[Expr], env: &mut EnvRef) -> EvalResult {
    debug!("eval_call: {} with args {:?}", operator, args);

    let procedure = match operator {
        Expr::Procedure(_) => operator,
        _ => {
            return Err(EvalError::RuntimeError(
                "expected procedure as operator in call".to_string(),
            ))
        }
    };

    match procedure {
        Expr::Procedure(proc) => {
            debug!("calling {} with args {:?}", proc, args);
            proc.apply(args, env)
        }
        _ => Err(EvalError::RuntimeError(
            "expected procedure in call".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::ProcedureType, environment};

    use super::*;

    #[test]
    fn eval_number() {
        let source = "1";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(1));
    }

    #[test]
    fn eval_sum() {
        let source = "(+ 1 2)";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(3));
    }

    #[test]
    fn eval_sum_multiple() {
        let source = "(+ 1 2 3 4 5)";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(15));
    }

    #[test]
    fn eval_mult_multiple() {
        let source = "(* 1 2 3 4 5)";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(120));
    }

    #[test]
    fn eval_sum_no_args() {
        let source = "(+)";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(0));
    }

    #[test]
    fn eval_mult_no_args() {
        let source = "(*)";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(1));
    }

    #[test]
    fn eval_complex_arithmethic() {
        let source = "(+ (- 1 (* 3 (/ 3 (- 2 1)))) (* 3 (+ 2 (- 1 2))))";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(-5));
    }

    #[test]
    fn eval_anon_lambda() {
        let source = "((lambda (x) (* x x)) 3)";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(9));
    }

    #[test]
    fn eval_complex_expr() {
        let source = "(define   square\r\n(lambda\t(x)\n\n(* x x)\n)\n) (square 3)";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        match env.get("square") {
            Some(Expr::Procedure(proc)) => match proc.data {
                ProcedureType::Compound(compound) => {
                    assert_eq!(compound.params, vec!["x".to_string()]);
                    assert_eq!(
                        compound.body,
                        Box::new(Expr::List(vec![
                            Expr::Symbol("*".to_string()),
                            Expr::Symbol("x".to_string()),
                            Expr::Symbol("x".to_string()),
                        ]))
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
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(1));
    }

    #[test]
    fn eval_begin() {
        let source = "(begin (define x 1) (+ x 1))";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Integer(2));
    }

    #[test]
    fn eval_define() {
        let source = "(define x 1)";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);
        assert_eq!(env.get("x"), Some(Expr::Integer(1)));
    }

    #[test]
    fn eval_lambda() {
        let source = "(lambda (x) (* x x))";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        match result {
            Expr::Procedure(proc) => match proc.data {
                ProcedureType::Compound(compound) => {
                    assert_eq!(compound.params, vec!["x".to_string()]);
                    assert_eq!(
                        compound.body,
                        Box::new(Expr::List(vec![
                            Expr::Symbol("*".to_string()),
                            Expr::Symbol("x".to_string()),
                            Expr::Symbol("x".to_string()),
                        ]))
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
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);

        match env.get("square") {
            Some(Expr::Procedure(proc)) => match proc.data {
                ProcedureType::Compound(compound) => {
                    assert_eq!(compound.params, vec!["x".to_string()]);
                    assert_eq!(
                        compound.body,
                        Box::new(Expr::List(vec![
                            Expr::Symbol("*".to_string()),
                            Expr::Symbol("x".to_string()),
                            Expr::Symbol("x".to_string()),
                        ]))
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
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(result, Expr::Void);

        match env.get("square") {
            Some(Expr::Procedure(proc)) => match proc.data {
                ProcedureType::Compound(compound) => {
                    assert_eq!(proc.name, Some("square".to_string()));
                    assert_eq!(compound.params, vec!["x".to_string()]);
                    assert_eq!(
                        compound.body,
                        Box::new(Expr::List(vec![
                            Expr::Symbol("*".to_string()),
                            Expr::Symbol("x".to_string()),
                            Expr::Symbol("x".to_string()),
                        ]))
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
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

        assert_eq!(
            result,
            Expr::List(vec![
                Expr::Integer(1),
                Expr::List(vec![Expr::Integer(2), Expr::Integer(7)])
            ])
        );
    }

    #[test]
    fn eval_apply() {
        let source = "(apply + (list 1 2))";
        let mut env = environment::new_root_env();
        let result = eval(source, &mut env).unwrap();

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
        let mut env = environment::new_root_env();

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Expr::Integer((314 * 10 * 10) as i64));
    }

    #[test]
    fn eval_program_factorial() {
        let program = "
                (define fact (lambda (n) (if (= n 0) 1 (* n (fact (- n 1))))))
                (fact 5)
        ";

        let mut env = environment::new_root_env();

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Expr::Integer(120));
    }

    #[test]
    fn eval_program_fibonacci() {
        let program = "
                (define fib (lambda (n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))
                (fib 10)
        ";
        let mut env = environment::new_root_env();

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Expr::Integer(55));
    }
}
