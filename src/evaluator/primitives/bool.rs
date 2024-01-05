use super::utils::define_procedures;
use crate::{
    evaluator::EnvRef,
    expr::{Arity, Expr, Exprs, ProcedureResult, ProcedureReturn},
};

define_procedures! {
    not = ("not", not_fn, Arity::Exact(1)),
}


fn not_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let arg = &args[0];

    match arg {
        Expr::Boolean(b) => Ok(Expr::Boolean(!b)),
        _ => Ok(Expr::Boolean(false)),
    }
    .map(ProcedureReturn::Value)
}
