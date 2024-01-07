use super::utils::define_procedures;
use crate::{
    evaluator::EnvRef,
    expr::{proc_result_value, Arity, Expr, Exprs, ProcedureResult},
};

define_procedures! {
    eqv = ("eqv?", eqv_fn, Arity::Exact(2)),
}

fn eqv_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let first = args.pop_front().unwrap();
    let second = args.pop_front().unwrap();
    let result = first == second;
    proc_result_value!(Expr::Boolean(result))
}
