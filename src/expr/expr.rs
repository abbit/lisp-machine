use super::{
    list::{List, ListKind},
    port::Port,
    procedure::Procedure,
};
use core::fmt;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(PartialEq, Debug, Clone)]
/// Represents all possible values in interpreter.
///
/// This is the only type that can be used in interpreter.
/// All other types are converted to this one.
pub enum Expr {
    /// Integer number
    Integer(i64),
    /// Real number
    Float(f64),
    /// Symbol
    Symbol(String),
    /// A reference to mutable string.
    String(Rc<RefCell<String>>),
    /// Character
    Char(char),
    /// Boolean
    Boolean(bool),
    /// An immutable list of expressions.
    ///
    /// `'()` (null) is represented as empty list.
    ///  Dotted lists (nested pairs) and proper lists are represented as list of expressions.
    ///
    /// Internal `List` struct does not exported, so you can't construct list directly.
    /// To construct a list you should use [`Expr::new_empty_list`], [`Expr::new_proper_list`] and [`Expr::new_dotted_list`] instead.
    ///
    /// NOTE: Since lists are immutable, they are flattened on creation.
    /// This means that `(1 2 3)` and `(1 . (2 . (3 . '()))` are represented as same list internally.
    List(List),
    /// Unspecified value
    ///
    /// <div class="warning">
    /// This expression is created only on evaluation stage and evaluating it will cause error.
    /// </div>
    Void,
    /// Procedure
    ///
    /// <div class="warning">
    /// This expression is created only on evaluation stage and evaluating it will cause error.
    ///
    /// Because of this, internal <code>Procedure</code> struct does not exported, so you can't construct procedure directly.
    /// </div>
    Procedure(Procedure),
    /// Port
    Port(Rc<RefCell<Port>>),
}

/// A list of [`Expr`]s. Also can be created with [`exprs!`] macro.
///
/// [`exprs!`]: ./macro.exprs.html
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

/// A helper macro to construct [`Exprs`] from given expressions
#[macro_export]
macro_rules! exprs {
    () => {{
        $crate::Exprs::new()
    }};
    ($($x:expr),*) => {{
        #[allow(unused_mut)]
        let mut exprs = $crate::Exprs::new();
        $(
            exprs.push_back($x);
        )*
        exprs
    }};
    ($($x:expr,)*) => (exprs![$($x),*])
}

/// Represents result of conversion from [`Expr`] to another type
///
/// If conversion was successful, returns `Ok(T)`, where `T` is a type that implements [`FromExpr`].
///
/// If conversion failed, returns `Err(Expr)` with original [`Expr`].
///
/// You can safely [`unwrap`](#method.unwrap) this result, if converting to [`Expr`].
pub type FromExprResult<T> = Result<T, Expr>;

impl Expr {
    /// Creates new empty list `'()`
    pub fn new_empty_list() -> Self {
        Expr::List(List::new_empty())
    }

    /// Creates new dotted list like `'(1 2 . 3)` from [`Exprs`]
    ///
    /// Last element of `list` is used as `cdr` of last nested pair
    ///
    /// If list is empty, returns empty list
    pub fn new_dotted_list(mut list: Exprs) -> Self {
        let tail = match list.pop_back() {
            Some(tail) => tail,
            None => return Expr::new_empty_list(),
        };
        Expr::List(List::new_dotted(list, tail))
    }

    /// Creates new proper list like `'(1 2 3)` from [`Exprs`]
    pub fn new_proper_list(list: Exprs) -> Self {
        Expr::List(List::new_proper(list))
    }

    pub(crate) fn new_list(list: Exprs, kind: ListKind) -> Self {
        match kind {
            ListKind::Proper => Expr::new_proper_list(list),
            ListKind::Dotted => Expr::new_dotted_list(list),
        }
    }

    /// Creates new [`Expr::String`] from any type that implements [`Into<String>`]
    pub fn new_string<S: Into<String>>(string: S) -> Self {
        Expr::String(Rc::new(RefCell::new(string.into())))
    }

    /// Creates new [`Expr::Symbol`] from any type that implements [`Into<String>`]
    pub fn new_symbol<S: Into<String>>(string: S) -> Self {
        Expr::Symbol(string.into())
    }

    pub(crate) fn new_port(port: Port) -> Self {
        Self::Port(Rc::new(RefCell::new(port)))
    }

    /// Returns string representation of the type of `self`
    // Note: This method is named `kind` instead of `type` because `type` is a reserved keyword
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
            Expr::Port(_) => "port",
        }
    }

    /// Returns true if `self` represents a true value
    pub fn is_truthy(&self) -> bool {
        match self {
            Expr::Boolean(boolean) => *boolean,
            _ => true,
        }
    }

    // check methods

    /// Checks if `self` is a [`Expr::Symbol`]
    pub fn is_symbol(&self) -> bool {
        matches!(self, Expr::Symbol(_))
    }

    /// Checks if `self` is a [`Expr::Symbol`] with given `name`
    pub fn is_specific_symbol(&self, name: &str) -> bool {
        matches!(self, Expr::Symbol(sym) if sym == name)
    }

    /// checks if `self` is a [`Expr::List`]
    pub fn is_list(&self) -> bool {
        matches!(self, Expr::List(_))
    }

    /// Checks if `self` represents an empty list `'()`
    pub fn is_empty_list(&self) -> bool {
        matches!(self, Expr::List(list) if list.is_empty())
    }

    /// Checks if `self` represents a proper list
    pub fn is_proper_list(&self) -> bool {
        matches!(self, Expr::List(list) if list.is_proper())
    }

    /// Checks if `self` represents a dotted list
    /// This means that last element of last nested list is not `'()`
    pub fn is_dotted_list(&self) -> bool {
        matches!(self, Expr::List(list) if list.is_dotted())
    }

    /// Checks if `self` is a [`Expr::String`]
    pub fn is_string(&self) -> bool {
        matches!(self, Expr::String(_))
    }

    /// Checks if `self` is a [`Expr::Char`]
    pub fn is_char(&self) -> bool {
        matches!(self, Expr::Char(_))
    }

    /// Checks if `self` is a [`Expr::Integer`]
    pub fn is_integer(&self) -> bool {
        matches!(self, Expr::Integer(_))
    }

    /// Checks if `self` is a [`Expr::Float`]
    pub fn is_float(&self) -> bool {
        matches!(self, Expr::Float(_))
    }

    /// Checks if `self` is a [`Expr::Boolean`]
    pub fn is_boolean(&self) -> bool {
        matches!(self, Expr::Boolean(_))
    }

    /// Checks if `self` is a [`Expr::Void`]
    pub fn is_void(&self) -> bool {
        matches!(self, Expr::Void)
    }

    /// Checks if `self` is a [`Expr::Procedure`]
    pub fn is_procedure(&self) -> bool {
        matches!(self, Expr::Procedure(_))
    }

    /// Checks if `self` is a [`Expr::Port`]
    pub fn is_port(&self) -> bool {
        matches!(self, Expr::Port(_))
    }

    // exctraction methods

    /// Tries to convert `self` into `T`, which is a type that implements [`FromExpr`]
    pub fn into<T: FromExpr>(self) -> FromExprResult<T> {
        T::from_expr(self)
    }

    pub(crate) fn into_boolean(self) -> FromExprResult<bool> {
        match self {
            Expr::Boolean(boolean) => Ok(boolean),
            _ => Err(self),
        }
    }

    pub(crate) fn into_char(self) -> FromExprResult<char> {
        match self {
            Expr::Char(char) => Ok(char),
            _ => Err(self),
        }
    }

    pub(crate) fn into_integer(self) -> FromExprResult<i64> {
        match self {
            Expr::Integer(integer) => Ok(integer),
            _ => Err(self),
        }
    }

    pub(crate) fn into_float(self) -> FromExprResult<f64> {
        match self {
            Expr::Float(float) => Ok(float),
            _ => Err(self),
        }
    }

    pub(crate) fn into_symbol(self) -> FromExprResult<String> {
        match self {
            Expr::Symbol(symbol) => Ok(symbol),
            _ => Err(self),
        }
    }

    pub(crate) fn into_string(self) -> FromExprResult<Rc<RefCell<String>>> {
        match self {
            Expr::String(string) => Ok(Rc::clone(&string)),
            _ => Err(self),
        }
    }

    pub(crate) fn into_list(self) -> FromExprResult<List> {
        match self {
            Expr::List(list) => Ok(list),
            _ => Err(self),
        }
    }

    pub(crate) fn into_procedure(self) -> FromExprResult<Procedure> {
        match self {
            Expr::Procedure(proc) => Ok(proc),
            _ => Err(self),
        }
    }

    pub(crate) fn into_port(self) -> FromExprResult<Rc<RefCell<Port>>> {
        match self {
            Expr::Port(port) => Ok(port),
            _ => Err(self),
        }
    }

    pub(crate) fn as_list(&self) -> Option<&List> {
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
            Expr::Char(ch) => write!(f, "'{}'", ch),
            Expr::Procedure(proc) => write!(f, "{}", proc),
            Expr::Boolean(bool) => write!(f, "{}", if *bool { "#t" } else { "#f" }),
            Expr::List(list) => write!(f, "{}", list),
            Expr::Port(port) => write!(f, "{}", port.borrow()),
        }
    }
}

impl From<bool> for Expr {
    fn from(boolean: bool) -> Self {
        Expr::Boolean(boolean)
    }
}

impl From<char> for Expr {
    fn from(char: char) -> Self {
        Expr::Char(char)
    }
}

impl From<i64> for Expr {
    fn from(integer: i64) -> Self {
        Expr::Integer(integer)
    }
}

impl From<f64> for Expr {
    fn from(float: f64) -> Self {
        Expr::Float(float)
    }
}

impl From<String> for Expr {
    fn from(string: String) -> Self {
        Expr::new_string(string)
    }
}

impl From<&str> for Expr {
    fn from(string: &str) -> Self {
        Expr::new_string(string.to_string())
    }
}

impl From<List> for Expr {
    fn from(list: List) -> Self {
        Expr::List(list)
    }
}

impl From<Procedure> for Expr {
    fn from(proc: Procedure) -> Self {
        Expr::Procedure(proc)
    }
}

impl<T: Into<Expr>> From<Vec<T>> for Expr {
    fn from(vec: Vec<T>) -> Self {
        Expr::List(List::new_proper(
            vec.into_iter().map(|e| e.into()).collect(),
        ))
    }
}

impl<T: Into<Expr>> From<VecDeque<T>> for Expr {
    fn from(vec: VecDeque<T>) -> Self {
        Expr::List(List::new_proper(
            vec.into_iter().map(|e| e.into()).collect(),
        ))
    }
}

impl<A: Into<Expr>, B: Into<Expr>> From<(A, B)> for Expr {
    fn from((a, b): (A, B)) -> Self {
        Expr::List(List::new_dotted(exprs![a.into()], b.into()))
    }
}

/// The exit point for turning [`Expr`] into Rust types.
///
/// This trait implemented for most primitives.
/// You can also manually implement this for any type.
///
/// # Implemented conversions
/// | [`Expr`] | Rust type |
/// | ---- | ---- |
/// | [`Expr::Void`] | [`unit`]
/// | [`Expr::Boolean`] | [`bool`]
/// | [`Expr::Char`] | [`char`]
/// | [`Expr::Integer`] | [`i64`]
/// | [`Expr::Float`] | [`f64`]
/// | [`Expr::Symbol`] | [`String`]
/// | [`Expr::String`] | [`Rc<RefCell<String>>`]
/// | proper [`Expr::List`] | [`Vec<T>`] or [`VecDeque<T>`], where `T` is a type that implements [`FromExpr`]
/// | dotted [`Expr::List`] with 2 elements | `(A, B)`, where `A` and `B` are types that implements [`FromExpr`]
pub trait FromExpr: Sized {
    /// Tries to convert [`Expr`] into `Self`. Returns [`FromExprResult<Self>`] with result of conversion.
    fn from_expr(expr: Expr) -> FromExprResult<Self>;
}

impl FromExpr for () {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        match expr {
            Expr::Void => Ok(()),
            _ => Err(expr),
        }
    }
}

impl FromExpr for Expr {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        Ok(expr)
    }
}

impl FromExpr for bool {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        expr.into_boolean()
    }
}

impl FromExpr for char {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        expr.into_char()
    }
}

impl FromExpr for i64 {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        expr.into_integer()
    }
}

impl FromExpr for f64 {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        expr.into_float()
    }
}

impl FromExpr for String {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        expr.into_symbol()
    }
}

impl FromExpr for Rc<RefCell<String>> {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        expr.into_string()
    }
}

impl<T: FromExpr> FromExpr for Vec<T> {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        match expr {
            Expr::List(list) if list.is_proper() => {
                list.into_exprs().into_iter().map(T::from_expr).collect()
            }
            _ => Err(expr),
        }
    }
}

impl<T: FromExpr> FromExpr for VecDeque<T> {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        match expr {
            Expr::List(list) if list.is_proper() => {
                list.into_exprs().into_iter().map(T::from_expr).collect()
            }
            _ => Err(expr),
        }
    }
}

impl<A: FromExpr, B: FromExpr> FromExpr for (A, B) {
    fn from_expr(expr: Expr) -> FromExprResult<Self> {
        match &expr {
            Expr::List(list) => {
                if list.len() == 2 {
                    let mut iter = list.clone().into_iter();
                    let a = iter.next().unwrap();
                    let b = iter.next().unwrap();
                    Ok((A::from_expr(a)?, B::from_expr(b)?))
                } else {
                    Err(expr)
                }
            }
            _ => Err(expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exprs;

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
