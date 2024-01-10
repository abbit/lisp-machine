#![warn(missing_docs)]
//! LispDM is a Scheme interpreter. Currently, it supports only a subset of Scheme.
//! ```
//! use lispdm::Engine;
//! let mut engine = Engine::default();
//!
//! let result = engine.eval::<i64>("(+ 1 2)").unwrap();
//! assert_eq!(result, Ok(3));
//! ```
mod evaluator;
mod expr;
mod parser;
mod utils;

pub use evaluator::EnvRef;
use evaluator::EvalError;
use expr::Procedure;
pub use expr::{
    Arity, Expr, Exprs, FromExpr, FromExprResult, ProcedureFn, ProcedureKind, ProcedureResult,
    ProcedureReturn,
};
use parser::ParseError;

const PRELUDE: &str = include_str!("./prelude.scm");

#[derive(Debug)]
/// Error type that represents all possible errors in LispDM.
pub enum LispDMError {
    /// Error occurred while parsing source code.
    ParseError(String),
    /// Error occurred while evaluating expressions.
    EvalError(String),
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

impl From<ParseError> for LispDMError {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err.to_string())
    }
}

impl From<EvalError> for LispDMError {
    fn from(err: EvalError) -> Self {
        Self::EvalError(err.to_string())
    }
}

/// Provides the main functionality of LispDM.
///
/// Holds the environment and provides methods interacting with it.
pub struct Engine {
    root_env: EnvRef,
}

impl Engine {
    fn load_prelude(&mut self) {
        self.eval::<()>(PRELUDE)
            .expect("failed to load prelude!")
            .unwrap();
    }

    /// Creates a new instance of [`Engine`] without loading the prelude.
    ///
    /// In this case the root environment will not contain some of the special forms and standard procedures.
    ///
    /// # Examples
    /// ```
    /// use lispdm::Engine;
    /// let mut engine = Engine::new_without_prelude();
    /// assert!(!engine.env().has("and"));
    /// ```
    pub fn new_without_prelude() -> Self {
        let root_env = evaluator::new_root_env();
        Self { root_env }
    }

    /// Evaluates the given source code and returns the result.
    /// The result can be converted to any type that implements [`FromExpr`].
    ///
    /// If evaluation succeeds, returns [`FromExprResult<R>`],
    /// where `R` is the type that implements [`FromExpr`].
    ///
    /// If evaluation fails, returns [`LispDMError`], which describes the error.
    ///
    /// # Examples
    /// Get evaluated value as Rust type:
    /// ```
    /// use lispdm::Engine;
    /// let mut engine = Engine::default();
    /// let result = engine.eval::<i64>("(+ 1 2)").unwrap().unwrap();
    /// assert_eq!(result, 3);
    /// ```
    ///
    /// Get evaluated value as [`Expr`]:
    /// ```
    /// use lispdm::{Engine, Expr};
    /// let mut engine = Engine::default();
    /// let result = engine.eval::<Expr>("(+ 1 2)").unwrap().unwrap();
    /// assert_eq!(result, Expr::Integer(3));
    /// ```
    pub fn eval<R: FromExpr>(&mut self, src: &str) -> Result<FromExprResult<R>, LispDMError> {
        let ast = parser::parse_str(src).map_err(LispDMError::from)?;
        evaluator::eval_exprs(ast, &mut self.root_env)
            .map(R::from_expr)
            .map_err(LispDMError::from)
    }

    /// Returns reference to the root environment.
    ///
    /// # Examples
    /// Get a reference to the root environment:
    /// ```
    /// use lispdm::Engine;
    /// let mut engine = Engine::default();
    /// let root_env = engine.env();
    /// ```
    /// Get a value from the root environment:
    /// ```
    /// use lispdm::Engine;
    /// let mut engine = Engine::default();
    /// let result = engine.eval::<()>("(define x 1)").unwrap();
    /// let root_env = engine.env();
    /// let x = root_env.get::<i64>("x").unwrap().unwrap();
    /// assert_eq!(x, 1);
    /// ```
    pub fn env(&self) -> EnvRef {
        self.root_env.clone()
    }

    /// Registers a new procedure in the root environment.
    /// Can be normal procedure or a special form.
    ///
    /// # Examples
    /// Register a normal procedure:
    /// ```
    /// use lispdm::{Engine, EnvRef, Expr, Exprs, ProcedureKind, Arity, ProcedureResult,
    /// ProcedureReturn};
    ///
    /// fn sum3(args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    ///    let mut sum = 0;
    ///    for arg in args {
    ///        sum += arg.into::<i64>().map_err(|_| "expected integer")?;
    ///    }
    ///    Ok(ProcedureReturn::Value(Expr::Integer(sum)))
    /// }
    ///
    /// let mut engine = Engine::default();
    /// engine.register_fn(
    ///    "sum3",
    ///    ProcedureKind::Procedure,
    ///    Arity::Exact(3),
    ///    sum3,
    /// );
    /// assert!(engine.env().has("sum3"));
    ///
    /// let result = engine.eval::<i64>("(sum3 1 2 3)").unwrap().unwrap();
    /// assert_eq!(result, 6);
    ///
    /// // Error: wrong arity
    /// let result = engine.eval::<i64>("(sum3 1 2)");
    /// assert!(result.is_err());
    pub fn register_fn<S: ToString>(
        &mut self,
        name: S,
        kind: ProcedureKind,
        arity: Arity,
        proc: ProcedureFn,
    ) {
        self.root_env.add(
            name.to_string(),
            Procedure::new_atomic(name.to_string(), kind, proc, arity),
        );
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
