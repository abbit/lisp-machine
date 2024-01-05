use super::utils::define_procedures;
use crate::{
    evaluator::EnvRef,
    expr::{Arity, Expr, Exprs, ProcedureResult, ProcedureReturn},
};

define_procedures! {
    is_char = ("char?", is_char_fn, Arity::Exact(1)),
    is_number = ("number?", is_number_fn, Arity::Exact(1)),
    is_string = ("string?", is_string_fn, Arity::Exact(1)),
}

fn is_char_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let value = args.pop_front().unwrap();
    let char = match value {
        Expr::Char(_) => true,
        _ => false,
    };

    Ok(Expr::Boolean(char)).map(ProcedureReturn::Value)
}

fn is_number_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let value = args.pop_front().unwrap();
    let number = match value {
        Expr::Integer(_) | Expr::Float(_) => true,
        _ => false,
    };

    Ok(Expr::Boolean(number)).map(ProcedureReturn::Value)
}

fn is_string_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.into_iter().next().unwrap();

    let string = match expr {
        Expr::String(_) => true,
        _ => false,
    };

    Ok(Expr::Boolean(string)).map(ProcedureReturn::Value)
}