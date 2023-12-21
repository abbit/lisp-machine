use super::utils::{define_procedures, fold_binary_op};
use crate::{
    evaluator::{error::runtime_error, EnvRef, EvalResult},
    expr::{Arity, Expr, Exprs},
};

define_procedures! {
    add = ("+", add_fn, Arity::Any),
    mult = ("*", mult_fn, Arity::Any),
    sub = ("-", sub_fn, Arity::AtLeast(1)),
    divide = ("/", divide_fn, Arity::AtLeast(1)),
    less = ("<", less_fn, Arity::Exact(2)),
    equal = ("=", equal_fn, Arity::Exact(2)),
    more = (">", more_fn, Arity::Exact(2)),
}

fn add_fn(args: Exprs, env: &mut EnvRef) -> EvalResult {
    fold_binary_op(Expr::Integer(0), args, env, |(acc, arg), _| {
        match (acc, arg) {
            (Expr::Integer(lhs), Expr::Integer(rhs)) => Ok(Expr::Integer(lhs + rhs)),
            (Expr::Integer(lhs), Expr::Float(rhs)) => Ok(Expr::Float(lhs as f64 + rhs)),
            (Expr::Float(lhs), Expr::Integer(rhs)) => Ok(Expr::Float(lhs + rhs as f64)),
            (Expr::Float(lhs), Expr::Float(rhs)) => Ok(Expr::Float(lhs + rhs)),
            (lhs, rhs) => Err(runtime_error!(
                "expected integers or floats for +, got {} and {}",
                lhs.kind(),
                rhs.kind(),
            )),
        }
    })
}

fn sub_fn(mut args: Exprs, env: &mut EnvRef) -> EvalResult {
    let first_arg = if args.len() > 1 {
        args.pop_front().unwrap()
    } else {
        Expr::Integer(0)
    };

    fold_binary_op(first_arg, args, env, |(acc, arg), _| match (acc, arg) {
        (Expr::Integer(lhs), Expr::Integer(rhs)) => Ok(Expr::Integer(lhs - rhs)),
        (Expr::Integer(lhs), Expr::Float(rhs)) => Ok(Expr::Float(lhs as f64 - rhs)),
        (Expr::Float(lhs), Expr::Integer(rhs)) => Ok(Expr::Float(lhs - rhs as f64)),
        (Expr::Float(lhs), Expr::Float(rhs)) => Ok(Expr::Float(lhs - rhs)),
        (lhs, rhs) => Err(runtime_error!(
            "expected integers or floats -, got {} and {}",
            lhs,
            rhs
        )),
    })
}

fn mult_fn(args: Exprs, env: &mut EnvRef) -> EvalResult {
    fold_binary_op(Expr::Integer(1), args, env, |(acc, arg), _| {
        match (acc, arg) {
            (Expr::Integer(lhs), Expr::Integer(rhs)) => Ok(Expr::Integer(lhs * rhs)),
            (Expr::Integer(lhs), Expr::Float(rhs)) => Ok(Expr::Float(lhs as f64 * rhs)),
            (Expr::Float(lhs), Expr::Integer(rhs)) => Ok(Expr::Float(lhs * rhs as f64)),
            (Expr::Float(lhs), Expr::Float(rhs)) => Ok(Expr::Float(lhs * rhs)),
            (lhs, rhs) => Err(runtime_error!(
                "expected integers or floats for *, got {} and {}",
                lhs.kind(),
                rhs.kind()
            )),
        }
    })
}

fn divide_fn(mut args: Exprs, env: &mut EnvRef) -> EvalResult {
    let first_arg = if args.len() > 1 {
        args.pop_front().unwrap()
    } else {
        Expr::Integer(1)
    };

    fold_binary_op(first_arg, args, env, |(acc, arg), _| match (acc, arg) {
        (Expr::Integer(lhs), Expr::Integer(rhs)) => Ok(Expr::Integer(lhs / rhs)),
        (Expr::Integer(lhs), Expr::Float(rhs)) => Ok(Expr::Float(lhs as f64 / rhs)),
        (Expr::Float(lhs), Expr::Integer(rhs)) => Ok(Expr::Float(lhs / rhs as f64)),
        (Expr::Float(lhs), Expr::Float(rhs)) => Ok(Expr::Float(lhs / rhs)),
        (lhs, rhs) => Err(runtime_error!(
            "expected integers or floats for /, got {} and {}",
            lhs.kind(),
            rhs.kind()
        )),
    })
}

fn less_fn(mut args: Exprs, _: &mut EnvRef) -> EvalResult {
    let lhs = args.pop_front().unwrap();
    let rhs = args.pop_front().unwrap();

    match (lhs, rhs) {
        (Expr::Integer(lhs), Expr::Integer(rhs)) => Ok(Expr::Boolean(lhs < rhs)),
        (Expr::Integer(lhs), Expr::Float(rhs)) => Ok(Expr::Boolean((lhs as f64) < rhs)),
        (Expr::Float(lhs), Expr::Integer(rhs)) => Ok(Expr::Boolean(lhs < rhs as f64)),
        (Expr::Float(lhs), Expr::Float(rhs)) => Ok(Expr::Boolean(lhs < rhs)),
        (lhs, rhs) => Err(runtime_error!(
            "expected integers or floats for <, got {} and {}",
            lhs.kind(),
            rhs.kind()
        )),
    }
}

fn equal_fn(mut args: Exprs, _: &mut EnvRef) -> EvalResult {
    let lhs = args.pop_front().unwrap();
    let rhs = args.pop_front().unwrap();

    match (lhs, rhs) {
        (Expr::Integer(lhs), Expr::Integer(rhs)) => Ok(Expr::Boolean(lhs == rhs)),
        (Expr::Integer(lhs), Expr::Float(rhs)) => Ok(Expr::Boolean((lhs as f64) == rhs)),
        (Expr::Float(lhs), Expr::Integer(rhs)) => Ok(Expr::Boolean(lhs == rhs as f64)),
        (Expr::Float(lhs), Expr::Float(rhs)) => Ok(Expr::Boolean(lhs == rhs)),
        (lhs, rhs) => Err(runtime_error!(
            "expected integers or floats for =, got {} and {}",
            lhs.kind(),
            rhs.kind()
        )),
    }
}

fn more_fn(mut args: Exprs, _: &mut EnvRef) -> EvalResult {
    let lhs = args.pop_front().unwrap();
    let rhs = args.pop_front().unwrap();

    match (lhs, rhs) {
        (Expr::Integer(lhs), Expr::Integer(rhs)) => Ok(Expr::Boolean(lhs > rhs)),
        (Expr::Integer(lhs), Expr::Float(rhs)) => Ok(Expr::Boolean((lhs as f64) > rhs)),
        (Expr::Float(lhs), Expr::Integer(rhs)) => Ok(Expr::Boolean(lhs > rhs as f64)),
        (Expr::Float(rhs), Expr::Float(lhs)) => Ok(Expr::Boolean(rhs > lhs)),
        (lhs, rhs) => Err(runtime_error!(
            "expected integers or floats for >, got {} and {}",
            lhs.kind(),
            rhs.kind()
        )),
    }
}
