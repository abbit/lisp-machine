use super::utils::define_special_forms;
use crate::{
    evaluator::{eval, EnvRef},
    expr::{proc_result_tailcall, proc_result_value, Arity, Expr, Exprs, ProcedureResult},
};

define_special_forms! {
    and = ("and", and_fn, Arity::Any),
    or = ("or", or_fn, Arity::Any),
}

fn and_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    if args.is_empty() {
        return proc_result_value!(Expr::Boolean(true));
    }

    let last_arg = args.pop_back().unwrap();
    for arg in args {
        let expr = eval::eval_expr(arg, env)?;
        if !expr.is_truthy() {
            return proc_result_value!(Expr::Boolean(false));
        }
    }

    if last_arg.is_truthy() {
        proc_result_tailcall!(last_arg, env)
    } else {
        proc_result_value!(Expr::Boolean(false))
    }
}

fn or_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    if args.is_empty() {
        return proc_result_value!(Expr::Boolean(false));
    }

    let last_arg = args.pop_back().unwrap();
    for arg in args {
        let expr = eval::eval_expr(arg, env)?;
        if expr.is_truthy() {
            return proc_result_value!(expr);
        }
    }

    if last_arg.is_truthy() {
        proc_result_tailcall!(last_arg, env)
    } else {
        proc_result_value!(Expr::Boolean(false))
    }
}
