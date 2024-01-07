use super::utils::{define_procedures, define_special_forms};
use crate::{
    evaluator::{error::runtime_error, primitives::utils::create_procedure, EnvRef},
    expr::{proc_result_value, Arity, Body, Expr, Exprs, ListKind, ProcedureResult},
};
use uuid::Uuid;

define_special_forms! {
    define_macro = ("define-macro", define_macro_fn, Arity::AtLeast(2)),
}

define_procedures! {
    gensym = ("gensym", gensym_fn, Arity::Exact(0)),
}

fn define_macro_fn(mut args: Exprs, env: &mut EnvRef) -> ProcedureResult {
    let name_and_params = args.pop_front().unwrap().into_list().map_err(|expr| {
        runtime_error!(
            "expected list as the first argument for define-macro, got {}",
            expr
        )
    })?;

    // check that all elements define-macro formals list are symbols
    name_and_params
        .iter()
        .zip(1..)
        .try_for_each(|expr| match expr {
            (Expr::Symbol(_), _) => Ok(()),
            (expr, idx) => Err(runtime_error!(
                "expected symbols in define procedure formals list, got {} at position {}",
                expr.kind(),
                idx
            )),
        })?;

    let (name_expr, mut params) = name_and_params.split_first().map_err(|_| {
        runtime_error!("expected at least 1 argument for define-macro formals list, got 0")
    })?;
    // unwrap is safe since we checked `name_and_params` above
    let name = name_expr.into_symbol().unwrap();

    // define a procedure with variadic params
    let params_expr = match (params.kind(), params.len()) {
        (ListKind::Dotted, 1) => {
            // safe to unwrap since params.len() == 1
            params.pop_front().unwrap()
        }
        _ => Expr::List(params),
    };

    let body: Body = args.into();
    let procedure = create_procedure(Some(name.to_string()), params_expr, body, env)?;
    env.add_macro(name.to_string(), procedure.into_procedure().unwrap());

    proc_result_value!(Expr::Void)
}

fn gensym_fn(_args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let name = format!("gensym-{}", Uuid::new_v4());
    proc_result_value!(Expr::Symbol(name))
}
