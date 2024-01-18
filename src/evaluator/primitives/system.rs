use super::utils::{define_procedures, define_special_forms, read_exprs_from_path, resolve_path};
use crate::{
    evaluator::{error::runtime_error, eval, EnvRef},
    expr::{proc_result_tailcall, proc_result_value, Arity, Expr, Exprs, ProcedureResult},
};
use std::{time::{SystemTime, UNIX_EPOCH}, path::Path, fs, cell::RefCell, rc::Rc, env, collections::HashMap};

define_special_forms! {
    include = ("include", include_fn, Arity::AtLeast(1)),
}

define_procedures! {
    load = ("load", load_fn, Arity::Exact(1)),
    file_exists = ("file-exists?", file_exists_fn, Arity::Exact(1)),
    delete_file = ("delete-file", delete_file_fn, Arity::Exact(1)),
    error = ("error", error_fn, Arity::Exact(1)),
    exit = ("exit", exit_fn, Arity::Exact(0)),
    current_second = ("current-second", current_second_fn, Arity::Exact(0)),
    command_line = ("command-line", command_line_fn, Arity::Exact(0)),
    get_environment_variables = ("get-environment-variables", get_environment_variables_fn, Arity::Exact(0)),
    get_environment_variable = ("get-environment-variable", get_environment_variable_fn, Arity::Exact(1)),
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

fn file_exists_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let path_expr = args.get(0).ok_or_else(|| {
        runtime_error!("expected one argument for file-exists?, but got none")
    })?;

    let path_str = path_expr.clone()
        .into_string()
        .map_err(|expr| runtime_error!("expected string as argument for file-exists?, got {}", expr.kind()))?;

    let binding = path_str.borrow();
    let path = Path::new(&*binding);

    let exists = path.exists();

    proc_result_value!(Expr::Boolean(exists))
}

fn delete_file_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let path_expr = args.get(0).ok_or_else(|| {
        runtime_error!("expected one argument for delete-file, but got none")
    })?;

    let path_str = path_expr.clone()
        .into_string()
        .map_err(|expr| runtime_error!("expected string as argument for delete-file, got {}", expr.kind()))?;

    let path = path_str.borrow().clone();

    match fs::remove_file(&path) {
        Ok(_) => proc_result_value!(Expr::Void),
        Err(_) => {
            runtime_error!("No such file or directory: {}", path);
            proc_result_value!(Expr::Void)
        },
    }
}

fn command_line_fn(_: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let command_line_: Vec<String> = env::args().collect();
    let command_line_exprs: Exprs = command_line_.into_iter().map(|s| Expr::String(Rc::new(RefCell::new(s)))).collect();
    proc_result_value!(Expr::new_proper_list(command_line_exprs))

fn error_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let msg = args.pop_front().unwrap().into_string().map_err(|expr| {
        runtime_error!("expected string as argument of error, got {}", expr.kind())
    })?;
    Err(runtime_error!("{}", (*msg).borrow().clone()))
}

fn get_environment_variables_fn(_: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let env_vars: HashMap<String, String> = std::env::vars().collect();
    let mut result_list = Exprs::new();

    for (key, value) in env_vars {
        let key_expr = Expr::String(Rc::new(RefCell::new(key)));
        let value_expr = Expr::String(Rc::new(RefCell::new(value)));
        result_list.push_back(Expr::new_dotted_list(vec![key_expr, value_expr].into()));
    }

    proc_result_value!(Expr::new_dotted_list(result_list))
}

fn get_environment_variable_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let key_expr = args
        .get(0)
        .ok_or_else(|| runtime_error!("Expected one argument for get-environment-variable, but got none"))?;

    let key = key_expr.clone()
        .into_string()
        .map_err(|expr| runtime_error!("Expected string as argument for get-environment-variable, got {}", expr.kind()))?;

    let value = match std::env::var(&*key.borrow()) {
        Ok(val) => Expr::String(Rc::new(RefCell::new(val))),
        Err(_) => Expr::Void,
    };

    proc_result_value!(value)
}

