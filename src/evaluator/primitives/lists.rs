use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef, EvalResult},
    expr::{exprs, Arity, Expr, Exprs},
};

define_procedures! {
    cons = ("cons", cons_fn, Arity::Exact(2)),
    car_ = ("car", car_fn, Arity::Exact(1)),
    cdr_ = ("cdr", cdr_fn, Arity::Exact(1)),
    list_ = ("list", list_fn, Arity::Any),
}

fn cons_fn(mut args: Exprs, _: &mut EnvRef) -> EvalResult {
    let car = args.pop_front().unwrap();
    let cdr = args.pop_front().unwrap();

    let mut list = exprs![car];
    match cdr {
        Expr::List(cdr_list) => {
            list.extend(cdr_list);
            Ok(Expr::new_proper_list(list))
        }
        _ => {
            list.push_back(cdr);
            Ok(Expr::new_dotted_list(list))
        }
    }
}

fn car_fn(mut args: Exprs, _: &mut EnvRef) -> EvalResult {
    let list = match args.pop_front().unwrap() {
        Expr::List(list) => list,
        expr => return Err(runtime_error!("expected list for car, got {}", expr.kind())),
    };

    list.car()
        .cloned()
        .ok_or(runtime_error!("expected non-empty list for car"))
}

fn cdr_fn(mut args: Exprs, _: &mut EnvRef) -> EvalResult {
    let list = match args.pop_front().unwrap() {
        Expr::List(list) => list,
        expr => return Err(runtime_error!("expected list for cdr, got {}", expr.kind())),
    };

    if list.is_empty() {
        return Err(runtime_error!("expected non-empty list for cdr"));
    }

    let kind = list.kind();
    let list: Exprs = list.cdr().cloned().collect();

    if list.len() == 1 {
        Ok(list.into_iter().next().unwrap())
    } else {
        Ok(Expr::new_list(list, kind))
    }
}

fn list_fn(args: Exprs, _: &mut EnvRef) -> EvalResult {
    Ok(Expr::new_proper_list(args))
}
