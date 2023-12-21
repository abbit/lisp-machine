use super::utils::define_procedures;
use crate::{
    evaluator::{self, error::runtime_error, procedure::ApplyProcedure, EnvRef, EvalResult},
    expr::{Arity, Exprs},
};

define_procedures! {
    apply = ("apply", apply_fn, Arity::Exact(2)),
    eval = ("eval", eval_fn, Arity::Exact(1)),
}

fn apply_fn(mut args: Exprs, env: &mut EnvRef) -> EvalResult {
    let proc = args.pop_front().unwrap().into_procedure().map_err(|expr| {
        runtime_error!(
            "expected procedure as first argument of apply, got {}",
            expr.kind()
        )
    })?;
    let operands = args.pop_front().unwrap().into_list().map_err(|expr| {
        runtime_error!(
            "expected list as second argument for apply, got {}",
            expr.kind()
        )
    })?;

    proc.apply(operands.into(), env)
}

fn eval_fn(mut args: Exprs, env: &mut EnvRef) -> EvalResult {
    let expr = args.pop_front().unwrap();

    evaluator::eval_expr(expr, env)
}
