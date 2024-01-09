//! LispDM is a Scheme interpreter. Currently, it supports only a subset of Scheme.
//! ```
//! use lispdm::Engine;
//! let mut engine = Engine::default();
//!
//! let result = engine.eval::<i64>("(+ 1 2)").unwrap();
//! assert_eq!(result, 3);
//! ```
mod evaluator;
mod expr;
mod parser;
mod utils;

pub use evaluator::EnvRef;
use evaluator::EvalError;
use expr::FromExpr;
pub use expr::{Expr, Exprs};
use parser::ParseError;

const PRELUDE: &str = include_str!("./prelude.scm");

#[derive(Debug)]
/// Error type for LispDM.
pub enum LispDMError {
    /// Error occurred while parsing source code.
    ParseError(ParseError),
    /// Error occurred while evaluating expressions.
    EvalError(EvalError),
    /// Error occurred while converting an expression to a Rust type.
    ConvertError(String),
}

impl std::error::Error for LispDMError {}

impl std::fmt::Display for LispDMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "parse error: {}", err),
            Self::EvalError(err) => write!(f, "eval error: {}", err),
            Self::ConvertError(err) => write!(f, "convert error: {}", err),
        }
    }
}

macro_rules! convert_error {
    ($($arg:tt)*) => (
        crate::LispDMError::ConvertError(format!($($arg)*))
    )
}

impl From<ParseError> for LispDMError {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<EvalError> for LispDMError {
    fn from(err: EvalError) -> Self {
        Self::EvalError(err)
    }
}

/// Provides the main functionality of LispDM.
/// It holds the environment and provides eval method.
pub struct Engine {
    root_env: EnvRef,
}

impl Engine {
    fn load_prelude(&mut self) {
        self.eval::<()>(PRELUDE).expect("failed to load prelude!");
    }

    /// Evaluates the given source code and returns the result.
    /// The result can be converted to any type that implements FromExpr.
    /// If you want to get an Expr, use `eval::<Expr>(src)`.
    pub fn eval<R: FromExpr>(&mut self, src: &str) -> Result<R, LispDMError> {
        let ast = parser::parse_str(src).map_err(LispDMError::ParseError)?;
        evaluator::eval_exprs(ast, &mut self.root_env)
            .map_err(LispDMError::EvalError)
            .and_then(|expr| {
                R::from_expr(expr).map_err(|expr| convert_error!("expected {:?}", expr))
            })
    }

    /// Returns reference to the root environment.
    pub fn env(&self) -> EnvRef {
        self.root_env.clone()
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
