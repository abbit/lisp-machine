use super::primitives::{
    chars, convert, equal, eval, forms, io, lists, macros, nums, ports, strings, system, types,
};
use crate::expr::{port::Port, Expr, FromExpr, FromExprResult, Procedure};
use core::fmt;
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
struct Env {
    bindings: HashMap<String, Expr>,
    macros: HashMap<String, Procedure>,
    parent: Option<EnvRef>,
    cwd: PathBuf,
    cur_input_port: Rc<RefCell<Port>>,
    cur_output_port: Rc<RefCell<Port>>,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            bindings: HashMap::new(),
            macros: HashMap::new(),
            parent: None,
            cwd: PathBuf::new(),
            cur_input_port: Rc::new(RefCell::new(Port::new_stdin())),
            cur_output_port: Rc::new(RefCell::new(Port::new_stdout())),
        }
    }
}

impl Env {
    fn extend(parent: EnvRef) -> Env {
        let cwd = parent.0.borrow().cwd.clone();
        let cur_input_port = parent.0.borrow().cur_input_port.clone();
        let cur_output_port = parent.0.borrow().cur_output_port.clone();

        Env {
            cwd,
            cur_input_port,
            cur_output_port,
            bindings: HashMap::new(),
            macros: HashMap::new(),
            parent: Some(parent),
        }
    }

    fn has(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    fn get(&self, name: &str) -> Option<Expr> {
        match self.bindings.get(name) {
            Some(value) => Some(value.clone()),
            None => self.parent.as_ref().and_then(|e| e.get_expr(name).clone()),
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

    fn has_macro(&self, name: &str) -> bool {
        self.macros.contains_key(name)
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
                $env.add(
                    $proc.0.to_string(),
                    $crate::expr::Procedure::new_atomic($proc.0.to_string(), $proc.1, $proc.2, $proc.3),
                );
            )*
        }
    };
);

#[derive(Clone, Default)]
/// Reference to the environment.
///
/// Environment holds:
/// - bindings of symbols to expressions.
/// - current working directory.
/// - reference to the parent environment.
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
    /// Checks if `self` is the root environment.
    pub fn is_root(&self) -> bool {
        self.0.borrow().parent.is_none()
    }

    /// Creates a reference to the deep copy of underlying environment.
    pub fn copy(&self) -> Self {
        EnvRef(Rc::new(RefCell::new(self.0.borrow().clone())))
    }

    /// Creates a reference to a new environment that has this environment as a parent.
    pub fn extend(&self) -> Self {
        EnvRef(Rc::new(RefCell::new(Env::extend(self.clone()))))
    }

    /// Checks if `self` contains a binding with `name`.
    pub fn has(&self, name: &str) -> bool {
        self.0.borrow().has(name)
    }

    /// Checks if `self` contains a macro with `name`.
    pub fn has_macro(&self, name: &str) -> bool {
        self.0.borrow().has_macro(name)
    }

    /// Returns the expression bound to `name`.

    /// If the no expression is found, returns [`None`].
    pub fn get_expr(&self, name: &str) -> Option<Expr> {
        self.0.borrow().get(name)
    }

    /// [`get_expr`](#method.get_expr) with a type conversion.
    ///
    /// If no binding is found for `name`, returns [`None`].
    ///
    /// If binding is found returns [`Some`] with result of conversion [`FromExprResult`].
    pub fn get<T: FromExpr>(&self, name: &str) -> Option<FromExprResult<T>> {
        Some(T::from_expr(self.get_expr(name)?))
    }

    /// Adds a new binding to the environment.
    /// If the binding already exists, it is overwritten.
    pub fn add<T: Into<Expr>>(&mut self, name: String, expr: T) {
        self.0.borrow_mut().add(name, expr.into())
    }

    /// Sets new value to an existing binding.
    /// Returns an [`Err`] if the binding does not exist.
    pub fn set<T: Into<Expr>>(&mut self, name: String, expr: T) -> Result<(), String> {
        self.0.borrow_mut().set(name, expr.into())
    }

    pub(super) fn get_macro(&self, name: &str) -> Option<Procedure> {
        self.0.borrow().get_macro(name)
    }

    pub(super) fn add_macro(&mut self, name: String, macro_: Procedure) {
        self.0.borrow_mut().add_macro(name, macro_)
    }

    /// Returns the current working directory of the environment.
    pub fn cwd(&self) -> PathBuf {
        self.0.borrow().cwd.clone()
    }

    /// Sets the current working directory of the environment.
    pub fn set_cwd(&mut self, cwd: PathBuf) {
        self.0.borrow_mut().cwd = cwd;
    }

    /// Returns the current input port of the environment.
    pub fn current_input_port(&self) -> Rc<RefCell<Port>> {
        self.0.borrow().cur_input_port.clone()
    }

    /// Returns the current output port of the environment.
    pub fn current_output_port(&self) -> Rc<RefCell<Port>> {
        self.0.borrow().cur_output_port.clone()
    }

    /// Sets the current input port of the environment.
    pub fn set_current_input_port(&mut self, port: Rc<RefCell<Port>>) {
        self.0.borrow_mut().cur_input_port = port;
    }

    /// Sets the current output port of the environment.
    pub fn set_current_output_port(&mut self, port: Rc<RefCell<Port>>) {
        self.0.borrow_mut().cur_output_port = port;
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
        forms::cond,
        forms::begin,
        forms::do_,
        // macros
        macros::define_macro,
        // evaluation
        eval::eval,
        eval::apply,
        // equivalence
        equal::eqv,
        equal::equal,
        // boolean
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
        // equal
        equal::eqv,
        equal::eq,
        equal::equal,
        // list operations
        lists::cons,
        lists::car_,
        lists::cdr_,
        lists::make_list,
        lists::list_copy,
        // type convertion
        convert::number_to_string,
        convert::string_to_number,
        convert::char_to_integer,
        convert::integer_to_char,
        convert::string_to_list,
        convert::symbol_to_string,
        convert::string_to_symbol,
        // type checking
        types::is_pair,
        types::is_number,
        types::is_symbol,
        types::is_string,
        types::is_procedure,
        types::is_char,
        types::is_port,
        // system interaction
        system::include,
        system::load,
        system::file_exists,
        system::exit,
        system::current_second,
        //strings
        strings::string_set,
        strings::string_eq,
        strings::string_lt,
        strings::string_gt,
        strings::string_le,
        strings::string_ge,
        strings::make_string,
        strings::string_,
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
        // ports
        ports::open_input_file,
        ports::open_output_file,
        ports::is_input_port,
        ports::is_output_port,
        ports::current_input_port,
        ports::current_output_port,
        ports::close_input_port,
        ports::close_output_port,
        ports::with_input_from_file,
        ports::with_output_to_file,
        ports::call_with_input_file,
        ports::call_with_output_file,
        // io
        io::read,
        io::read_char,
        io::read_string,
        io::write,
        io::write_char,
        io::write_string,
        io::display,
        io::newline,
    }

    env
}
