use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{proc_result_value, Arity, Expr, Exprs, ListKind, ProcedureResult, list::List}, exprs,
};

define_procedures! {
    cons = ("cons", cons_fn, Arity::Exact(2)),
    car_ = ("car", car_fn, Arity::Exact(1)),
    cdr_ = ("cdr", cdr_fn, Arity::Exact(1)),
    make_list = ("make-list", make_list_fn, Arity::AtLeast(1)), 
    list_copy = ("list-copy", list_copy_fn, Arity::Exact(1)),
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
        .ok_or(runtime_error!("expected non-empty list for car"))?
        .clone();

    proc_result_value!(res)
}

fn cdr_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let list = match args.pop_front().unwrap() {
        Expr::List(list) => list,
        expr => return Err(runtime_error!("expected list for cdr, got {}", expr.kind())),
    };

    let (_, cdr_list) = list
        .split_first()
        .map_err(|_| runtime_error!("expected non-empty list for cdr"))?;

    let res = if cdr_list.len() == 1 && cdr_list.kind() == ListKind::Dotted {
        cdr_list.into_iter().next().unwrap()
    } else {
        Expr::List(cdr_list)
    };

    proc_result_value!(res)
}

fn make_list_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let k = args.pop_front().unwrap().into_integer().map_err(|_| {
        runtime_error!("make-list expected an integer as its first argument")
    })?;

    if args.is_empty() {
        let list = std::iter::repeat(Expr::Integer(0)).take(k as usize).collect::<Exprs>();
        proc_result_value!(Expr::List(List::new_proper(list)))
    } else {
        let fill = args.pop_front().unwrap();
        let list = std::iter::repeat(fill.clone()).take(k as usize).collect::<Exprs>();
        proc_result_value!(Expr::List(List::new_proper(list)))
    }
}

fn list_copy_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let obj = args.pop_front().unwrap();

    let copy_list = match obj {
        Expr::List(original_list) => {
            let cloned_elements: Exprs = original_list.iter().cloned().collect();
            Expr::List(List::new_proper(cloned_elements))
        }
        _ => obj,
    };

    proc_result_value!(copy_list)
}
