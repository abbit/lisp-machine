use crate::{
    evaluator::{error::runtime_error, EnvRef, EvalError, EvalResult},
    expr::{Body, Expr, Exprs, ListKind, Procedure, ProcedureParams},
};

pub fn fold_binary_op(
    first_arg: Expr,
    rest_args: Exprs,
    env: &mut EnvRef,
    op: fn((Expr, Expr), &mut EnvRef) -> EvalResult,
) -> EvalResult {
    // fold over the rest of the arguments, adding them to the first argument
    rest_args
        .into_iter()
        .try_fold(first_arg, |acc, arg| op((acc, arg), env))
}

macro_rules! define_special_forms {
    {$($var:ident = ($name:expr,$proc:expr,$arity:expr),)*} => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $var: (&str, $crate::expr::ProcedureKind, $crate::expr::ProcedureFn, $crate::expr::Arity) =
                ($name, $crate::expr::ProcedureKind::SpecialForm, $proc, $arity);
        )*
    };
}
pub(super) use define_special_forms;

macro_rules! define_procedures {
    {$($var:ident = ($name:expr,$proc:expr,$arity:expr),)*} => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $var: (&str, $crate::expr::ProcedureKind, $crate::expr::ProcedureFn, $crate::expr::Arity) =
                ($name, $crate::expr::ProcedureKind::Procedure, $proc, $arity);
        )*
    };
}

pub(super) use define_procedures;

pub fn create_procedure(
    name: Option<String>,
    params: Expr,
    body: Body,
    env: &mut EnvRef,
) -> EvalResult {
    let params = match params {
        Expr::List(list) => {
            let kind = list.kind();
            let mut params = list
                .into_iter()
                .map(|expr| {
                    expr.into_symbol().map_err(|expr| {
                        runtime_error!("expected symbols in lambda params, got {}", expr.kind())
                    })
                })
                .collect::<Result<Vec<String>, EvalError>>()?;

            match kind {
                ListKind::Proper => ProcedureParams::Fixed(params),
                ListKind::Dotted => {
                    // safe to unwrap since improper list must have at least two elements
                    let variadic_param = params.pop().unwrap();
                    ProcedureParams::Mixed(params, variadic_param)
                }
            }
        }
        Expr::Symbol(param) => ProcedureParams::Variadic(param),
        _ => {
            return Err(runtime_error!(
                "expected list, symbol or dotted list as first argument for lambda"
            ))
        }
    };

    let procedure = Expr::Procedure(Procedure::new_compound(name, params, body, env));

    Ok(procedure)
}
