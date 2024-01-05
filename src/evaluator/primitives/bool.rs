use super::utils::define_procedures;
use crate::{
    evaluator::EnvRef,
    expr::{Arity, Expr, Exprs, ProcedureResult, ProcedureReturn},
};

define_procedures! {
    and = ("and", and_fn, Arity::Any),
    or = ("or", or_fn, Arity::Any),
}

fn and_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let result = args.into_iter().all(|arg| match arg {
        Expr::Boolean(b) => b,
        _ => false,
    });

    Ok(Expr::Boolean(result)).map(ProcedureReturn::Value)
}

fn or_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let result = args.into_iter().any(|arg| match arg {
        Expr::Boolean(b) => b,
        _ => false,
    });

    Ok(Expr::Boolean(result)).map(ProcedureReturn::Value)
}