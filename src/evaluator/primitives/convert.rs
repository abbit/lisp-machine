use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{Arity, Expr, Exprs, ProcedureResult, ProcedureReturn},
};

define_procedures! {
    number_to_string = ("number->string", number_to_string_fn, Arity::AtLeast(1)),
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

    let result = match (number, radix) {
        (Expr::Integer(n), r) => {
            let string_representation = format!("{}", n.to_string_radix(r));
            Expr::String(string_representation)
        }
        (Expr::Float(f), 10) => {
            let string_representation = format!("{}", f);
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