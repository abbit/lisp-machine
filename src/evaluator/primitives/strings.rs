use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{proc_result_value, Arity, Expr, Exprs, ProcedureResult},
};

define_procedures! {
    string_set = ("string-set!", string_set_fn, Arity::Exact(3)),
    string_eq = ("string=?", string_eq_fn, Arity::Exact(2)),
    string_lt = ("string<?", string_lt_fn, Arity::Exact(2)),
    string_gt = ("string>?", string_gt_fn, Arity::Exact(2)),
    string_le = ("string<=?", string_le_fn, Arity::Exact(2)),
    string_ge = ("string>=?", string_ge_fn, Arity::Exact(2)),
}

fn string_set_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let string = args
        .pop_front()
        .unwrap()
        .into_string()
        .map_err(|_| runtime_error!("string-set! expected a string as its first argument"))?;
    let pos = args
        .pop_front()
        .unwrap()
        .into_integer()
        .map_err(|_| runtime_error!("string-set! expected a integer as its second argument"))?;
    let pos: usize = pos
        .try_into()
        .map_err(|_| runtime_error!("string-set! index out of bounds: {}", pos))?;
    let char =
        args.pop_front().unwrap().into_char().map_err(|_| {
            runtime_error!("string-set! expected a character as its third argument")
        })?;

    if pos >= string.borrow().len() {
        return Err(runtime_error!("string-set! index out of bounds: {}", pos));
    }

    string
        .borrow_mut()
        .replace_range(pos..(pos + 1), &char.to_string());

    proc_result_value!(Expr::Void)
}

fn string_eq_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 == str2))
}


fn string_lt_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 < str2))
}

fn string_gt_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 > str2))
}

fn string_le_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 <= str2))
}

fn string_ge_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 >= str2))
}