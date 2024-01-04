use super::utils::{define_procedures, fold_binary_op};
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{proc_result_value, Arity, Expr, Exprs, ProcedureResult, ProcedureReturn},
};

define_procedures! {
    add = ("+", add_fn, Arity::Any),
    mult = ("*", mult_fn, Arity::Any),
    sub = ("-", sub_fn, Arity::AtLeast(1)),
    divide = ("/", divide_fn, Arity::AtLeast(1)),
    less = ("<", less_fn, Arity::Exact(2)),
    equal = ("=", equal_fn, Arity::Exact(2)),
    more = (">", more_fn, Arity::Exact(2)),
    abs = ("abs", abs_fn, Arity::Exact(1)),
    even = ("even?", even_fn, Arity::Exact(1)),
    odd = ("odd?", odd_fn, Arity::Exact(1)),
}

fn add_fn(args: Exprs, env: &mut EnvRef) -> ProcedureResult {
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
    .map(ProcedureReturn::Value)
}

fn sub_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
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
    .map(ProcedureReturn::Value)
}

fn mult_fn(args: Exprs, env: &mut EnvRef) -> ProcedureResult {
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
    .map(ProcedureReturn::Value)
}

fn divide_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
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
    .map(ProcedureReturn::Value)
}

fn less_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let lhs = args.pop_front().unwrap();
    let rhs = args.pop_front().unwrap();

    let res = match (lhs, rhs) {
        (Expr::Integer(lhs), Expr::Integer(rhs)) => Expr::Boolean(lhs < rhs),
        (Expr::Integer(lhs), Expr::Float(rhs)) => Expr::Boolean((lhs as f64) < rhs),
        (Expr::Float(lhs), Expr::Integer(rhs)) => Expr::Boolean(lhs < rhs as f64),
        (Expr::Float(lhs), Expr::Float(rhs)) => Expr::Boolean(lhs < rhs),
        (lhs, rhs) => {
            return Err(runtime_error!(
                "expected integers or floats for <, got {} and {}",
                lhs.kind(),
                rhs.kind()
            ))
        }
    };

    proc_result_value!(res)
}

fn equal_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let lhs = args.pop_front().unwrap();
    let rhs = args.pop_front().unwrap();

    let res = match (lhs, rhs) {
        (Expr::Integer(lhs), Expr::Integer(rhs)) => Expr::Boolean(lhs == rhs),
        (Expr::Integer(lhs), Expr::Float(rhs)) => Expr::Boolean((lhs as f64) == rhs),
        (Expr::Float(lhs), Expr::Integer(rhs)) => Expr::Boolean(lhs == rhs as f64),
        (Expr::Float(lhs), Expr::Float(rhs)) => Expr::Boolean(lhs == rhs),
        (lhs, rhs) => {
            return Err(runtime_error!(
                "expected integers or floats for =, got {} and {}",
                lhs.kind(),
                rhs.kind()
            ))
        }
    };

    proc_result_value!(res)
}

fn more_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let lhs = args.pop_front().unwrap();
    let rhs = args.pop_front().unwrap();

    let res = match (lhs, rhs) {
        (Expr::Integer(lhs), Expr::Integer(rhs)) => Expr::Boolean(lhs > rhs),
        (Expr::Integer(lhs), Expr::Float(rhs)) => Expr::Boolean((lhs as f64) > rhs),
        (Expr::Float(lhs), Expr::Integer(rhs)) => Expr::Boolean(lhs > rhs as f64),
        (Expr::Float(rhs), Expr::Float(lhs)) => Expr::Boolean(rhs > lhs),
        (lhs, rhs) => {
            return Err(runtime_error!(
                "expected integers or floats for >, got {} and {}",
                lhs.kind(),
                rhs.kind()
            ))
        }
    };

    proc_result_value!(res)
}

fn abs_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    if args.len() != 1 {
        return Err(runtime_error!(
            "expected 1 argument for abs, got {}",
            args.len()
        ));
    }

    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Integer(n.abs())),
        Expr::Float(f) => Ok(Expr::Float(f.abs())),
        _ => Err(runtime_error!(
            "expected integer or float for abs, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

fn even_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    if args.len() != 1 {
        return Err(runtime_error!(
            "expected 1 argument for even?, got {}",
            args.len()
        ));
    }

    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Boolean(n % 2 == 0)),
        _ => Err(runtime_error!(
            "expected integer for even?, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

fn odd_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    if args.len() != 1 {
        return Err(runtime_error!(
            "expected 1 argument for odd?, got {}",
            args.len()
        ));
    }

    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Boolean(n % 2 != 0)),
        _ => Err(runtime_error!(
            "expected integer for odd?, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

