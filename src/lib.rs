mod evaluator;
mod expr;
mod parser;
mod utils;

use std::collections::HashMap;

use evaluator::{EnvRef, EvalError};
use parser::ParseError;

#[derive(Debug)]
pub enum LispDMError {
    ParseError(ParseError),
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

pub struct Engine {
    root_env: EnvRef,
    macro_table: HashMap<String, expr::Body>,
}

impl Engine {
    pub fn eval(&mut self, src: &str) -> Result<expr::Expr, LispDMError> {
        let ast = parser::parse_str(src).map_err(LispDMError::ParseError)?;
        evaluator::eval_exprs(ast.into_iter(), &mut self.root_env).map_err(LispDMError::EvalError)
    }

    pub fn add_macro(&mut self, name: String, body: expr::Body) {
        self.macro_table.insert(name, body);
    }
}

impl Default for Engine {
    fn default() -> Self {
        let root_env = evaluator::new_root_env();
        let macro_table = HashMap::new();
        Self { root_env, macro_table }
    }
}
