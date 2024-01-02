use super::primitives::{eval, forms, lists, nums, system};
use crate::expr::{Expr, Procedure};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq, Clone, Default)]
struct Env {
    bindings: HashMap<String, Expr>,
    parent: Option<EnvRef>,
}

impl Env {
    fn extend(parent: EnvRef) -> Env {
        Env {
            bindings: HashMap::new(),
            parent: Some(parent),
        }
    }

    fn get(&self, name: &str) -> Option<Expr> {
        match self.bindings.get(name) {
            Some(value) => Some(value.clone()),
            None => self.parent.as_ref().and_then(|e| e.get(name).clone()),
        }
    }

    fn add(&mut self, name: String, val: Expr) {
        self.bindings.insert(name, val);
    }

    fn set(&mut self, name: String, val: Expr) -> Result<(), String> {
        #[allow(clippy::map_entry)]
        // false positive, using contains_key() to avoid cloning name
        if self.bindings.contains_key(&name) {
            self.bindings.insert(name, val);
            Ok(())
        } else {
            match &mut self.parent {
                Some(parent) => parent.set(name, val),
                None => Err(format!("symbol '{}' is not defined", name)),
            }
        }
    }
}

// macro for adding special forms and built-in procedures to the environment
macro_rules! insert_procedures(
    { $env:expr, $($proc:expr,)* } => {
        {
            $(
                $env.add($proc.0.to_string(), Expr::Procedure(Procedure::new_atomic($proc.0.to_string(), $proc.1, $proc.2, $proc.3)));
            )*
        }
    };
);

#[derive(Debug, PartialEq, Clone, Default)]
pub struct EnvRef(Rc<RefCell<Env>>);

impl EnvRef {
    pub fn extend(self) -> Self {
        EnvRef(Rc::new(RefCell::new(Env::extend(self))))
    }

    pub fn get(&self, name: &str) -> Option<Expr> {
        self.0.borrow().get(name)
    }

    pub fn add(&mut self, name: String, val: Expr) {
        self.0.borrow_mut().add(name, val)
    }

    pub fn set(&mut self, name: String, val: Expr) -> Result<(), String> {
        self.0.borrow_mut().set(name, val)
    }
}

pub fn new_root_env() -> EnvRef {
    let mut env = EnvRef::default();

    insert_procedures! {
        env,
        // core forms
        forms::lambda,
        forms::define,
        forms::set,
        forms::quote,
        forms::quasiquote,
        forms::if_,
        forms::begin,
        // evaluation
        eval::eval,
        eval::apply,
         // arithmetic
        nums::add,
        nums::sub,
        nums::mult,
        nums::divide,
        // comparison
        nums::less,
        nums::equal,
        nums::more,
        // list operations
        lists::cons,
        lists::car_,
        lists::cdr_,
        lists::list_,
        // type checking
        // "null?" => builtin::is_null,
        // "pair?" => builtin::is_pair,
        // "number?" => builtin::is_number,
        // "symbol?" => builtin::is_symbol,
        // "string?" => builtin::is_string,
        // "boolean?" => builtin::is_boolean,
        // "procedure?" => builtin::is_procedure,
        // system interaction
        system::read,
        system::read_line,
        system::display,
        system::newline,
        system::exit,
    }

    env
}
