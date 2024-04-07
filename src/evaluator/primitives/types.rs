use super::utils::define_procedures;
use crate::{
    evaluator::EnvRef,
    expr::{proc_result_value, Arity, Expr, Exprs, ProcedureResult},
};

define_procedures! {
    is_char = ("char?", is_char_fn, Arity::Exact(1)),
    is_number = ("number?", is_number_fn, Arity::Exact(1)),
    is_string = ("string?", is_string_fn, Arity::Exact(1)),
    is_pair = ("pair?", is_pair_fn, Arity::Exact(1)),
    is_procedure = ("procedure?", is_procedure_fn, Arity::Exact(1)),
    is_symbol = ("symbol?", is_symbol_fn, Arity::Exact(1)),
    is_input_port = ("input-port?", is_input_port_fn, Arity::Exact(1)),
    is_output_port = ("output-port?", is_output_port_fn, Arity::Exact(1)),
    is_port = ("port?", is_port_fn, Arity::Exact(1)),
}

fn is_char_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_type = expr.is_char();

    proc_result_value!(Expr::Boolean(is_type))
}

fn is_number_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_type = expr.is_integer() || expr.is_float();

    proc_result_value!(Expr::Boolean(is_type))
}

fn is_string_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_type = expr.is_string();

    proc_result_value!(Expr::Boolean(is_type))
}

fn is_pair_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_type = match expr {
        Expr::List(list) => !list.is_empty(),
        _ => false,
    };

    proc_result_value!(Expr::Boolean(is_type))
}

fn is_procedure_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_type = expr.is_procedure();

    proc_result_value!(Expr::Boolean(is_type))
}

fn is_symbol_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_type = expr.is_symbol();

    proc_result_value!(Expr::Boolean(is_type))
}

fn is_input_port_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_type = expr.is_input_port();

    proc_result_value!(Expr::Boolean(is_type))
}

fn is_output_port_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_type = expr.is_output_port();

    proc_result_value!(Expr::Boolean(is_type))
}

fn is_port_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.pop_front().unwrap();
    let is_type = expr.is_input_port() || expr.is_output_port();

    proc_result_value!(Expr::Boolean(is_type))
}
