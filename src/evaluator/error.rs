#[derive(Debug, PartialEq)]
pub enum EvalError {
    RuntimeError(String),
}

impl std::error::Error for EvalError {}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EvalError::RuntimeError(err) => write!(f, "runtime error: {}", err),
        }
    }
}

macro_rules! runtime_error {
    ($($arg:tt)*) => (
        crate::evaluator::error::EvalError::RuntimeError(format!($($arg)*))
    )
}
pub(super) use runtime_error;
