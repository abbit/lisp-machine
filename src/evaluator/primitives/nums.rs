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
    sqrt = ("sqrt", sqrt_fn, Arity::Exact(1)),
    square = ("square", square_fn, Arity::Exact(1)),
    expt = ("expt", expt_fn, Arity::Exact(2)),
    min = ("min", min_fn, Arity::AtLeast(1)),
    max = ("max", max_fn, Arity::AtLeast(1)),
    positive = ("positive?", positive_fn, Arity::Exact(1)),
    negative = ("negative?", negative_fn, Arity::Exact(1)),
    floor = ("floor", floor_fn, Arity::Exact(1)),
    ceiling = ("ceiling", ceiling_fn, Arity::Exact(1)),
    truncate = ("truncate", truncate_fn, Arity::Exact(1)),
    round = ("round", round_fn, Arity::Exact(1)),
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
    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Boolean(n % 2 != 0)),
        _ => Err(runtime_error!(
            "expected integer for odd?, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

fn sqrt_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    (match &args[0] {
        Expr::Integer(n) if *n >= 0 => Ok(Expr::Float(((*n) as f64).sqrt())),
        Expr::Float(f) if *f >= 0.0 => Ok(Expr::Float(f.sqrt())),
        _ => Err(runtime_error!(
            "expected non-negative integer or float for sqrt, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

fn square_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    match &args[0] {
        Expr::Integer(n) => Ok(Expr::Integer(n * n)),
        Expr::Float(f) => Ok(Expr::Float(f * f)),
        _ => Err(runtime_error!(
            "expected integer or float for square, got {}",
            args[0].kind()
        )),
    }
    .map(ProcedureReturn::Value)
}

fn expt_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let base = &args[0];
    let exponent = &args[1];

    match (base, exponent) {
        (Expr::Integer(base), Expr::Integer(exp)) => {
            Ok(Expr::Float(((*base) as f64).powi(*exp as i32)))
        }
        (Expr::Integer(base), Expr::Float(exp)) => Ok(Expr::Float(((*base) as f64).powf(*exp))),
        (Expr::Float(base), Expr::Integer(exp)) => Ok(Expr::Float(base.powi(*exp as i32))),
        (Expr::Float(base), Expr::Float(exp)) => Ok(Expr::Float(base.powf(*exp))),
        _ => Err(runtime_error!(
            "expected integers or floats for expt, got {} and {}",
            base.kind(),
            exponent.kind()
        )),
    }
    .map(ProcedureReturn::Value)
}

fn min_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let min_value = args
        .into_iter()
        .try_fold(std::f64::INFINITY, |acc, arg| match arg {
            Expr::Integer(n) => Ok(acc.min(n as f64)),
            Expr::Float(f) => Ok(acc.min(f)),
            _ => Err(runtime_error!(
                "expected integers or floats for min, got {}",
                arg.kind()
            )),
        })?;

    Ok(Expr::Float(min_value)).map(ProcedureReturn::Value)
}

fn max_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let max_value = args
        .into_iter()
        .try_fold(std::f64::NEG_INFINITY, |acc, arg| match arg {
            Expr::Integer(n) => Ok(acc.max(n as f64)),
            Expr::Float(f) => Ok(acc.max(f)),
            _ => Err(runtime_error!(
                "expected integers or floats for max, got {}",
                arg.kind()
            )),
        })?;

    Ok(Expr::Float(max_value)).map(ProcedureReturn::Value)
}

fn positive_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Boolean(*n > 0)),
        Expr::Float(f) => Ok(Expr::Boolean(*f > 0.0)),
        _ => Err(runtime_error!(
            "expected integer or float for positive?, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

fn negative_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Boolean(*n < 0)),
        Expr::Float(f) => Ok(Expr::Boolean(*f < 0.0)),
        _ => Err(runtime_error!(
            "expected integer or float for negative?, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

fn floor_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Float(*n as f64)),
        Expr::Float(f) => Ok(Expr::Float(f.floor())),
        _ => Err(runtime_error!(
            "expected integer or float for floor, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

fn ceiling_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Float(*n as f64)),
        Expr::Float(f) => Ok(Expr::Float(f.ceil())),
        _ => Err(runtime_error!(
            "expected integer or float for ceiling, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

fn truncate_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Float(*n as f64)),
        Expr::Float(f) => Ok(Expr::Float(f.trunc())),
        _ => Err(runtime_error!(
            "expected integer or float for truncate, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}

fn round_fn(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    (match &args[0] {
        Expr::Integer(n) => Ok(Expr::Float(*n as f64)),
        Expr::Float(f) => Ok(Expr::Float(f.round())),
        _ => Err(runtime_error!(
            "expected integer or float for round, got {}",
            args[0].kind()
        )),
    })
    .map(ProcedureReturn::Value)
}