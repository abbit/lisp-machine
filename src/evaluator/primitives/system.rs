use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{proc_result_value, Arity, Expr, Exprs, ProcedureResult},
    parser,
};
use std::time::{SystemTime, UNIX_EPOCH};

define_procedures! {
    read = ("read", read_fn, Arity::Exact(0)),
    read_line = ("read-line", read_line_fn, Arity::Exact(0)),
    display = ("display", display_fn, Arity::Exact(1)),
    newline = ("newline", newline_fn, Arity::Exact(0)),
    exit = ("exit", exit_fn, Arity::Exact(0)),
    current_second = ("current-second", current_second_fn, Arity::Exact(0)),
}

fn exit_fn(_: Exprs, _: &mut EnvRef) -> ProcedureResult {
    std::process::exit(0);
}

fn read_input() -> std::io::Result<String> {
    let mut input = String::new();
    let res = std::io::stdin().read_line(&mut input);

    res.map(|_| input)
}

fn read_line_fn(_: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let input = read_input().map_err(|e| runtime_error!("Could not read input: {}", e))?;
    let input = match input.strip_suffix('\n') {
        Some(input) => input.to_string(),
        None => input,
    };

    proc_result_value!(Expr::new_string(input))
}

fn read_fn(_: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let input = read_input().map_err(|e| runtime_error!("Could not read input: {}", e))?;

    let expr = parser::parse_str(&input)
        .map_err(|e| runtime_error!("Could not parse input: {}", e))?
        .pop_front()
        .ok_or_else(|| runtime_error!("Could not parse input: empty input"))?;

    proc_result_value!(expr)
}

fn display_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    for expr in args.iter() {
        match expr {
            Expr::Char(char) => print!("{}", char),
            expr => print!("{}", expr),
        }
    }
    proc_result_value!(Expr::Void)
}

fn newline_fn(_: Exprs, _: &mut EnvRef) -> ProcedureResult {
    println!();
    proc_result_value!(Expr::Void)
}

fn current_second_fn(_: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    proc_result_value!(Expr::Float(current_time))
}
