use core::fmt;

use super::list::{List, ListKind};

use crate::{
    //environment::EnvRef,
    //interpreter::{eval_expr, EvalError, EvalResult},
    list::{DisplayList, ListLocation},
};

use std::collections::VecDeque;

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Symbol(String),
    String(String),
    Char(char),
    List(List),
    Void,
    //Procedure(ProcedureData),
    Boolean(bool)
}

pub type Exprs = VecDeque<Expr>;

macro_rules! exprs {
    ($($x:expr),*) => {{
        #[allow(unused_mut)]
        let mut exprs = $crate::ast::Exprs::new();
        $(
            exprs.push_back($x);
        )*
        exprs
    }};
    ($($x:expr,)*) => (exprs![$($x),*])
}
pub(crate) use exprs;

type ExprIntoResult<T> = Result<T, Expr>;

impl Expr {
    pub fn new_empty_list() -> Self {
        Expr::List(List::new_proper(Exprs::new()))
    }

    pub fn new_dotted_list(list: Exprs) -> Self {
        Expr::List(List::new_dotted(list))
    }

    pub fn new_proper_list(list: Exprs) -> Self {
        Expr::List(List::new_proper(list))
    }

    pub fn new_list(list: Exprs, kind: ListKind) -> Self {
        if list.is_empty() {
            return Expr::new_empty_list();
        }

        match kind {
            ListKind::Proper => Expr::new_proper_list(list),
            ListKind::Dotted => Expr::new_dotted_list(list),
        }
    }

    pub fn into_list(self) -> ExprIntoResult<List> {
        match self {
            Expr::List(list) => Ok(list),
            _ => Err(self),
        }
    }
}

/*pub trait Procedure {
    fn apply(&self, args: &[Expr], env: &mut EnvRef) -> EvalResult;
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProcedureData {
    pub name: Option<String>,
    pub data: ProcedureType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProcedureType {
    Atomic(AtomicProcedure),
    Compound(CompoundProcedure),
}

impl ProcedureData {
    pub fn new_atomic(name: String, procedure: fn(&[Expr], &mut EnvRef) -> EvalResult) -> Self {
        ProcedureData {
            name: Some(name),
            data: ProcedureType::Atomic(AtomicProcedure(procedure)),
        }
    }

    pub fn new_compound(
        name: Option<String>,
        params: Vec<String>,
        body: Expr,
        env: &mut EnvRef,
    ) -> Self {
        ProcedureData {
            name,
            data: ProcedureType::Compound(CompoundProcedure {
                params,
                body: Box::new(body),
                env: env.clone(),
            }),
        }
    }
}

impl Procedure for ProcedureData {
    fn apply(&self, args: &[Expr], env: &mut EnvRef) -> EvalResult {
        match &self.data {
            ProcedureType::Atomic(primitive) => primitive.apply(args, env),
            ProcedureType::Compound(user_defined) => user_defined.apply(args, env),
        }
    }
}

impl fmt::Display for ProcedureData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match &self.name {
            Some(name) => name,
            None => "anon",
        };
        match self.data {
            ProcedureType::Atomic(_) => write!(f, "#<atomic procedure '{}'>", name),
            ProcedureType::Compound(_) => write!(f, "#<compound procedure '{}'>", name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AtomicProcedure(fn(&[Expr], &mut EnvRef) -> EvalResult);

impl Procedure for AtomicProcedure {
    fn apply(&self, args: &[Expr], env: &mut EnvRef) -> EvalResult {
        (self.0)(args, env)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompoundProcedure {
    pub params: Vec<String>,
    pub body: Box<Expr>,
    pub env: EnvRef,
}

impl Procedure for CompoundProcedure {
    fn apply(&self, args: &[Expr], args_env: &mut EnvRef) -> EvalResult {
        let mut eval_env = self.env.clone().extend();

        for (i, param) in self.params.iter().enumerate() {
            let arg = match args.get(i) {
                Some(arg) => eval_expr(arg, args_env)?,
                None => {
                    return Err(EvalError::RuntimeError(
                        "not enough arguments for procedure".to_string(),
                    ))
                }
            };
            eval_env.add(param, arg).unwrap();
        }

        eval_expr(&self.body, &mut eval_env)
    }
}*/

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Void => write!(f, "#<void>"),
            Expr::Integer(int) => write!(f, "{}", int),
            Expr::Float(float) => write!(f, "{}", float),
            Expr::Symbol(symbol) => write!(f, "{}", symbol),
            Expr::String(string) => write!(f, "{}", string),
            Expr::Char(char) => write!(f, "{}", char),
            //Expr::Procedure(proc) => write!(f, "{}", proc),
            Expr::Boolean(bool) => write!(f, "{}", bool),
            Expr::List(list) => list.fmt_with_location(f, ListLocation::Outer),
        }
    }
}