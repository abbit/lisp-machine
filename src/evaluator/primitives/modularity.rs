use std::fs;

use super::utils::{define_procedures, define_special_forms};
use crate::{
    evaluator::{
        error::runtime_error,
        eval::{self},
        EnvRef,
    },
    expr::{proc_result_value, Arity, Exprs, ProcedureResult},
    parser,
};

define_special_forms! {
    include = ("include", include_fn, Arity::AtLeast(1)),
}

define_procedures! {
    load = ("load", load_fn, Arity::Exact(1)),
}

fn include_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let mut paths = Vec::new();
    for (arg, idx) in args.into_iter().zip(1..) {
        let path = arg.into_string().map_err(|expr| {
            runtime_error!(
                "expected symbols in define procedure formals list, got {} at position {}",
                expr.kind(),
                idx
            )
        })?;
        paths.push(path);
    }

    Err(runtime_error!("`include` is not implemented :)"))
}

fn load_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let src_path =
        args.pop_front().unwrap().into_string().map_err(|expr| {
            runtime_error!("expected string as load argument, got {}", expr.kind())
        })?;
    let current_dir = std::env::current_dir()
        .map_err(|err| runtime_error!("failed to get current directory: {}", err))?;

    let src_path = current_dir.join(src_path);
    let src_path = src_path.canonicalize().map_err(|err| {
        runtime_error!(
            "failed to resolve path to file {}: {}",
            src_path.display(),
            err
        )
    })?;
    src_path
        .is_file()
        .then_some(())
        .ok_or_else(|| runtime_error!("load argument {} is not a file", src_path.display()))?;

    let src = fs::read_to_string(&src_path)
        .map_err(|err| runtime_error!("failed to read file {}: {}", src_path.display(), err))?;
    let exprs = parser::parse_str(&src)
        .map_err(|err| runtime_error!("failed to parse file {}: {}", src_path.display(), err))?;
    let res = eval::eval_exprs(exprs.into_iter(), env)?;

    proc_result_value!(res)
}
