use super::{env::EnvRef, eval};
use crate::{
    evaluator::utils::CheckArity,
    expr::{
        Arity, AtomicProcedure, CompoundProcedure, Expr, Exprs, NamedProcedure, Procedure,
        ProcedureParams, ProcedureResult,
    },
    utils::debug,
};
use std::ops::Deref;

pub trait ApplyProcedure {
    fn apply(&self, args: Exprs, env: &mut EnvRef) -> ProcedureResult;
}

impl ApplyProcedure for Procedure {
    fn apply(&self, args: Exprs, env: &mut EnvRef) -> ProcedureResult {
        debug!("applying {} to args {:?}", self, args);
        match self {
            Procedure::Atomic(proc) => proc.apply(args, env),
            Procedure::Compound(proc) => proc.apply(args, env),
        }
    }
}

impl ApplyProcedure for AtomicProcedure {
    fn apply(&self, args: Exprs, env: &mut EnvRef) -> ProcedureResult {
        args.validate_arity(self.name(), self.arity())?;
        (self.proc())(args, env)
    }
}

impl ApplyProcedure for CompoundProcedure {
    fn apply(&self, mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
        let mut eval_env = self.env.clone().extend();

        match self.params.clone() {
            ProcedureParams::Fixed(params) => {
                // validate that the number of arguments matches the number of parameters
                args.exact_or_expected_arity(
                    params.len(),
                    self.name(),
                    Arity::Exact(params.len()),
                )?;

                for (param, arg) in params.into_iter().zip(args) {
                    eval_env.add(param, arg);
                }
            }
            ProcedureParams::Variadic(param) => {
                eval_env.add(param, Expr::new_proper_list(args));
            }
            ProcedureParams::Mixed(params, variadic) => {
                // validate that the number of arguments is at least the number of required parameters
                args.at_least_or_expected_arity(
                    params.len(),
                    self.name(),
                    Arity::AtLeast(params.len()),
                )?;

                // split args into fixed arguments and variadic arguments
                let rest_args = args.split_off(params.len());
                let fixed_args = args;
                for (param, arg) in params.into_iter().zip(fixed_args) {
                    eval_env.add(param, arg);
                }
                eval_env.add(variadic, Expr::new_proper_list(rest_args));
            }
        }

        let body = self.body.deref().as_exprs().clone();
        eval::eval_exprs_with_tailcall(body, &mut eval_env)
    }
}
