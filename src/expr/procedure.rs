use super::expr::{Expr, Exprs};
use crate::{
    evaluator::{EnvRef, EvalError},
    exprs,
    utils::debug,
};

pub trait NamedProcedure {
    fn name_stored(&self) -> Option<&str>;

    fn name(&self) -> &str {
        self.name_stored().unwrap_or("anon")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Procedure {
    Atomic(AtomicProcedure),
    Compound(CompoundProcedure),
}

/// The return value of a procedure.
/// Used to determine whether a tail call should be performed.
pub enum ProcedureReturn {
    /// Return [`Expr`].
    Value(Expr),
    /// Perform a tail call of [`Expr`] in [`EnvRef`].
    TailCall(Expr, EnvRef),
}

/// The result of a procedure call.
pub type ProcedureResult = Result<ProcedureReturn, EvalError>;

macro_rules! proc_result_value {
    ($expr:expr) => {
        Ok($crate::expr::ProcedureReturn::Value($expr))
    };
}
pub(crate) use proc_result_value;

macro_rules! proc_result_tailcall {
    ($expr:expr, $env:expr) => {
        Ok($crate::expr::ProcedureReturn::TailCall($expr, $env.clone()))
    };
}
pub(crate) use proc_result_tailcall;

/// The type of a procedure function.
///
/// `args` is a list of arguments passed to the procedure.
/// `env` is a reference to the environment in which the procedure is called.
/// The result is either a value or a tail call.
pub type ProcedureFn = fn(args: Exprs, env: &mut EnvRef) -> ProcedureResult;

impl Procedure {
    pub fn new_atomic(name: String, kind: ProcedureKind, proc: ProcedureFn, arity: Arity) -> Self {
        Procedure::Atomic(AtomicProcedure::new(name, kind, proc, arity))
    }

    pub fn new_compound(
        name: Option<String>,
        params: ProcedureParams,
        body: Body,
        env: EnvRef,
    ) -> Self {
        debug!(
            "creating procedure with name {:?}, params: {:?} and body: {:?}",
            name, params, body
        );
        Procedure::Compound(CompoundProcedure::new(name, params, body, env))
    }

    pub fn is_special_form(&self) -> bool {
        match self {
            Procedure::Atomic(proc) => proc.is_special_form(),
            Procedure::Compound(_) => false,
        }
    }

    pub fn arity(&self) -> Arity {
        match self {
            Procedure::Atomic(proc) => proc.arity(),
            Procedure::Compound(proc) => match &proc.params {
                ProcedureParams::Fixed(params) => Arity::Exact(params.len()),
                ProcedureParams::Variadic(_) => Arity::Any,
                ProcedureParams::Mixed(params, _) => Arity::AtLeast(params.len()),
            },
        }
    }
}

impl NamedProcedure for Procedure {
    fn name_stored(&self) -> Option<&str> {
        match self {
            Procedure::Atomic(proc) => proc.name_stored(),
            Procedure::Compound(proc) => proc.name_stored(),
        }
    }
}

impl std::fmt::Display for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = self.name();
        match self {
            Procedure::Atomic(_) => {
                if self.is_special_form() {
                    write!(f, "#<special form '{}'>", name)
                } else {
                    write!(f, "#<atomic procedure '{}'>", name)
                }
            }
            Procedure::Compound(_) => write!(f, "#<compound procedure '{}'>", name),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Represents the kind of a procedure.
pub enum ProcedureKind {
    /// A special form.
    /// Arguments *are not* evaluated before the special form is called.
    SpecialForm,
    /// A normal procedure.
    /// Arguments *are* evaluated before the procedure is called.
    Procedure,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Represents the arity of a procedure.
///
/// It is used to check the number of arguments, when the procedure is called.
pub enum Arity {
    /// Exact number of arguments.
    Exact(usize),
    /// At least this many arguments.
    AtLeast(usize),
    /// From min to max arguments.
    Range(usize, usize),
    /// Any number of arguments.
    Any,
}

impl std::fmt::Display for Arity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arity::Exact(count) => write!(f, "{}", count),
            Arity::AtLeast(count) => write!(f, "at least {}", count),
            Arity::Range(min, max) => write!(f, "{} to {}", min, max),
            Arity::Any => write!(f, "any"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AtomicProcedure {
    name: Option<String>,
    kind: ProcedureKind,
    arity: Arity,
    proc: ProcedureFn,
}

impl AtomicProcedure {
    pub fn new(name: String, kind: ProcedureKind, proc: ProcedureFn, arity: Arity) -> Self {
        AtomicProcedure {
            name: Some(name),
            kind,
            arity,
            proc,
        }
    }

    pub fn is_special_form(&self) -> bool {
        matches!(self.kind, ProcedureKind::SpecialForm)
    }

    pub fn proc(&self) -> ProcedureFn {
        self.proc
    }

    pub fn arity(&self) -> Arity {
        self.arity
    }
}

impl NamedProcedure for AtomicProcedure {
    fn name_stored(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProcedureParams {
    Fixed(Vec<String>),
    Variadic(String),
    Mixed(Vec<String>, String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    body: Exprs,
}

impl Body {
    pub fn new(body: Exprs) -> Self {
        Body { body }
    }

    pub fn new_single(expr: Expr) -> Self {
        Body { body: exprs![expr] }
    }

    pub fn as_exprs(&self) -> &Exprs {
        &self.body
    }
}

impl From<Exprs> for Body {
    fn from(body: Exprs) -> Self {
        Body::new(body)
    }
}

impl From<Expr> for Body {
    fn from(expr: Expr) -> Self {
        Body::new_single(expr)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompoundProcedure {
    name: Option<String>,
    pub params: ProcedureParams,
    pub body: Box<Body>,
    pub env: EnvRef,
}

impl CompoundProcedure {
    pub fn new(name: Option<String>, params: ProcedureParams, body: Body, env: EnvRef) -> Self {
        CompoundProcedure {
            name,
            params,
            env,
            body: Box::new(body),
        }
    }
}

impl NamedProcedure for CompoundProcedure {
    fn name_stored(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
