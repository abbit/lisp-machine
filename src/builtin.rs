use crate::{
    ast::{Expr, Procedure, ProcedureData},
    debug,
    environment::EnvRef,
    interpreter::{eval_expr, EvalError, EvalResult},
};

// TODO: test set! and define

pub enum ModifyEnv {
    Add,
    Set,
}

fn modify_env(args: &Vec<Expr>, env: &mut EnvRef, mod_env: ModifyEnv) -> EvalResult {
    if args.len() != 2 {
        return Err(EvalError::RuntimeError(format!(
            "expected 2 arguments for define, got {}",
            args.len()
        )));
    }

    let symbol = match args.get(0) {
        Some(Expr::Symbol(symbol)) => symbol,
        _ => {
            return Err(EvalError::RuntimeError(format!(
                "expected symbol in define, got {:?}",
                args.get(1)
            )))
        }
    };
    let body = eval_expr(args.get(1).unwrap(), env)?;

    let mod_env_result = match mod_env {
        ModifyEnv::Add => env.add(symbol, body),
        ModifyEnv::Set => env.set(symbol, body),
    };

    match mod_env_result {
        Ok(_) => Ok(Expr::Void),
        Err(err) => Err(EvalError::RuntimeError(format!("{}", err))),
    }
}

fn define_procedure(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    debug!("defining procedure: {:?}", args);

    let name_and_params = match args.get(0) {
        Some(Expr::List(list)) => list,
        _ => {
            return Err(EvalError::RuntimeError(
                "expected list as first argument for define procedure".to_string(),
            ))
        }
    };

    let name = match name_and_params.get(0) {
        Some(Expr::Symbol(symbol)) => symbol,
        _ => {
            return Err(EvalError::RuntimeError(
                "expected symbol as first argument for define procedure".to_string(),
            ))
        }
    };

    let params = name_and_params
        .iter()
        .skip(1)
        .map(|expr| match expr {
            Expr::Symbol(symbol) => Ok(symbol.clone()),
            _ => Err(EvalError::RuntimeError(
                "expected symbol in lambda params".to_string(),
            )),
        })
        .collect::<Result<Vec<String>, EvalError>>()?;

    let body = match args.get(1) {
        Some(Expr::List(list)) => Expr::List(list.clone()),
        _ => {
            return Err(EvalError::RuntimeError(
                "expected list for body in lambda".to_string(),
            ))
        }
    };

    let procedure = Expr::Procedure(ProcedureData::new_compound(
        Some(name.to_string()),
        params,
        body,
        env,
    ));
    debug!("created procedure: {}", procedure);

    match env.add(name, procedure) {
        Ok(_) => Ok(Expr::Void),
        Err(err) => Err(EvalError::RuntimeError(format!("{}", err))),
    }
}

pub fn define(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    match args.get(0) {
        // define a variable
        Some(Expr::Symbol(_)) => modify_env(args, env, ModifyEnv::Add),
        // define a procedure
        Some(Expr::List(_)) => define_procedure(args, env),
        _ => Err(EvalError::RuntimeError(
            "expected symbol or list as first argument for define".to_string(),
        )),
    }
}

pub fn set(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    modify_env(args, env, ModifyEnv::Set)
}

pub fn lambda(list: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    let params = match list.get(0) {
        Some(Expr::List(params)) => params
            .iter()
            .map(|expr| match expr {
                Expr::Symbol(symbol) => Ok(symbol.clone()),
                _ => Err(EvalError::RuntimeError(
                    "expected symbol in lambda params".to_string(),
                )),
            })
            .collect::<Result<Vec<String>, EvalError>>()?,
        _ => {
            return Err(EvalError::RuntimeError(
                "expected list for params in lambda".to_string(),
            ))
        }
    };

    let body = match list.get(1) {
        Some(Expr::List(list)) => Expr::List(list.clone()),
        _ => {
            return Err(EvalError::RuntimeError(
                "expected list for body in lambda".to_string(),
            ))
        }
    };

    let procedure = Expr::Procedure(ProcedureData::new_compound(None, params, body, env));
    debug!("created procedure: {}", procedure);

    Ok(procedure)
}

pub fn begin(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    args.iter()
        .try_fold(Expr::Void, |_, expr| eval_expr(expr, env))
}

pub fn list(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    let list = args
        .iter()
        .map(|expr| eval_expr(expr, env))
        .collect::<Result<Vec<Expr>, EvalError>>()?;

    Ok(Expr::List(list))
}

pub fn apply(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    let operator = args.get(0).ok_or(EvalError::RuntimeError(
        "expected procedure as first argument for apply".to_string(),
    ))?;
    let operands = match args.get(1) {
        Some(Expr::List(list)) => list,
        _ => {
            return Err(EvalError::RuntimeError(
                "expected list as second argument for apply".to_string(),
            ))
        }
    };

    let operator = eval_expr(operator, env)?;
    let operands = match eval_expr(&Expr::List(operands.clone()), env)? {
        Expr::List(list) => list,
        _ => {
            return Err(EvalError::RuntimeError(
                "expected list as second argument for apply".to_string(),
            ))
        }
    };

    match &operator {
        Expr::Procedure(proc) => proc.apply(&operands, env),
        _ => Err(EvalError::RuntimeError(
            "expected procedure in first argument of apply".to_string(),
        )),
    }
}

fn binary_op(
    args: &Vec<Expr>,
    env: &mut EnvRef,
    op: fn((Expr, Expr), &mut EnvRef) -> EvalResult,
    first_arg_default: Option<&Expr>,
) -> EvalResult {
    let first_arg = match (args.get(0), first_arg_default) {
        (Some(arg), _) => eval_expr(arg, env)?,
        (None, Some(arg)) => eval_expr(arg, env)?,
        _ => {
            return Err(EvalError::RuntimeError(
                "expected at least one argument for binary operation".to_string(),
            ))
        }
    };

    let rest_args = args.iter().skip(1).collect::<Vec<&Expr>>();

    let first_arg = eval_expr(&first_arg, env)?;
    // fold over the rest of the arguments, adding them to the first argument
    rest_args.iter().try_fold(first_arg, |acc, arg| {
        let arg = eval_expr(arg, env)?;
        op((acc, arg), env)
    })
}

pub fn add(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    binary_op(
        args,
        env,
        |(acc, arg), _| match (acc, arg) {
            (Expr::Integer(int1), Expr::Integer(int2)) => Ok(Expr::Integer(int1 + int2)),
            (Expr::Float(float1), Expr::Float(float2)) => Ok(Expr::Float(float1 + float2)),
            (Expr::Symbol(str1), Expr::Symbol(str2)) => {
                Ok(Expr::Symbol(format!("{}{}", str1, str2)))
            }
            (lhs, rhs) => Err(EvalError::RuntimeError(format!(
                "expected integers, floats or strings, got {} and {}",
                lhs, rhs
            ))),
        },
        Some(&Expr::Integer(0)),
    )
}

pub fn sub(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    binary_op(
        args,
        env,
        |(acc, arg), _| match (acc, arg) {
            (Expr::Integer(int1), Expr::Integer(int2)) => Ok(Expr::Integer(int1 - int2)),
            (Expr::Float(float1), Expr::Float(float2)) => Ok(Expr::Float(float1 - float2)),
            (lhs, rhs) => Err(EvalError::RuntimeError(format!(
                "expected integers or floats, got {} and {}",
                lhs, rhs
            ))),
        },
        None,
    )
}

pub fn mult(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    binary_op(
        args,
        env,
        |(acc, arg), _| match (acc, arg) {
            (Expr::Integer(int1), Expr::Integer(int2)) => Ok(Expr::Integer(int1 * int2)),
            (Expr::Float(float1), Expr::Float(float2)) => Ok(Expr::Float(float1 * float2)),
            (lhs, rhs) => Err(EvalError::RuntimeError(format!(
                "expected integers or floats, got {} and {}",
                lhs, rhs
            ))),
        },
        Some(&Expr::Integer(1)),
    )
}

pub fn divide(args: &Vec<Expr>, env: &mut EnvRef) -> EvalResult {
    binary_op(
        args,
        env,
        |(acc, arg), _| match (acc, arg) {
            (Expr::Integer(int1), Expr::Integer(int2)) => Ok(Expr::Integer(int1 / int2)),
            (Expr::Float(float1), Expr::Float(float2)) => Ok(Expr::Float(float1 / float2)),
            (lhs, rhs) => Err(EvalError::RuntimeError(format!(
                "expected integers or floats, got {} and {}",
                lhs, rhs
            ))),
        },
        None,
    )
}

pub fn exit(_: &Vec<Expr>, _: &mut EnvRef) -> EvalResult {
    std::process::exit(0);
}
