use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{port::InputPortTrait, proc_result_value, Arity, Expr, Exprs, ProcedureResult},
    parser,
};
use std::io::Write;

define_procedures! {
    read = ("read", read_fn, Arity::Range(0, 1)),
    read_char = ("read-char", read_char_fn, Arity::Range(0, 1)),
    read_string = ("read-string", read_string_fn, Arity::Range(0, 1)),
    write = ("write", write_fn, Arity::Range(1, 2)),
    write_char = ("write-char", write_char_fn, Arity::Range(1, 2)),
    write_string = ("write-string", write_string_fn, Arity::Range(1, 4)),
    display = ("display", write_fn, Arity::Range(1, 2)),
    newline = ("newline", newline_fn, Arity::Range(0 ,1)),
}

fn read_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let port = match args.pop_front() {
        Some(e) => e.into_port().map_err(|expr| {
            runtime_error!("expected port as read-string argument, got {}", expr.kind())
        })?,
        None => env.current_input_port(),
    };
    let input = port
        .borrow_mut()
        .as_input()
        .ok_or(runtime_error!("expected input port"))?
        .read_string()
        .map_err(|e| runtime_error!("Could not read input string: {}", e))?;

    let expr = parser::parse_str(&input)
        .map_err(|e| runtime_error!("Could not parse input: {}", e))?
        .pop_front()
        .ok_or(runtime_error!("Could not parse input: empty input"))?;

    proc_result_value!(expr)
}

fn read_char_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let port = match args.pop_front() {
        Some(e) => e.into_port().map_err(|expr| {
            runtime_error!("expected port as read-char argument, got {}", expr.kind())
        })?,
        None => env.current_input_port(),
    };
    let char_result = port
        .borrow_mut()
        .as_input()
        .ok_or(runtime_error!("expected input port"))?
        .read_char()
        .map_err(|e| runtime_error!("Could not read character: {}", e))?;

    proc_result_value!(Expr::Char(char_result))
}

fn read_string_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let port = match args.pop_front() {
        Some(e) => e.into_port().map_err(|expr| {
            runtime_error!("expected port as read-string argument, got {}", expr.kind())
        })?,
        None => env.current_input_port(),
    };
    let input = port
        .borrow_mut()
        .as_input()
        .ok_or(runtime_error!("expected input port"))?
        .read_string()
        .map_err(|e| runtime_error!("Could not read input string: {}", e))?;

    proc_result_value!(Expr::new_string(input))
}

fn write_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let port = match args.pop_front() {
        Some(expr) => expr.into_port().map_err(|expr| {
            runtime_error!(
                "expected port as second display argument, got {}",
                expr.kind()
            )
        })?,
        None => env.current_output_port(),
    };

    let mut port = port.borrow_mut();
    let output_port = port
        .as_output()
        .ok_or(runtime_error!("expected output port"))?;
    write!(output_port, "{}", expr).map_err(|e| e.to_string())?;

    proc_result_value!(Expr::Void)
}

fn newline_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let port = match args.pop_front() {
        Some(expr) => expr.into_port().map_err(|expr| {
            runtime_error!("expected port as newline argument, got {}", expr.kind())
        })?,
        None => env.current_output_port(),
    };

    let mut port = port.borrow_mut();
    let output_port = port
        .as_output()
        .ok_or(runtime_error!("expected output port"))?;
    writeln!(output_port).map_err(|e| e.to_string())?;

    proc_result_value!(Expr::Void)
}

fn write_char_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let char_arg = args.pop_front().unwrap().into_char().map_err(|expr| {
        runtime_error!(
            "expected char as write-char argument, got {}",
            expr.kind()
        )
    })?;

    let port = match args.pop_front() {
        Some(expr) => expr.into_port().map_err(|expr| {
            runtime_error!(
                "expected port as second write-char argument, got {}",
                expr.kind()
            )
        })?,
        None => env.current_output_port(),
    };

    let mut port = port.borrow_mut();
    let output_port = port
        .as_output()
        .ok_or(runtime_error!("expected output port"))?;

    write!(output_port, "{}", char_arg).map_err(|e| e.to_string())?;

    proc_result_value!(Expr::Void)
}


fn write_string_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let string_arg = args.pop_front().unwrap().into_string().map_err(|expr| {
        runtime_error!(
            "expected string as write-string argument, got {}",
            expr.kind()
        )
    })?;

    let port = match args.pop_front() {
        Some(expr) => expr.into_port().map_err(|expr| {
            runtime_error!(
                "expected port as second write-string argument, got {}",
                expr.kind()
            )
        })?,
        None => env.current_output_port(),
    };

    let start = match args.pop_front() {
        Some(expr) => expr.into_integer().map_err(|expr| {
            runtime_error!(
                "expected integer as third write-string argument, got {}",
                expr.kind()
            )
        })? as usize,
        None => 0,
    };

    let end = match args.pop_front() {
        Some(expr) => expr.into_integer().map_err(|expr| {
            runtime_error!(
                "expected integer as fourth write-string argument, got {}",
                expr.kind()
            )
        })? as usize,
        None => string_arg.borrow().len(),
    };

    let string_value = string_arg.borrow();
    let substring = string_value[start..end].to_string();

    let mut port = port.borrow_mut();
    let output_port = port
        .as_output()
        .ok_or(runtime_error!("expected output port"))?;

    write!(output_port, "{}", substring).map_err(|e| e.to_string())?;

    proc_result_value!(Expr::Void)
}
