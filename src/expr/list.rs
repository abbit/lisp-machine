use super::expr::{Expr, Exprs};
use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct List {
    but_last: Exprs,
    last: Option<Box<Expr>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ListKind {
    Proper,
    Dotted,
}

impl List {
    pub fn new(but_last: Exprs, last: Option<Expr>) -> Self {
        List {
            but_last,
            last: last.map(Box::new),
        }
        .flatten()
    }

    pub fn new_empty() -> Self {
        List::new(Exprs::new(), None)
    }

    pub fn new_proper(list: Exprs) -> Self {
        List::new(list, None)
    }

    pub fn new_dotted(but_last: Exprs, last: Expr) -> Self {
        List::new(but_last, Some(last))
    }

    pub fn flatten(self) -> Self {
        let mut last = match self.last {
            None => return self,
            expr => expr.map(|e| *e),
        };
        let mut but_last = self.but_last;

        // flatten list
        while let Some(Expr::List(list)) = last {
            but_last.extend(list.but_last);
            last = list.last.map(|e| *e);
        }

        // NOTE: do not use `List::new` here, because it will infinetly recurse
        List {
            but_last,
            last: last.map(Box::new),
        }
    }

    pub fn len(&self) -> usize {
        self.but_last.len()
            + self
                .last
                .as_ref()
                .map_or(0, |expr| expr.as_list().map_or(1, |list| list.len()))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self) -> ListKind {
        match self.last {
            Some(ref expr) => expr.as_list().map_or(ListKind::Dotted, |list| list.kind()),
            None => ListKind::Proper,
        }
    }

    pub fn is_proper(&self) -> bool {
        matches!(self.kind(), ListKind::Proper)
    }

    pub fn is_dotted(&self) -> bool {
        matches!(self.kind(), ListKind::Dotted)
    }

    pub fn car(&self) -> Option<&Expr> {
        self.but_last.front()
    }

    /// Returns the first element of the list.
    /// Returns `None` if list is empty.
    pub fn pop_front(&mut self) -> Option<Expr> {
        self.but_last
            .pop_front()
            .or_else(|| self.last.take().map(|expr| *expr))
    }

    /// Splits list into first element and the rest of the list.
    /// Returns `Err(self)` if list is empty.
    pub fn split_first(mut self) -> Result<(Expr, List), List> {
        if self.is_empty() {
            return Err(self);
        }

        let first = self.pop_front().unwrap();
        Ok((first, self))
    }

    /// Returns iterator over the cdr of the list.
    pub fn cdr(&self) -> impl Iterator<Item = &Expr> {
        self.iter().skip(1)
    }

    /// Returns iterator over all elements except the last.
    pub fn but_last(&self) -> impl Iterator<Item = &Expr> + '_ {
        self.but_last.iter()
    }

    pub fn last(&self) -> Option<&Expr> {
        self.last.as_deref()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Expr> {
        self.into_iter()
    }

    pub fn into_exprs(self) -> Exprs {
        self.into_iter().collect()
    }
}

pub struct Iter<'a> {
    but_last_iter: std::collections::vec_deque::Iter<'a, Expr>,
    last: Option<&'a Expr>,
}

impl<'a> Iter<'a> {
    fn new(list: &'a List) -> Self {
        Iter {
            but_last_iter: list.but_last.iter(),
            last: list.last.as_deref(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Expr;

    fn next(&mut self) -> Option<Self::Item> {
        match self.but_last_iter.next() {
            Some(expr) => Some(expr),
            None => self.last.take(),
        }
    }
}

pub struct IntoIter {
    but_last_iter: std::collections::vec_deque::IntoIter<Expr>,
    last: Option<Expr>,
}

impl IntoIter {
    fn new(list: List) -> Self {
        IntoIter {
            but_last_iter: list.but_last.into_iter(),
            last: list.last.map(|expr| *expr),
        }
    }
}

impl Iterator for IntoIter {
    type Item = Expr;

    fn next(&mut self) -> Option<Self::Item> {
        match self.but_last_iter.next() {
            Some(expr) => Some(expr),
            None => self.last.take(),
        }
    }
}

impl IntoIterator for List {
    type Item = Expr;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = &'a Expr;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl From<List> for Exprs {
    fn from(list: List) -> Self {
        list.into_exprs()
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut but_last = self.but_last();
        write!(f, "(")?;
        if let Some(expr) = but_last.next() {
            write!(f, "{}", expr)?;
        }
        for expr in but_last {
            write!(f, " {}", expr)?;
        }
        if let Some(expr) = self.last() {
            write!(f, " . {}", expr)?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exprs;

    #[test]
    fn validate_empty() {
        let list = List::new_empty();
        assert!(list.is_empty());
        assert!(list.is_proper());
        assert_eq!(list.into_exprs(), exprs![]);
    }

    #[test]
    fn validate_proper() {
        let list = List::new_proper(exprs![Expr::Integer(1), Expr::Integer(2)]);
        assert_eq!(list.len(), 2);
        assert!(list.is_proper());
        assert_eq!(
            list.into_exprs(),
            exprs![Expr::Integer(1), Expr::Integer(2)]
        );
    }

    #[test]
    fn validate_dotted_simple() {
        let list = List::new_dotted(exprs![Expr::Integer(1), Expr::Integer(2)], Expr::Integer(3));
        assert_eq!(list.len(), 3);
        assert!(list.is_dotted());
        assert_eq!(
            list.into_exprs(),
            exprs![Expr::Integer(1), Expr::Integer(2), Expr::Integer(3)]
        );
    }

    #[test]
    fn validate_dotted_nested() {
        // (1 2 . (3 . 4))
        let list = List::new_dotted(
            exprs![Expr::Integer(1), Expr::Integer(2)],
            Expr::List(List::new_dotted(exprs![Expr::Integer(3)], Expr::Integer(4))),
        );
        assert_eq!(list.len(), 4);
        assert!(list.is_dotted());
        assert_eq!(
            list.into_exprs(),
            exprs![
                Expr::Integer(1),
                Expr::Integer(2),
                Expr::Integer(3),
                Expr::Integer(4),
            ]
        );
    }

    #[test]
    fn validate_dotted_with_proper_tail() {
        let list = List::new_dotted(
            exprs![Expr::Integer(1), Expr::Integer(2)],
            Expr::List(List::new_proper(exprs![Expr::Integer(3), Expr::Integer(4)])),
        );
        assert_eq!(list.len(), 4);
        assert!(list.is_proper());
        assert_eq!(
            list.into_exprs(),
            exprs![
                Expr::Integer(1),
                Expr::Integer(2),
                Expr::Integer(3),
                Expr::Integer(4),
            ]
        );
    }
}
