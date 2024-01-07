use super::primitives::{
    booleans, equivalence, eval, forms, lists, macros, modularity, nums, strings, system,
};
use crate::expr::{Expr, Procedure};
use core::fmt;
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

#[derive(Debug, PartialEq, Clone, Default)]
struct Env {
    bindings: HashMap<String, Expr>,
    macros: HashMap<String, Procedure>,
    parent: Option<EnvRef>,
    cwd: PathBuf,
}

impl Env {
    fn extend(parent: EnvRef) -> Env {
        let cwd = parent.0.borrow().cwd.clone();

        Env {
            cwd,
            bindings: HashMap::new(),
            macros: HashMap::new(),
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

    fn get_macro(&self, name: &str) -> Option<Procedure> {
        match self.macros.get(name) {
            Some(value) => Some(value.clone()),
            None => self.parent.as_ref().and_then(|e| e.get_macro(name).clone()),
        }
    }

    fn add_macro(&mut self, name: String, macro_: Procedure) {
        self.macros.insert(name, macro_);
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

#[derive(Clone, Default)]
pub struct EnvRef(Rc<RefCell<Env>>);

impl PartialEq for EnvRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for EnvRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EnvRef({:?})", Rc::as_ptr(&self.0))
    }
}

impl EnvRef {
    pub fn is_root(&self) -> bool {
        self.0.borrow().parent.is_none()
    }

    pub fn extend(&self) -> Self {
        EnvRef(Rc::new(RefCell::new(Env::extend(self.clone()))))
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

    pub fn get_macro(&self, name: &str) -> Option<Procedure> {
        self.0.borrow().get_macro(name)
    }

    pub fn add_macro(&mut self, name: String, macro_: Procedure) {
        self.0.borrow_mut().add_macro(name, macro_)
    }

    pub fn cwd(&self) -> PathBuf {
        self.0.borrow().cwd.clone()
    }

    pub fn set_cwd(&mut self, cwd: PathBuf) {
        self.0.borrow_mut().cwd = cwd;
    }
}

pub fn new_root_env() -> EnvRef {
    let mut env = EnvRef::default();
    env.set_cwd(std::env::current_dir().expect("failed to get current working directory"));

    insert_procedures! {
        env,
        // core forms
        forms::lambda,
        forms::define,
        forms::set,
        forms::let_,
        forms::letrec,
        forms::quote,
        forms::quasiquote,
        forms::if_,
        forms::begin,
        // macros
        macros::define_macro,
        macros::gensym,
        // evaluation
        eval::eval,
        eval::apply,
        // equivalence
        equivalence::eqv,
        // boolean
        booleans::and,
        booleans::or,
        // modularity
        modularity::include,
        modularity::load,
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
        lists::is_null,
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
        strings::string_set,
    }

    env
}
