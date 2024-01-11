use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, procedure::ApplyProcedure, EnvRef},
    expr::{proc_result_tailcall, Arity, Exprs, ProcedureResult},
};

define_procedures! {
    apply = ("apply", apply_fn, Arity::AtLeast(2)),
    eval = ("eval", eval_fn, Arity::Exact(1)),
}

fn apply_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let proc = args.pop_front().unwrap().into_procedure().map_err(|expr| {
        runtime_error!(
            "expected procedure as first argument of apply, got {}",
            expr.kind()
        )
    })?;
    let last_list = args.pop_back().unwrap().into_list().map_err(|expr| {
        runtime_error!(
            "expected list as second argument for apply, got {}",
            expr.kind()
        )
    })?;

    args.extend(last_list);

    proc.apply(args, env)
}

fn eval_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    proc_result_tailcall!(args.pop_front().unwrap(), env)
}
