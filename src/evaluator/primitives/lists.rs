use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{exprs, proc_result_value, Arity, Expr, Exprs, ListKind, ProcedureResult},
};

define_procedures! {
    cons = ("cons", cons_fn, Arity::Exact(2)),
    car_ = ("car", car_fn, Arity::Exact(1)),
    cdr_ = ("cdr", cdr_fn, Arity::Exact(1)),
    list_ = ("list", list_fn, Arity::Any),
}

fn cons_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let car = args.pop_front().unwrap();
    let cdr = args.pop_front().unwrap();

    let mut list = exprs![car];
    let res = match cdr {
        Expr::List(cdr_list) => {
            list.extend(cdr_list);
            Expr::new_proper_list(list)
        }
        _ => {
            list.push_back(cdr);
            Expr::new_dotted_list(list)
        }
    };

    proc_result_value!(res)
}

fn car_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let list = match args.pop_front().unwrap() {
        Expr::List(list) => list,
        expr => return Err(runtime_error!("expected list for car, got {}", expr.kind())),
    };

    let res = list
        .car()
        .cloned()
        .ok_or(runtime_error!("expected non-empty list for car"))?;

    proc_result_value!(res)
}

fn cdr_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let list = match args.pop_front().unwrap() {
        Expr::List(list) => list,
        expr => return Err(runtime_error!("expected list for cdr, got {}", expr.kind())),
    };

    if list.is_empty() {
        return Err(runtime_error!("expected non-empty list for cdr"));
    }

    let kind = list.kind();
    let list_cdr: Exprs = list.cdr().cloned().collect();

    let res = if list_cdr.len() == 1 && kind == ListKind::Dotted {
        list_cdr.into_iter().next().unwrap()
    } else {
        Expr::new_list(list_cdr, kind)
    };

    proc_result_value!(res)
}

fn list_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    proc_result_value!(Expr::new_proper_list(args))
}
