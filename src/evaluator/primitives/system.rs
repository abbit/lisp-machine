use super::utils::{define_procedures, define_special_forms, read_exprs_from_path, resolve_path};
use crate::{
    evaluator::{error::runtime_error, eval, EnvRef},
    expr::{proc_result_tailcall, proc_result_value, Arity, Expr, Exprs, ProcedureResult},
};
use std::time::{SystemTime, UNIX_EPOCH};

define_special_forms! {
    include = ("include", include_fn, Arity::AtLeast(1)),
}

define_procedures! {
    load = ("load", load_fn, Arity::Exact(1)),
    exit = ("exit", exit_fn, Arity::Exact(0)),
    current_second = ("current-second", current_second_fn, Arity::Exact(0)),
}

fn include_fn(args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let mut exprs = Exprs::new();
    for (arg, idx) in args.into_iter().zip(1..) {
        let src_path = arg.into_string().map_err(|expr| {
            runtime_error!(
                "expected strings as arguments for include, got {} at position {}",
                expr.kind(),
                idx
            )
        })?;
        let src_path = resolve_path(&*src_path.borrow(), env)?;

        exprs.extend(read_exprs_from_path(&src_path)?);
    }

    exprs.push_front(Expr::new_symbol("begin"));
    proc_result_tailcall!(Expr::new_proper_list(exprs), env)
}

fn load_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let src_path =
        args.pop_front().unwrap().into_string().map_err(|expr| {
            runtime_error!("expected string as load argument, got {}", expr.kind())
        })?;
    let src_path = resolve_path(&*src_path.borrow(), env)?;
    let exprs = read_exprs_from_path(&src_path)?;
    let mut eval_env = env.extend();
    eval_env.set_cwd(src_path.parent().unwrap().to_path_buf());
    let res = eval::eval_exprs(exprs, &mut eval_env)?;

    proc_result_value!(res)
}

fn exit_fn(_: Exprs, _: &mut EnvRef) -> ProcedureResult {
    std::process::exit(0);
}

fn current_second_fn(_: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    proc_result_value!(Expr::Float(current_time))
}
