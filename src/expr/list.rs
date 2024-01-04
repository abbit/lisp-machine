use super::expr::{Expr, Exprs};

#[derive(Debug, PartialEq, Clone)]
pub struct List {
    data: Exprs,
    kind: ListKind,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ListKind {
    Proper,
    Dotted,
}

impl List {
    pub fn new_dotted(list: Exprs) -> Self {
        assert!(list.len() > 1, "dotted list must have at least 2 elements");
        List {
            data: list,
            kind: ListKind::Dotted,
        }
    }

    pub fn new_proper(list: Exprs) -> Self {
        List {
            data: list,
            kind: ListKind::Proper,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn kind(&self) -> ListKind {
        self.kind
    }

    pub fn is_proper(&self) -> bool {
        matches!(self.kind, ListKind::Proper)
    }

    pub fn is_dotted(&self) -> bool {
        matches!(self.kind, ListKind::Dotted)
    }

    pub fn car(&self) -> Option<&Expr> {
        self.data.front()
    }

    pub fn nth(&self, n: usize) -> Option<&Expr> {
        self.data.get(n)
    }

    pub fn pop_front(&mut self) -> Option<Expr> {
        self.data.pop_front()
    }

    pub fn split_first(mut self) -> Option<(Expr, List)> {
        self.data.pop_front().map(|first| (first, self))
    }

    pub fn cdr(&self) -> impl Iterator<Item = &Expr> + '_ {
        self.data.iter().skip(1)
    }

    pub fn but_last(&self) -> impl Iterator<Item = &Expr> + '_ {
        self.data.iter().take(self.data.len().saturating_sub(1))
    }

    pub fn last(&self) -> Option<&Expr> {
        self.data.back()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Expr> + '_ {
        self.data.iter()
    }

    pub fn into_vec_deque(self) -> Exprs {
        self.data
    }
}

impl IntoIterator for List {
    type Item = Expr;
    type IntoIter = std::collections::vec_deque::IntoIter<Expr>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl From<List> for Exprs {
    fn from(list: List) -> Self {
        list.into_vec_deque()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ListLocation {
    Outer,
    Inner,
}

pub trait DisplayList {
    fn fmt_with_location(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        list_location: ListLocation,
    ) -> std::fmt::Result;
}

impl DisplayList for List {
    fn fmt_with_location(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        list_location: ListLocation,
    ) -> std::fmt::Result {
        if self.len() == 2 {
            // write special forms with their syntax sugar
            match self.car().unwrap() {
                // write (quote <expr>) as '<expr>
                Expr::Symbol(symbol) if symbol == "quote" => {
                    return write!(f, "'{}", self.nth(1).unwrap());
                }
                // write (quasiquote <expr>) as `<expr>
                Expr::Symbol(symbol) if symbol == "quasiquote" => {
                    return write!(f, "`{}", self.nth(1).unwrap());
                }
                // write (unquote <expr>) as ,<expr>
                Expr::Symbol(symbol) if symbol == "unquote" => {
                    return write!(f, ",{}", self.nth(1).unwrap());
                }
                // write (unquote-splicing <expr>) as ,@<expr>
                Expr::Symbol(symbol) if symbol == "unquote-splicing" => {
                    return write!(f, ",@{}", self.nth(1).unwrap());
                }
                _ => {}
            };
        };

        // if list_location == ListLocation::Outer {
        // write outer list with leading and trailing parentheses
        write!(f, "(")?;
        // }

        if self.car().is_some() {
            let start = match list_location {
                // write first element without leading space, its first element in most outer list
                ListLocation::Outer => '\0',
                // write first element with leading space, its first element in inner list
                ListLocation::Inner => ' ',
            };
            write!(f, "{}{}", start, self.car().unwrap())?;
        }

        for expr in self.but_last().skip(1) {
            // write all elements except first and last with leading space
            write!(f, " {}", expr)?;
        }

        match (self.last(), self.kind()) {
            (Some(Expr::List(list)), _) => list.fmt_with_location(f, ListLocation::Inner)?,
            (Some(last), ListKind::Proper) => write!(f, " {}", last)?,
            (Some(last), ListKind::Dotted) => write!(f, " . {}", last)?,
            (None, _) => {}
        };

        // if list_location == ListLocation::Outer {
        // write outer list with leading and trailing parentheses
        write!(f, ")")?;
        // }

        std::fmt::Result::Ok(())
    }
}
