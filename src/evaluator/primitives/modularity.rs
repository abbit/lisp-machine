use std::{
    fs,
    path::{Path, PathBuf},
};

use super::utils::{define_procedures, define_special_forms};
use crate::{
    evaluator::{error::runtime_error, eval, EnvRef, EvalError},
    expr::{proc_result_tailcall, proc_result_value, Arity, Expr, Exprs, ProcedureResult},
    parser,
};

define_special_forms! {
    include = ("include", include_fn, Arity::AtLeast(1)),
}

define_procedures! {
    load = ("load", load_fn, Arity::Exact(1)),
}

fn resolve_import_path<P: AsRef<Path>>(
    relative_path: P,
    cwd: PathBuf,
) -> Result<PathBuf, EvalError> {
    let src_path = cwd.join(relative_path);
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

    Ok(src_path)
}

fn read_exprs<P: AsRef<Path>>(src_path: P) -> Result<Exprs, EvalError> {
    let src_path = src_path.as_ref();
    let src = fs::read_to_string(src_path)
        .map_err(|err| runtime_error!("failed to read file {}: {}", src_path.display(), err))?;

    parser::parse_str(&src)
        .map_err(|err| runtime_error!("failed to parse file {}: {}", src_path.display(), err))
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
        let src_path = resolve_import_path(&*src_path.borrow(), env.cwd())?;

        exprs.extend(read_exprs(&src_path)?);
    }

    exprs.push_front(Expr::new_symbol("begin"));
    proc_result_tailcall!(Expr::new_proper_list(exprs), env)
}

fn load_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let src_path =
        args.pop_front().unwrap().into_string().map_err(|expr| {
            runtime_error!("expected string as load argument, got {}", expr.kind())
        })?;
    let src_path = resolve_import_path(&*src_path.borrow(), env.cwd())?;
    let exprs = read_exprs(&src_path)?;
    let mut eval_env = env.extend();
    eval_env.set_cwd(src_path.parent().unwrap().to_path_buf());
    let res = eval::eval_exprs(exprs.into_iter(), &mut eval_env)?;

    proc_result_value!(res)
}
