use std::{cell::RefCell, rc::Rc};

use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{Arity, Expr, Exprs, ProcedureResult, ProcedureReturn},
};

define_procedures! {
    number_to_string = ("number->string", number_to_string_fn, Arity::AtLeast(1)),
    string_to_number = ("string->number", string_to_number_fn, Arity::AtLeast(1)),
    char_to_integer = ("char->integer", char_to_integer_fn, Arity::Exact(1)),
    integer_to_char = ("integer->char", integer_to_char_fn, Arity::Exact(1)),
}

fn number_to_string_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let number = args.pop_front().unwrap();
    let radix_arg = args.pop_front().clone();

    let radix = match radix_arg {
        Some(arg) => match arg {
            Expr::Integer(r) if [2, 8, 10, 16].contains(&r) => r as u32,
            _ => {
                return Err(runtime_error!(
                    "radix must be one of 2, 8, 10, or 16, got {}",
                    arg.kind()
                ))
            }
        },
        None => 10,
    };

    let result = match number {
        Expr::Integer(n) => {
            let string_representation = Rc::new(RefCell::new(n.to_string_radix(radix)));
            Expr::String(string_representation)
        }
        Expr::Float(f) => {
            let string_representation = Rc::new(RefCell::new(format!("{}", f)));
            Expr::String(string_representation)
        }
        _ => {
            return Err(runtime_error!(
                "number->string is only supported for integers and inexact numbers with radix 10"
            ))
        }
    };

    Ok(result).map(ProcedureReturn::Value)
}

trait ToRadix {
    fn to_string_radix(&self, radix: u32) -> String;
}

impl ToRadix for i64 {
    fn to_string_radix(&self, radix: u32) -> String {
        if radix < 2 || radix > 36 {
            panic!("Radix out of range: {}", radix);
        }

        let mut n = *self;
        let mut result = String::new();

        while n != 0 {
            let digit = n % radix as i64;
            let digit_char = if digit < 10 {
                (b'0' + digit as u8) as char
            } else {
                (b'A' + (digit - 10) as u8) as char
            };

            result.insert(0, digit_char);
            n /= radix as i64;
        }

        if result.is_empty() {
            result.push('0');
        }

        result
    }
}

fn string_to_number_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let string_arg = args.pop_front().unwrap();
    let radix_arg = args.pop_front().clone();

    let radix = match radix_arg {
        Some(arg) => match arg {
            Expr::Integer(r) if [2, 8, 10, 16].contains(&r) => r as u32,
            _ => {
                return Err(runtime_error!(
                    "radix must be one of 2, 8, 10, or 16, got {}",
                    arg.kind()
                ))
            }
        },
        None => 10,
    };

    let result = match string_arg {
        Expr::String(s) => {
            let s = s.borrow();
            if let Ok(parsed) = i64::from_str_radix(&s, radix) {
                Expr::Integer(parsed)
            } else if let Ok(parsed) = s.parse::<f64>() {
                Expr::Float(parsed)
            } else {
                return Err(runtime_error!(
                    "string->number: cannot parse string as a number with the given radix"
                ));
            }
        }
        _ => {
            return Err(runtime_error!(
                "string->number is only supported for string arguments"
            ))
        }
    };

    Ok(result).map(ProcedureReturn::Value)
}

fn char_to_integer_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let char_arg = args.pop_front().unwrap();

    let unicode_code_point = match char_arg {
        Expr::Char(c) => c as i64,
        _ => {
            return Err(runtime_error!(
                "char->integer is only supported for char arguments"
            ))
        }
    };

    Ok(Expr::Integer(unicode_code_point)).map(ProcedureReturn::Value)
}

fn integer_to_char_fn(mut args: Exprs, _env: &mut EnvRef) -> ProcedureResult {
    let integer_arg = args.pop_front().unwrap();

    let character = match integer_arg {
        Expr::Integer(code_point) => match std::char::from_u32(code_point as u32) {
            Some(c) => c,
            None => {
                return Err(runtime_error!(
                    "integer->char: code point {} does not correspond to a valid Unicode character",
                    code_point
                ))
            }
        },
        _ => {
            return Err(runtime_error!(
                "integer->char is only supported for integer arguments"
            ))
        }
    };

    Ok(Expr::Char(character)).map(ProcedureReturn::Value)
}