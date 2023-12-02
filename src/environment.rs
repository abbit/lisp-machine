use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{Expr, ProcedureData},
    builtin,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Env {
    pub bindings: HashMap<String, Expr>,
    pub parent: Option<EnvRef>,
}

impl Env {
    fn new() -> Self {
        Env {
            bindings: HashMap::new(),
            parent: None,
        }
    }

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

    fn add(&mut self, name: &str, val: Expr) -> Result<(), String> {
        self.bindings.insert(name.to_string(), val);
        Ok(())
    }

    fn set(&mut self, name: &str, val: Expr) -> Result<(), String> {
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), val);
            Ok(())
        } else {
            match &mut self.parent {
                Some(parent) => parent.set(name, val),
                None => Err(format!("symbol '{}' is not defined", name)),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnvRef(Rc<RefCell<Env>>);

// macro for creating a new environment with the default bindings
macro_rules! new_environment(
    { $($key:expr => $value:expr),* } => {
        {
            let mut env = crate::environment::EnvRef::default();
            $(
                env.add($key, Expr::Procedure(ProcedureData::new_atomic($key.to_string(), $value))).unwrap();
            )*
            env
        }
    };
);

impl EnvRef {
    pub fn new() -> Self {
        EnvRef(Rc::new(RefCell::new(Env::new())))
    }

    pub fn extend(self) -> Self {
        EnvRef(Rc::new(RefCell::new(Env::extend(self))))
    }

    pub fn get(&self, name: &str) -> Option<Expr> {
        self.0.borrow().get(name)
    }

    pub fn add(&mut self, name: &str, val: Expr) -> Result<(), String> {
        self.0.borrow_mut().add(name, val)
    }

    pub fn set(&mut self, name: &str, val: Expr) -> Result<(), String> {
        self.0.borrow_mut().set(name, val)
    }
}

impl Default for EnvRef {
    fn default() -> Self {
        EnvRef::new()
    }
}

pub fn new_root_env() -> EnvRef {
    new_environment! {
        "define" => builtin::define,
        "set!" => builtin::set,
        "lambda" => builtin::lambda,

        "begin" => builtin::begin,
        // "quote" => builtin::quote,
        // "eval" => builtin::eval,
        "list" => builtin::list,
        "apply" => builtin::apply,

        "+" => builtin::add,
        "-" => builtin::sub,
        "*" => builtin::mult,
        "/" => builtin::divide,

        "exit" => builtin::exit
    }
}
