use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{proc_result_value, Arity, Expr, Exprs, ProcedureResult},
};

define_procedures! {
    char_upcase = ("char-upcase", char_upcase_fn, Arity::Exact(1)),
    char_downcase = ("char-downcase", char_downcase_fn, Arity::Exact(1)),
    char_foldcase = ("char-foldcase", char_foldcase_fn, Arity::Exact(1)),
    is_char_whitespace = ("char-whitespace?", is_char_whitespace_fn, Arity::Exact(1)),
    is_char_upper_case = ("char-upper-case?", is_char_upper_case_fn, Arity::Exact(1)),
    is_char_lower_case = ("char-lower-case?", is_char_lower_case_fn, Arity::Exact(1)),
    is_char_alphabetic = ("char-alphabetic?", is_char_alphabetic_fn, Arity::Exact(1)),
    is_char_numeric = ("char-numeric?", is_char_numeric_fn, Arity::Exact(1)),
    digit_value = ("digit-value", digit_value_fn, Arity::Exact(1)),
}

fn char_upcase_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Char(char.to_ascii_uppercase()))
}

fn char_downcase_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Char(char.to_ascii_lowercase()))
}

fn char_foldcase_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Char(char.to_ascii_lowercase()))
}

fn is_char_whitespace_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Boolean(char.is_whitespace()))
}

fn is_char_upper_case_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Boolean(char.is_ascii_uppercase()))
}

fn is_char_lower_case_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Boolean(char.is_ascii_lowercase()))
}

fn is_char_alphabetic_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Boolean(char.is_alphabetic()))
}

fn is_char_numeric_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Boolean(char.is_numeric()))
}

fn digit_value_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char_arg = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("Expected a character"))?;

    let result = if char_arg.is_numeric() {
        let digit = char_arg.to_digit(10).unwrap();
        Expr::Integer(digit as i64)
    } else {
        Expr::Boolean(false)
    };

    proc_result_value!(result)
}
