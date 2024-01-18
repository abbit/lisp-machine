use std::{cell::RefCell, rc::Rc};

use super::utils::{define_procedures, resolve_path};
use crate::{
    evaluator::{error::runtime_error, procedure::ApplyProcedure, EnvRef},
    expr::{
        port::{InputPortTrait, OutputPortTrait, Port},
        proc_result_value, Arity, Expr, Exprs, ProcedureResult,
    },
};

define_procedures! {
    open_input_file = ("open-input-file", open_input_file_fn, Arity::Exact(1)),
    open_output_file = ("open-output-file", open_output_file_fn, Arity::Exact(1)),
    is_input_port = ("input-port?", is_input_port_fn, Arity::Exact(1)),
    is_output_port = ("output-port?", is_output_port_fn, Arity::Exact(1)),
    current_input_port = ("current-input-port", current_input_port_fn, Arity::Exact(0)),
    current_output_port = ("current-output-port", current_output_port_fn, Arity::Exact(0)),
    close_input_port = ("close-input-port", close_input_port_fn, Arity::Exact(1)),
    close_output_port = ("close-output-port", close_output_port_fn, Arity::Exact(1)),
    with_input_from_file = ("with-input-from-file", with_input_from_file_fn, Arity::Exact(2)),
    with_output_to_file = ("with-output-to-file", with_output_to_file_fn, Arity::Exact(2)),
    call_with_input_file = ("call-with-input-file", call_with_input_file_fn, Arity::Exact(2)),
    call_with_output_file = ("call-with-output-file", call_with_output_file_fn, Arity::Exact(2)),
}

fn open_input_file_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let file_path = args.pop_front().unwrap().into_string().map_err(|expr| {
        runtime_error!(
            "expected string as open-input-file argument, got {}",
            expr.kind()
        )
    })?;

    let resolved_path = resolve_path(&*file_path.borrow(), env)?;
    let port = Port::new_input_file(resolved_path).map_err(|e| e.to_string())?;

    proc_result_value!(Expr::new_port(port))
}

fn open_output_file_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let file_path = args.pop_front().unwrap().into_string().map_err(|expr| {
        runtime_error!(
            "expected string as open-output-file argument, got {}",
            expr.kind()
        )
    })?;

    let resolved_path = resolve_path(&*file_path.borrow(), env)?;
    let port = Port::new_output_file(resolved_path).map_err(|e| e.to_string())?;

    proc_result_value!(Expr::new_port(port))
}

fn is_input_port_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_input = matches!(expr, Expr::Port(port) if (*port).borrow().is_input());

    proc_result_value!(Expr::Boolean(is_input))
}

fn is_output_port_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_output = matches!(expr, Expr::Port(port) if (*port).borrow().is_output());

    proc_result_value!(Expr::Boolean(is_output))
}

fn current_input_port_fn(_: Exprs, env: &mut EnvRef) -> ProcedureResult {
    proc_result_value!(Expr::Port(env.current_input_port()))
}

fn current_output_port_fn(_: Exprs, env: &mut EnvRef) -> ProcedureResult {
    proc_result_value!(Expr::Port(env.current_output_port()))
}

fn close_input_port_fn(mut exprs: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let port = exprs.pop_front().unwrap().into_port().map_err(|expr| {
        runtime_error!(
            "expected port as close-input-port argument, got {}",
            expr.kind()
        )
    })?;

    let mut port = port.borrow_mut();
    if let Some(input_port) = port.as_input() {
        input_port
            .close()
            .map_err(|e| runtime_error!("got error while closing input port: {}", e))?;
        proc_result_value!(Expr::Void)
    } else {
        Err(runtime_error!("expected input port in close-input-port"))
    }
}

fn close_output_port_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let port = args.pop_front().unwrap().into_port().map_err(|expr| {
        runtime_error!(
            "expected port as close-output-port argument, got {}",
            expr.kind()
        )
    })?;

    let mut port = port.borrow_mut();
    if let Some(output_port) = port.as_output() {
        output_port
            .close()
            .map_err(|e| runtime_error!("got error while closing output port: {}", e))?;
        proc_result_value!(Expr::Void)
    } else {
        Err(runtime_error!("expected output port in close-output-port"))
    }
}

fn with_input_from_file_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let file_path = args.pop_front().unwrap().into_string().map_err(|expr| {
        runtime_error!(
            "expected string as first with-input-from-file argument, got {}",
            expr.kind()
        )
    })?;
    let thunk = args.pop_front().unwrap().into_procedure().map_err(|expr| {
        runtime_error!(
            "expected procedure as second with-input-from-file argument, got {}",
            expr.kind()
        )
    })?;

    let resolved_path = resolve_path(&*file_path.borrow(), env)?;
    let port = Port::new_input_file(resolved_path).map_err(|e| e.to_string())?;

    let mut eval_env = env.extend();
    eval_env.set_current_input_port(Rc::new(RefCell::new(port)));

    thunk.apply(Exprs::new(), &mut eval_env)
}

fn with_output_to_file_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let file_path = args.pop_front().unwrap().into_string().map_err(|expr| {
        runtime_error!(
            "expected string as first with-output-to-file argument, got {}",
            expr.kind()
        )
    })?;
    let thunk = args.pop_front().unwrap().into_procedure().map_err(|expr| {
        runtime_error!(
            "expected procedure as second with-output-to-file argument, got {}",
            expr.kind()
        )
    })?;

    let resolved_path = resolve_path(&*file_path.borrow(), env)?;
    let port = Port::new_output_file(resolved_path).map_err(|e| e.to_string())?;

    let mut eval_env = env.extend();
    eval_env.set_current_output_port(Rc::new(RefCell::new(port)));

    thunk.apply(Exprs::new(), &mut eval_env)
}

fn call_with_input_file_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let file_path = args.pop_front().unwrap().into_string().map_err(|expr| {
        runtime_error!(
            "expected string as first call-with-input-file argument, got {}",
            expr.kind()
        )
    })?;
    let thunk = args.pop_front().unwrap().into_procedure().map_err(|expr| {
        runtime_error!(
            "expected procedure as second call-with-input-file argument, got {}",
            expr.kind()
        )
    })?;

    let resolved_path = resolve_path(&*file_path.borrow(), env)?;
    let port = Port::new_input_file(resolved_path).map_err(|e| e.to_string())?;

    let mut eval_env = env.extend();
    eval_env.set_current_input_port(Rc::new(RefCell::new(port)));

    thunk.apply(Exprs::new(), &mut eval_env)
}

fn call_with_output_file_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let file_path = args.pop_front().unwrap().into_string().map_err(|expr| {
        runtime_error!(
            "expected string as first call-with-output-file argument, got {}",
            expr.kind()
        )
    })?;
    let thunk = args.pop_front().unwrap().into_procedure().map_err(|expr| {
        runtime_error!(
            "expected procedure as second call-with-output-file argument, got {}",
            expr.kind()
        )
    })?;

    let resolved_path = resolve_path(&*file_path.borrow(), env)?;
    let port = Port::new_output_file(resolved_path).map_err(|e| e.to_string())?;

    let mut eval_env = env.extend();
    eval_env.set_current_output_port(Rc::new(RefCell::new(port)));

    thunk.apply(Exprs::new(), &mut eval_env)
}
