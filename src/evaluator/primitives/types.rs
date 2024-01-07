use super::utils::define_procedures;
use crate::{
    evaluator::EnvRef,
    expr::{Arity, Expr, Exprs, ProcedureResult, ProcedureReturn, proc_result_value},
};

define_procedures! {
    is_char = ("char?", is_char_fn, Arity::Exact(1)),
    is_number = ("number?", is_number_fn, Arity::Exact(1)),
    is_string = ("string?", is_string_fn, Arity::Exact(1)),
    is_pair = ("pair?", is_pair_fn, Arity::Exact(1)),
    is_procedure = ("procedure?", is_procedure_fn, Arity::Exact(1)),
}

fn is_char_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let value = args.pop_front().unwrap();
    let char = match value {
        Expr::Char(_) => true,
        _ => false,
    };

    Ok(Expr::Boolean(char)).map(ProcedureReturn::Value)
}

fn is_number_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let value = args.pop_front().unwrap();
    let number = match value {
        Expr::Integer(_) | Expr::Float(_) => true,
        _ => false,
    };

    Ok(Expr::Boolean(number)).map(ProcedureReturn::Value)
}

fn is_string_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let expr = args.into_iter().next().unwrap();

    let string = match expr {
        Expr::String(_) => true,
        _ => false,
    };

    Ok(Expr::Boolean(string)).map(ProcedureReturn::Value)
}

fn is_pair_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let arg = args.pop_front().unwrap();
    let result = match arg {
        Expr::List(list) => {
            if (list.is_proper() || list.is_dotted()) && !list.is_empty()  {
                Expr::Boolean(true)
            } else {
                Expr::Boolean(false)
            }
        }
        _ => Expr::Boolean(false),
    };
    proc_result_value!(result)
}

fn is_procedure_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let arg = args.pop_front().unwrap();
    let procedure = match arg {
        Expr::Procedure(_) => true,
        _ => false,
    };
    Ok(Expr::Boolean(procedure)).map(ProcedureReturn::Value)
}