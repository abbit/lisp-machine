use std::{cell::RefCell, rc::Rc};

use super::utils::define_procedures;
use crate::{
    evaluator::{error::runtime_error, EnvRef},
    expr::{proc_result_value, Arity, Expr, Exprs, ProcedureResult},
};

define_procedures! {
    string_set = ("string-set!", string_set_fn, Arity::Exact(3)),
    string_eq = ("string=?", string_eq_fn, Arity::Exact(2)),
    string_lt = ("string<?", string_lt_fn, Arity::Exact(2)),
    string_gt = ("string>?", string_gt_fn, Arity::Exact(2)),
    string_le = ("string<=?", string_le_fn, Arity::Exact(2)),
    string_ge = ("string>=?", string_ge_fn, Arity::Exact(2)),
    make_string = ("make-string", make_string_fn, Arity::AtLeast(1)),
    _string = ("string", string_fn, Arity::AtLeast(0)),
    string_length = ("string-length", string_length_fn, Arity::Exact(1)),
    substring = ("substring", substring_fn, Arity::Exact(3)),
    string_upcase = ("string-upcase", string_upcase_fn, Arity::Exact(1)),
    string_downcase = ("string-downcase", string_downcase_fn, Arity::Exact(1)),
    string_foldcase = ("string-foldcase", string_foldcase_fn, Arity::Exact(1)),
    string_ref = ("string-ref", string_ref_fn, Arity::Exact(2)),
    string_append = ("string-append", string_append_fn, Arity::AtLeast(0)),
    string_copy_ = ("string-copy!", string_copy_fn, Arity::AtLeast(3)),
    string_fill = ("string-fill!", string_fill_fn, Arity::AtLeast(2)),
}

fn string_set_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let string = args
        .pop_front()
        .unwrap()
        .into_string()
        .map_err(|_| runtime_error!("string-set! expected a string as its first argument"))?;
    let pos = args
        .pop_front()
        .unwrap()
        .into_integer()
        .map_err(|_| runtime_error!("string-set! expected a integer as its second argument"))?;
    let pos: usize = pos
        .try_into()
        .map_err(|_| runtime_error!("string-set! index out of bounds: {}", pos))?;
    let char =
        args.pop_front().unwrap().into_char().map_err(|_| {
            runtime_error!("string-set! expected a character as its third argument")
        })?;

    if pos >= string.borrow().len() {
        return Err(runtime_error!("string-set! index out of bounds: {}", pos));
    }

    string
        .borrow_mut()
        .replace_range(pos..(pos + 1), &char.to_string());

    proc_result_value!(Expr::Void)
}

fn string_eq_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 == str2))
}


fn string_lt_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 < str2))
}

fn string_gt_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 > str2))
}

fn string_le_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 <= str2))
}

fn string_ge_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let str1 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the first argument"))?;
    let str2 = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("Expected a string as the second argument"))?;
    proc_result_value!(Expr::Boolean(str1 >= str2))
}

fn make_string_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let k = args.pop_front().unwrap().into_integer().map_err(|_| runtime_error!("make-string expected an integer as its first argument"))?;
    
    if args.is_empty() {
        let result_string: String = std::iter::repeat('#').take(k as usize).collect();
        proc_result_value!(Expr::String(Rc::new(RefCell::new(result_string))))
    } else {
        let char_arg = args.pop_front().unwrap().into_char().map_err(|_| runtime_error!("make-string expected a character as its second argument"))?;
        let result_string: String = std::iter::repeat(char_arg).take(k as usize).collect();
        proc_result_value!(Expr::String(Rc::new(RefCell::new(result_string))))
    }
}

fn string_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let mut result_string = String::new();

    while let Some(arg) = args.pop_front() {
        let char_arg = arg.into_char().map_err(|_| runtime_error!("string expected characters as arguments"))?;
        result_string.push(char_arg);
    }

    proc_result_value!(Expr::String(Rc::new(RefCell::new(result_string))))
}

fn string_length_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let string = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("string-length expected a string as its argument"))?;
    
    let length = string.borrow().len() as i64;
    
    proc_result_value!(Expr::Integer(length))
}

fn substring_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let string = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("substring expected a string as its first argument"))?;
    let start = args.pop_front().unwrap().into_integer().map_err(|_| runtime_error!("substring expected an integer as its second argument"))?;
    let end = args.pop_front().unwrap().into_integer().map_err(|_| runtime_error!("substring expected an integer as its third argument"))?;

    let start = start as usize;
    let end = end as usize;

    if start > end || end > string.borrow().len() {
        return Err(runtime_error!("substring indices are out of bounds"));
    }

    let sub_string: String = string.borrow().chars().skip(start).take(end - start).collect();

    proc_result_value!(Expr::String(Rc::new(RefCell::new(sub_string))))
}

fn string_upcase_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let string = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("string-upcase expected a string as its argument"))?;
    let upcased_string: String = string.borrow().to_uppercase();
    proc_result_value!(Expr::String(Rc::new(RefCell::new(upcased_string))))
}

fn string_downcase_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let string = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("string-downcase expected a string as its argument"))?;
    let downcased_string: String = string.borrow().to_lowercase();
    proc_result_value!(Expr::String(Rc::new(RefCell::new(downcased_string))))
}

fn string_foldcase_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let string = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("string-foldcase expected a string as its argument"))?;
    let folded_string: String = string.borrow().to_lowercase();
    proc_result_value!(Expr::String(Rc::new(RefCell::new(folded_string))))
}

fn string_ref_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let string = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("string-ref expected a string as its first argument"))?;
    let k = args.pop_front().unwrap().into_integer().map_err(|_| runtime_error!("string-ref expected an integer as its second argument"))?;

    let k = k as usize;

    if k >= string.borrow().len() {
        return Err(runtime_error!("string-ref index out of bounds: {}", k));
    }

    let char_at_k = string.borrow().chars().nth(k).unwrap();

    proc_result_value!(Expr::Char(char_at_k))
}

fn string_append_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let mut result_string = String::new();

    while let Some(arg) = args.pop_front() {
        let string_arg = arg.into_string().map_err(|_| runtime_error!("string-append expected strings as arguments"))?;
        result_string.push_str(&string_arg.borrow());
    }

    proc_result_value!(Expr::String(Rc::new(RefCell::new(result_string))))
}

fn string_copy_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let to = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("string-copy! expected a string as its first argument"))?;
    let at = args.pop_front().unwrap().into_integer().map_err(|_| runtime_error!("string-copy! expected an integer as its second argument"))?;
    let from = args.pop_front().unwrap().into_string().map_err(|_| runtime_error!("string-copy! expected a string as its third argument"))?;

    let mut to_string = to.borrow_mut();

    let at = at as usize;

    if at >= to_string.len() {
        return Err(runtime_error!("string-copy! index out of bounds: {}", at));
    }

    let from_chars = from.borrow().chars().collect::<Vec<_>>();

    if args.is_empty() {
        if let Some(char_from) = from_chars.get(0) {
            to_string.replace_range(at..(at + 1), &char_from.to_string());
        } else {
            return Err(runtime_error!("string-copy! source string is empty"));
        }
    } else {
        let start = args.pop_front().unwrap().into_integer().map_err(|_| runtime_error!("string-copy! expected an integer as its fourth argument"))? as usize;

        if start >= from_chars.len() {
            return Err(runtime_error!("string-copy! start index out of bounds: {}", start));
        }

        if args.is_empty() {
            to_string.replace_range(at..(at + from_chars.len() - start), &from_chars[start..].iter().collect::<String>());
        } else {
            let end = args.pop_front().unwrap().into_integer().map_err(|_| runtime_error!("string-copy! expected an integer as its fifth argument"))? as usize;

            if end > from_chars.len() || end < start {
                return Err(runtime_error!("string-copy! end index out of bounds or less than start index: {}", end));
            }

            to_string.replace_range(at..(at + end - start), &from_chars[start..end].iter().collect::<String>());
        }
    }

    proc_result_value!(Expr::Void)
}

fn string_fill_fn(mut args: Exprs, _: &mut EnvRef) -> ProcedureResult {
    let string = args
        .pop_front()
        .unwrap()
        .into_string()
        .map_err(|_| runtime_error!("string-fill! expected a string as its first argument"))?;

    let fill = args
        .pop_front()
        .unwrap()
        .into_char()
        .map_err(|_| runtime_error!("string-fill! expected a character as its second argument"))?;

    let start = if !args.is_empty() {
        args.pop_front().unwrap().into_integer().map_err(|_| {
            runtime_error!("string-fill! expected an integer as its third argument")
        })? as usize
    } else {
        0
    };

    let end = if !args.is_empty() {
        args.pop_front().unwrap().into_integer().map_err(|_| {
            runtime_error!("string-fill! expected an integer as its fourth argument")
        })? as usize
    } else {
        string.borrow().len()
    };

    if fill.is_whitespace() {
        return Err(runtime_error!("string-fill! cannot fill with whitespace characters"));
    }

    let mut string_mut = string.borrow_mut();

    if start >= string_mut.len() || end > string_mut.len() || start > end {
        return Err(runtime_error!("string-fill! indices are out of bounds"));
    }

    for i in start..end {
        string_mut.replace_range(i..(i + 1), &fill.to_string());
    }

    proc_result_value!(Expr::Void)
}
