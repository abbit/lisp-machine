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
}

fn char_upcase_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args.pop_front().unwrap().into_char().map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Char(char.to_ascii_uppercase()))
}

fn char_downcase_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args.pop_front().unwrap().into_char().map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Char(char.to_ascii_lowercase()))
}

fn char_foldcase_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args.pop_front().unwrap().into_char().map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Char(char.to_ascii_lowercase()))
}

fn is_char_whitespace_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args.pop_front().unwrap().into_char().map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Boolean(char.is_whitespace()))
}

fn is_char_upper_case_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args.pop_front().unwrap().into_char().map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Boolean(char.is_ascii_uppercase()))
}

fn is_char_lower_case_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let char = args.pop_front().unwrap().into_char().map_err(|_| runtime_error!("Expected a character"))?;
    proc_result_value!(Expr::Boolean(char.is_ascii_lowercase()))
}