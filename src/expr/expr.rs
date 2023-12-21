use core::fmt;

use crate::utils::debug;

use super::{
    list::{DisplayList, List, ListKind, ListLocation},
    procedure::Procedure,
};

use std::collections::VecDeque;

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Symbol(String),
    String(String),
    Char(char),
    Boolean(bool),
    List(List),

    // expressions, that created on evaluation stage
    Void,
    Procedure(Procedure),
}

pub type Exprs = VecDeque<Expr>;

macro_rules! exprs {
    ($($x:expr),*) => {{
        #[allow(unused_mut)]
        let mut exprs = $crate::expr::Exprs::new();
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

    /// Returns kind of expression as string
    /// Note: This method is named `kind` instead of `type` because `type` is a reserved keyword
    pub fn kind(&self) -> &'static str {
        match self {
            Expr::Boolean(_) => "boolean",
            Expr::Char(_) => "char",
            Expr::Integer(_) => "integer",
            Expr::Float(_) => "float",
            Expr::Symbol(_) => "symbol",
            Expr::String(_) => "string",
            Expr::List(list) => match list.kind() {
                ListKind::Proper => "list",
                ListKind::Dotted => "dotted list",
            },
            Expr::Void => "void",
            Expr::Procedure(_) => "procedure",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Expr::Boolean(boolean) => *boolean,
            _ => true,
        }
    }

    // check methods

    pub fn is_symbol(&self) -> bool {
        matches!(self, Expr::Symbol(_))
    }

    pub fn is_empty_list(&self) -> bool {
        matches!(self, Expr::List(list) if list.is_empty())
    }

    pub fn is_list(&self) -> bool {
        debug!("is_list: {:?}", self);
        matches!(self, Expr::List(_))
    }

    // exctraction methods

    pub fn into_boolean(self) -> ExprIntoResult<bool> {
        match self {
            Expr::Boolean(boolean) => Ok(boolean),
            _ => Err(self),
        }
    }

    pub fn into_char(self) -> ExprIntoResult<char> {
        match self {
            Expr::Char(char) => Ok(char),
            _ => Err(self),
        }
    }

    pub fn into_integer(self) -> ExprIntoResult<i64> {
        match self {
            Expr::Integer(integer) => Ok(integer),
            _ => Err(self),
        }
    }

    pub fn into_float(self) -> ExprIntoResult<f64> {
        match self {
            Expr::Float(float) => Ok(float),
            _ => Err(self),
        }
    }

    pub fn into_symbol(self) -> ExprIntoResult<String> {
        match self {
            Expr::Symbol(symbol) => Ok(symbol),
            _ => Err(self),
        }
    }

    pub fn into_string(self) -> ExprIntoResult<String> {
        match self {
            Expr::String(string) => Ok(string),
            _ => Err(self),
        }
    }

    pub fn into_list(self) -> ExprIntoResult<List> {
        match self {
            Expr::List(list) => Ok(list),
            _ => Err(self),
        }
    }

    pub fn into_procedure(self) -> ExprIntoResult<Procedure> {
        match self {
            Expr::Procedure(proc) => Ok(proc),
            _ => Err(self),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Void => write!(f, "#<void>"),
            Expr::Integer(int) => write!(f, "{}", int),
            Expr::Float(float) => write!(f, "{}", float),
            Expr::Symbol(symbol) => write!(f, "{}", symbol),
            Expr::String(string) => write!(f, "{}", string),
            Expr::Char(char) => write!(f, "{}", char),
            Expr::Procedure(proc) => write!(f, "{}", proc),
            Expr::Boolean(bool) => write!(f, "{}", bool),
            Expr::List(list) => list.fmt_with_location(f, ListLocation::Outer),
        }
    }
}
