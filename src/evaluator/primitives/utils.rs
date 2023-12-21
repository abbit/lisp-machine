use crate::{
    evaluator::{EnvRef, EvalResult},
    expr::{Expr, Exprs},
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
