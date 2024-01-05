//! LispDM is a Scheme interpreter. Currently, it supports only a subset of Scheme.
//! ```
//! use lispdm::Engine;
//! let mut engine = Engine::default();
//!
//! let result = engine.eval("(+ 1 2)").unwrap();
//! assert_eq!(result, lispdm::Expr::Integer(3));
//! ```
mod evaluator;
mod expr;
mod parser;
mod utils;

use evaluator::{EnvRef, EvalError};
pub use expr::Expr;
use parser::ParseError;

const PRELUDE: &str = include_str!("./prelude.scm");

#[derive(Debug)]
/// Error type for LispDM.
pub enum LispDMError {
    /// Error occurred while parsing source code.
    ParseError(ParseError),
    /// Error occurred while evaluating expressions.
    EvalError(EvalError),
}

impl std::error::Error for LispDMError {}

impl std::fmt::Display for LispDMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "parse error: {}", err),
            Self::EvalError(err) => write!(f, "eval error: {}", err),
        }
    }
}

/// Engine provides the main functionality of LispDM.
/// It holds the environment and provides eval method.
pub struct Engine {
    root_env: EnvRef,
}

impl Engine {
    fn load_prelude(&mut self) {
        self.eval(PRELUDE).expect("failed to load prelude!");
    }

    /// Evaluates the given source code and returns the result.
    pub fn eval(&mut self, src: &str) -> Result<expr::Expr, LispDMError> {
        let ast = parser::parse_str(src).map_err(LispDMError::ParseError)?;
        evaluator::eval_exprs(ast.into_iter(), &mut self.root_env).map_err(LispDMError::EvalError)
    }
}

impl Default for Engine {
    fn default() -> Self {
        let root_env = evaluator::new_root_env();
        let mut engine = Self { root_env };
        engine.load_prelude();

        engine
    }
}