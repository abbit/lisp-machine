use std::rc::Rc;

use super::utils::define_procedures;
use crate::{
    evaluator::EnvRef,
    expr::{Arity, Expr, Exprs, ProcedureResult, proc_result_value},
    expr::list::List
};

define_procedures! {
    eqv = ("eqv?", eqv_fn, Arity::Exact(2)),
    eq = ("eq?", eq_fn, Arity::Exact(2)),
    equal = ("equal?", equal_fn, Arity::Exact(2)),
}

fn eqv_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let arg1 = args.pop_front().unwrap();
    let arg2 = args.pop_front().unwrap();

    let result = match (arg1, arg2) {
        (Expr::Boolean(a), Expr::Boolean(b)) => Expr::Boolean(a == b),
        (Expr::Integer(a), Expr::Integer(b)) => Expr::Boolean(a == b),
        (Expr::Float(a), Expr::Float(b)) => Expr::Boolean(a == b),
        (Expr::Char(a), Expr::Char(b)) => Expr::Boolean(a == b),
        (Expr::String(a), Expr::String(b)) => Expr::Boolean(Rc::ptr_eq(&a, &b)),
        (Expr::Symbol(a), Expr::Symbol(b)) => Expr::Boolean(a == b),
        (Expr::List(a), Expr::List(b)) => Expr::Boolean(std::ptr::eq(&a, &b)),
        (Expr::Procedure(a), Expr::Procedure(b)) => Expr::Boolean(a == b),
        _ => Expr::Boolean(false),
    };

    proc_result_value!(result)
}

fn eq_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let arg1 = args.pop_front().unwrap();
    let arg2 = args.pop_front().unwrap();

    let result = match (arg1, arg2) {
        (Expr::Boolean(a), Expr::Boolean(b)) => Expr::Boolean(a == b),
        (Expr::Integer(a), Expr::Integer(b)) => Expr::Boolean(a == b),
        (Expr::Float(a), Expr::Float(b)) => Expr::Boolean(a == b),
        (Expr::Char(a), Expr::Char(b)) => Expr::Boolean(a == b),
        (Expr::String(a), Expr::String(b)) => Expr::Boolean(Rc::ptr_eq(&a, &b)),
        (Expr::Symbol(a), Expr::Symbol(b)) => Expr::Boolean(a == b),
        (Expr::List(a), Expr::List(b)) => Expr::Boolean(std::ptr::eq(&a, &b)),
        (Expr::Procedure(a), Expr::Procedure(b)) => Expr::Boolean(a == b),
        _ => Expr::Boolean(false),
    };

    proc_result_value!(result)
}

fn equal_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let arg1 = args.pop_front().unwrap();
    let arg2 = args.pop_front().unwrap();

    let result = match (arg1, arg2) {
        (Expr::Boolean(a), Expr::Boolean(b)) => Expr::Boolean(a == b),
        (Expr::Integer(a), Expr::Integer(b)) => Expr::Boolean(a == b),
        (Expr::Float(a), Expr::Float(b)) => Expr::Boolean(a == b),
        (Expr::Char(a), Expr::Char(b)) => Expr::Boolean(a == b),
        (Expr::String(a), Expr::String(b)) => Expr::Boolean(a.borrow().as_str() == b.borrow().as_str()),
        (Expr::Symbol(a), Expr::Symbol(b)) => Expr::Boolean(a == b),
        (Expr::List(a), Expr::List(b)) => Expr::Boolean(equal_lists(&a, &b)),
        (Expr::Procedure(a), Expr::Procedure(b)) => Expr::Boolean(a == b),
        _ => Expr::Boolean(false),
    };

    proc_result_value!(result)
}

fn equal_lists(list1: &List, list2: &List) -> bool {
    if list1.len() != list2.len() {
        return false;
    }

    let mut iter1 = list1.iter();
    let mut iter2 = list2.iter();

    while let (Some(elem1), Some(elem2)) = (iter1.next(), iter2.next()) {
        if !equal_recursive(elem1, elem2) {
            return false;
        }
    }

    true
}

fn equal_recursive(expr1: &Expr, expr2: &Expr) -> bool {
    match (expr1, expr2) {
        (Expr::Boolean(a), Expr::Boolean(b)) => a == b,
        (Expr::Integer(a), Expr::Integer(b)) => a == b,
        (Expr::Float(a), Expr::Float(b)) => a == b,
        (Expr::Char(a), Expr::Char(b)) => a == b,
        (Expr::String(a), Expr::String(b)) => a.borrow().as_str() == b.borrow().as_str(),
        (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
        (Expr::List(a), Expr::List(b)) => equal_lists(a, b),
        (Expr::Procedure(a), Expr::Procedure(b)) => a == b,
        _ => false,
    }
}
