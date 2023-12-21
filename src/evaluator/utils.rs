use std::usize;

use super::error::{runtime_error, EvalError};
use crate::expr::{Arity, Expr, Exprs};

pub trait CheckArity<T> {
    // pop the first argument or return an expected arity error
    fn pop_front_or_expected_arity(&mut self, name: &str, arity: Arity) -> Result<T, EvalError>;

    // check that there are no more arguments
    // if there are, return an expected arity error
    fn empty_or_expected_arity(&self, name: &str, arity: Arity) -> Result<(), EvalError>;

    // check that there are at least `min` arguments
    // returns the length of the arguments
    // if there are less, return an expected arity error
    fn at_least_or_expected_arity(
        &self,
        min: usize,
        name: &str,
        arity: Arity,
    ) -> Result<usize, EvalError>;

    // check that there are exactly `count` arguments
    // returns the length of the arguments
    // if there are less or more, return an expected arity error
    fn exact_or_expected_arity(
        &self,
        count: usize,
        name: &str,
        arity: Arity,
    ) -> Result<(), EvalError> {
        match self.at_least_or_expected_arity(count, name, arity)? {
            len if len == count => Ok(()),
            _ => Err(runtime_error!("expected {} arguments for {}", arity, name)),
        }
    }

    fn validate_arity(&self, name: &str, arity: Arity) -> Result<(), EvalError> {
        match arity {
            Arity::Any => Ok(()),
            Arity::Exact(count) => self.exact_or_expected_arity(count, name, arity),
            Arity::AtLeast(min) => self
                .at_least_or_expected_arity(min, name, arity)
                .map(|_| ()),
        }
    }
}

impl CheckArity<Expr> for Exprs {
    fn pop_front_or_expected_arity(&mut self, name: &str, arity: Arity) -> Result<Expr, EvalError> {
        self.pop_front()
            .ok_or(runtime_error!("expected {} arguments for {}", arity, name))
    }

    fn empty_or_expected_arity(&self, name: &str, arity: Arity) -> Result<(), EvalError> {
        match self.is_empty() {
            true => Ok(()),
            false => Err(runtime_error!("expected {} arguments for {}", arity, name)),
        }
    }

    fn at_least_or_expected_arity(
        &self,
        min: usize,
        name: &str,
        arity: Arity,
    ) -> Result<usize, EvalError> {
        if self.len() < min {
            Err(runtime_error!("expected {} arguments for {}", arity, name,))
        } else {
            Ok(self.len())
        }
    }
}
