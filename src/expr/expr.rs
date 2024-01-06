use core::fmt;

use super::{
    list::{List, ListKind},
    procedure::Procedure,
};

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Symbol(String),
    String(Rc<RefCell<String>>),
    Char(char),
    Boolean(bool),
    List(List),

    // expressions, that created on evaluation stage
    Void,
    Procedure(Procedure),
}

pub type Exprs = VecDeque<Expr>;

pub trait AsExprs {
    /// Returns iterator over all elements except last one and last element
    /// If expressions is empty, returns `None`
    fn split_tail(self) -> Option<(impl Iterator<Item = Expr>, Expr)>;
}

impl AsExprs for Exprs {
    fn split_tail(self) -> Option<(impl Iterator<Item = Expr>, Expr)> {
        let tail = self.iter().last()?.clone();

        let len = self.len();
        let but_tail = self.into_iter().take(len.saturating_sub(1));
        Some((but_tail, tail))
    }
}

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
        Expr::List(List::new_empty())
    }

    pub fn new_dotted_list(mut list: Exprs) -> Self {
        let tail = match list.pop_back() {
            Some(tail) => tail,
            None => return Expr::new_empty_list(),
        };
        Expr::List(List::new_dotted(list, tail))
    }

    pub fn new_proper_list(list: Exprs) -> Self {
        Expr::List(List::new_proper(list))
    }

    pub fn new_list(list: Exprs, kind: ListKind) -> Self {
        match kind {
            ListKind::Proper => Expr::new_proper_list(list),
            ListKind::Dotted => Expr::new_dotted_list(list),
        }
    }

    pub fn new_string<S: Into<String>>(string: S) -> Self {
        Expr::String(Rc::new(RefCell::new(string.into())))
    }

    pub fn new_symbol<S: Into<String>>(string: S) -> Self {
        Expr::Symbol(string.into())
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

    pub fn into_string(self) -> ExprIntoResult<Rc<RefCell<String>>> {
        match self {
            Expr::String(string) => Ok(Rc::clone(&string)),
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

    pub fn as_list(&self) -> Option<&List> {
        match self {
            Expr::List(list) => Some(list),
            _ => None,
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
            Expr::String(string) => write!(f, "\"{}\"", string.borrow()),
            Expr::Char(char) => write!(f, "{}", char),
            Expr::Procedure(proc) => write!(f, "{}", proc),
            Expr::Boolean(bool) => write!(f, "{}", if *bool { "#t" } else { "#f" }),
            Expr::List(list) => write!(f, "{}", list),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::exprs;

    #[test]
    fn display_empty_list() {
        // ()
        let expr = Expr::new_empty_list();
        assert_eq!(format!("{}", expr), "()");
    }

    #[test]
    fn display_proper_list() {
        // (1 2 3)
        let expr =
            Expr::new_proper_list(exprs![Expr::Integer(1), Expr::Integer(2), Expr::Integer(3)]);
        assert_eq!(format!("{}", expr), "(1 2 3)");
    }

    #[test]
    fn display_dotted_list() {
        // (1 2 . 3)
        let expr =
            Expr::new_dotted_list(exprs![Expr::Integer(1), Expr::Integer(2), Expr::Integer(3)]);
        assert_eq!(format!("{}", expr), "(1 2 . 3)");
    }

    #[test]
    fn display_tailing_dotted_list() {
        // (1 . (2 . 3))
        let expr = Expr::new_dotted_list(exprs![
            Expr::Integer(1),
            Expr::new_dotted_list(exprs![Expr::Integer(2), Expr::Integer(3)])
        ]);
        assert_eq!(format!("{}", expr), "(1 2 . 3)");
    }

    #[test]
    fn display_nested_list() {
        // ((lambda (x) (* x x)) 3)
        let expr = Expr::new_proper_list(exprs![
            Expr::new_proper_list(exprs![
                Expr::Symbol("lambda".to_string()),
                Expr::new_proper_list(exprs![Expr::Symbol("x".to_string())]),
                Expr::new_proper_list(exprs![
                    Expr::Symbol("*".to_string()),
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("x".to_string())
                ],)
            ],),
            Expr::Integer(3)
        ]);
        assert_eq!(format!("{}", expr), "((lambda (x) (* x x)) 3)");
    }

    #[test]
    fn display_inner_dotted_list() {
        // ((lambda (x . y) (* x y)) 3 4)
        let expr = Expr::new_proper_list(exprs![
            Expr::new_proper_list(exprs![
                Expr::Symbol("lambda".to_string()),
                Expr::new_dotted_list(exprs![
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("y".to_string())
                ]),
                Expr::new_proper_list(exprs![
                    Expr::Symbol("*".to_string()),
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("y".to_string())
                ],)
            ],),
            Expr::Integer(3),
            Expr::Integer(4)
        ]);
        assert_eq!(format!("{}", expr), "((lambda (x . y) (* x y)) 3 4)");
    }
}
