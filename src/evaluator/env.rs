use super::primitives::{eval, forms, lists, modularity, nums, system, convert, bool, types, chars, equal};
use crate::{
    evaluator::primitives::strings,
    expr::{Expr, Procedure},
};
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct EnvRef(Rc<RefCell<Env>>);

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
        forms::define_macro,
        // evaluation
        eval::eval,
        eval::apply,
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
        nums::abs,
        nums::sqrt,
        nums::square,
        nums::expt,
        nums::min,
        nums::max,
        nums::round,
        nums::truncate,
        nums::ceiling,
        nums::floor,
        nums::is_integer,
        nums::quotient,
        nums::remainder,
        nums::modulo,
        // bool
        bool::and,
        bool::or,
        // equal
        equal::eqv,
        equal::eq,
        equal::equal,
        // list operations
        lists::cons,
        lists::car_,
        lists::cdr_,
        lists::list_,
        lists::make_list,
        // type convertion
        convert::number_to_string,
        convert::string_to_number,
        convert::char_to_integer,
        convert::integer_to_char,
        // type checking
        // "null?" => builtin::is_null,
        types::is_pair,
        types::is_number,
        // "symbol?" => builtin::is_symbol,
        types::is_string,
        // "boolean?" => builtin::is_boolean,
        // "procedure?" => builtin::is_procedure,
        types::is_char,
        // system interaction
        system::read,
        system::read_line,
        system::display,
        system::newline,
        system::exit,
        strings::string_set,
        //strings
        strings::string_eq,
        strings::string_lt,
        strings::string_gt,
        strings::string_le,
        strings::string_ge,
        strings::make_string,
        strings::_string,
        strings::string_length,
        strings::substring,
        strings::string_upcase,
        strings::string_downcase,
        strings::string_foldcase,
        strings::string_ref,
        strings::string_append,
        strings::string_copy_,
        strings::string_fill,
        // chars
        chars::char_upcase,
        chars::char_downcase,
        chars::char_foldcase,
        chars::is_char_whitespace,
        chars::is_char_upper_case,
        chars::is_char_lower_case,
        chars::is_char_alphabetic,
        chars::is_char_numeric,
        chars::digit_value,
    }

    env
}
