/// Built-in Primitive Functions for Cracked Shell
///
/// Implements arithmetic, comparison, and list operations

use crate::error::{Error, Result};
use crate::value::Value;
use std::rc::Rc;

/// Addition: (+ a b ...)
pub fn builtin_add(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.is_empty() {
        return Ok(Rc::new(Value::Integer(0)));
    }

    let mut has_float = false;
    let mut int_sum: i64 = 0;
    let mut float_sum: f64 = 0.0;

    for arg in args {
        match **arg {
            Value::Integer(n) => {
                if has_float {
                    float_sum += n as f64;
                } else {
                    int_sum += n;
                }
            }
            Value::Float(f) => {
                if !has_float {
                    has_float = true;
                    float_sum = int_sum as f64;
                }
                float_sum += f;
            }
            _ => return Err(Error::type_error("number", arg.type_name())),
        }
    }

    if has_float {
        Ok(Rc::new(Value::Float(float_sum)))
    } else {
        Ok(Rc::new(Value::Integer(int_sum)))
    }
}

/// Subtraction: (- a b ...)
pub fn builtin_sub(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.is_empty() {
        return Err(Error::arity_error("-", 1, 0));
    }

    if args.len() == 1 {
        // Unary negation
        return match **args.first().unwrap() {
            Value::Integer(n) => Ok(Rc::new(Value::Integer(-n))),
            Value::Float(f) => Ok(Rc::new(Value::Float(-f))),
            _ => Err(Error::type_error("number", args[0].type_name())),
        };
    }

    let first = &args[0];
    let mut has_float = matches!(**first, Value::Float(_));
    let mut result = match **first {
        Value::Integer(n) => n as f64,
        Value::Float(f) => f,
        _ => return Err(Error::type_error("number", first.type_name())),
    };

    for arg in &args[1..] {
        match **arg {
            Value::Integer(n) => result -= n as f64,
            Value::Float(f) => {
                has_float = true;
                result -= f;
            }
            _ => return Err(Error::type_error("number", arg.type_name())),
        }
    }

    if has_float {
        Ok(Rc::new(Value::Float(result)))
    } else {
        Ok(Rc::new(Value::Integer(result as i64)))
    }
}

/// Multiplication: (* a b ...)
pub fn builtin_mul(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.is_empty() {
        return Ok(Rc::new(Value::Integer(1)));
    }

    let mut has_float = false;
    let mut int_prod: i64 = 1;
    let mut float_prod: f64 = 1.0;

    for arg in args {
        match **arg {
            Value::Integer(n) => {
                if has_float {
                    float_prod *= n as f64;
                } else {
                    int_prod *= n;
                }
            }
            Value::Float(f) => {
                if !has_float {
                    has_float = true;
                    float_prod = int_prod as f64;
                }
                float_prod *= f;
            }
            _ => return Err(Error::type_error("number", arg.type_name())),
        }
    }

    if has_float {
        Ok(Rc::new(Value::Float(float_prod)))
    } else {
        Ok(Rc::new(Value::Integer(int_prod)))
    }
}

/// Division: (/ a b ...)
pub fn builtin_div(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.len() < 2 {
        return Err(Error::arity_error("/", 2, args.len()));
    }

    let first = args[0].as_float()?;
    let mut result = first;

    for arg in &args[1..] {
        let divisor = arg.as_float()?;
        if divisor == 0.0 {
            return Err(Error::DivisionByZero);
        }
        result /= divisor;
    }

    Ok(Rc::new(Value::Float(result)))
}

/// Less than: (< a b)
pub fn builtin_lt(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.len() != 2 {
        return Err(Error::arity_error("<", 2, args.len()));
    }

    let a = args[0].as_float()?;
    let b = args[1].as_float()?;

    Ok(Rc::new(Value::Bool(a < b)))
}

/// Greater than: (> a b)
pub fn builtin_gt(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.len() != 2 {
        return Err(Error::arity_error(">", 2, args.len()));
    }

    let a = args[0].as_float()?;
    let b = args[1].as_float()?;

    Ok(Rc::new(Value::Bool(a > b)))
}

/// Equal: (= a b)
pub fn builtin_eq(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.len() != 2 {
        return Err(Error::arity_error("=", 2, args.len()));
    }

    Ok(Rc::new(Value::Bool(*args[0] == *args[1])))
}

/// Car (first element): (car list)
pub fn builtin_car(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.len() != 1 {
        return Err(Error::arity_error("car", 1, args.len()));
    }

    let list = args[0].as_list()?;
    if list.is_empty() {
        return Err(Error::runtime("car: cannot take car of empty list"));
    }

    Ok(Rc::clone(&list[0]))
}

/// Cdr (rest of list): (cdr list)
pub fn builtin_cdr(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.len() != 1 {
        return Err(Error::arity_error("cdr", 1, args.len()));
    }

    let list = args[0].as_list()?;
    if list.is_empty() {
        return Err(Error::runtime("cdr: cannot take cdr of empty list"));
    }

    Ok(Rc::new(Value::List(list[1..].to_vec())))
}

/// Cons (construct list): (cons element list)
pub fn builtin_cons(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.len() != 2 {
        return Err(Error::arity_error("cons", 2, args.len()));
    }

    let elem = Rc::clone(&args[0]);
    let list = args[1].as_list()?;

    let mut new_list = Vec::with_capacity(list.len() + 1);
    new_list.push(elem);
    new_list.extend_from_slice(list);

    Ok(Rc::new(Value::List(new_list)))
}

/// List constructor: (list a b c ...)
pub fn builtin_list(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    Ok(Rc::new(Value::List(args.to_vec())))
}

/// Length of list: (length list)
pub fn builtin_length(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.len() != 1 {
        return Err(Error::arity_error("length", 1, args.len()));
    }

    let list = args[0].as_list()?;
    Ok(Rc::new(Value::Integer(list.len() as i64)))
}

/// Null predicate: (null? value)
pub fn builtin_null(args: &[Rc<Value>]) -> Result<Rc<Value>> {
    if args.len() != 1 {
        return Err(Error::arity_error("null?", 1, args.len()));
    }

    Ok(Rc::new(Value::Bool(args[0].is_nil() || matches!(args[0].as_list(), Ok(list) if list.is_empty()))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let args = vec![Rc::new(Value::Integer(1)), Rc::new(Value::Integer(2))];
        let result = builtin_add(&args).unwrap();
        assert_eq!(*result, Value::Integer(3));
    }

    #[test]
    fn test_add_multiple() {
        let args = vec![
            Rc::new(Value::Integer(1)),
            Rc::new(Value::Integer(2)),
            Rc::new(Value::Integer(3)),
        ];
        let result = builtin_add(&args).unwrap();
        assert_eq!(*result, Value::Integer(6));
    }

    #[test]
    fn test_add_float() {
        let args = vec![Rc::new(Value::Integer(1)), Rc::new(Value::Float(2.5))];
        let result = builtin_add(&args).unwrap();
        assert_eq!(*result, Value::Float(3.5));
    }

    #[test]
    fn test_add_empty() {
        let result = builtin_add(&[]).unwrap();
        assert_eq!(*result, Value::Integer(0));
    }

    #[test]
    fn test_sub() {
        let args = vec![Rc::new(Value::Integer(5)), Rc::new(Value::Integer(3))];
        let result = builtin_sub(&args).unwrap();
        assert_eq!(*result, Value::Integer(2));
    }

    #[test]
    fn test_sub_unary() {
        let args = vec![Rc::new(Value::Integer(5))];
        let result = builtin_sub(&args).unwrap();
        assert_eq!(*result, Value::Integer(-5));
    }

    #[test]
    fn test_mul() {
        let args = vec![Rc::new(Value::Integer(3)), Rc::new(Value::Integer(4))];
        let result = builtin_mul(&args).unwrap();
        assert_eq!(*result, Value::Integer(12));
    }

    #[test]
    fn test_mul_empty() {
        let result = builtin_mul(&[]).unwrap();
        assert_eq!(*result, Value::Integer(1));
    }

    #[test]
    fn test_div() {
        let args = vec![Rc::new(Value::Integer(10)), Rc::new(Value::Integer(2))];
        let result = builtin_div(&args).unwrap();
        assert_eq!(*result, Value::Float(5.0));
    }

    #[test]
    fn test_div_by_zero() {
        let args = vec![Rc::new(Value::Integer(10)), Rc::new(Value::Integer(0))];
        let result = builtin_div(&args);
        assert!(matches!(result, Err(Error::DivisionByZero)));
    }

    #[test]
    fn test_lt() {
        let args = vec![Rc::new(Value::Integer(1)), Rc::new(Value::Integer(2))];
        let result = builtin_lt(&args).unwrap();
        assert_eq!(*result, Value::Bool(true));

        let args = vec![Rc::new(Value::Integer(2)), Rc::new(Value::Integer(1))];
        let result = builtin_lt(&args).unwrap();
        assert_eq!(*result, Value::Bool(false));
    }

    #[test]
    fn test_gt() {
        let args = vec![Rc::new(Value::Integer(2)), Rc::new(Value::Integer(1))];
        let result = builtin_gt(&args).unwrap();
        assert_eq!(*result, Value::Bool(true));
    }

    #[test]
    fn test_eq() {
        let args = vec![Rc::new(Value::Integer(2)), Rc::new(Value::Integer(2))];
        let result = builtin_eq(&args).unwrap();
        assert_eq!(*result, Value::Bool(true));

        let args = vec![Rc::new(Value::Integer(2)), Rc::new(Value::Integer(3))];
        let result = builtin_eq(&args).unwrap();
        assert_eq!(*result, Value::Bool(false));
    }

    #[test]
    fn test_car() {
        let list = vec![
            Rc::new(Value::Integer(1)),
            Rc::new(Value::Integer(2)),
            Rc::new(Value::Integer(3)),
        ];
        let args = vec![Rc::new(Value::List(list))];
        let result = builtin_car(&args).unwrap();
        assert_eq!(*result, Value::Integer(1));
    }

    #[test]
    fn test_car_empty_error() {
        let args = vec![Rc::new(Value::List(vec![]))];
        let result = builtin_car(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_cdr() {
        let list = vec![
            Rc::new(Value::Integer(1)),
            Rc::new(Value::Integer(2)),
            Rc::new(Value::Integer(3)),
        ];
        let args = vec![Rc::new(Value::List(list))];
        let result = builtin_cdr(&args).unwrap();

        match &*result {
            Value::List(lst) => {
                assert_eq!(lst.len(), 2);
                assert_eq!(*lst[0], Value::Integer(2));
                assert_eq!(*lst[1], Value::Integer(3));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_cons() {
        let list = vec![Rc::new(Value::Integer(2)), Rc::new(Value::Integer(3))];
        let args = vec![Rc::new(Value::Integer(1)), Rc::new(Value::List(list))];
        let result = builtin_cons(&args).unwrap();

        match &*result {
            Value::List(lst) => {
                assert_eq!(lst.len(), 3);
                assert_eq!(*lst[0], Value::Integer(1));
                assert_eq!(*lst[1], Value::Integer(2));
                assert_eq!(*lst[2], Value::Integer(3));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_list() {
        let args = vec![
            Rc::new(Value::Integer(1)),
            Rc::new(Value::Integer(2)),
            Rc::new(Value::Integer(3)),
        ];
        let result = builtin_list(&args).unwrap();

        match &*result {
            Value::List(lst) => {
                assert_eq!(lst.len(), 3);
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_length() {
        let list = vec![
            Rc::new(Value::Integer(1)),
            Rc::new(Value::Integer(2)),
            Rc::new(Value::Integer(3)),
        ];
        let args = vec![Rc::new(Value::List(list))];
        let result = builtin_length(&args).unwrap();
        assert_eq!(*result, Value::Integer(3));
    }

    #[test]
    fn test_null() {
        let args = vec![Rc::new(Value::Nil)];
        let result = builtin_null(&args).unwrap();
        assert_eq!(*result, Value::Bool(true));

        let args = vec![Rc::new(Value::List(vec![]))];
        let result = builtin_null(&args).unwrap();
        assert_eq!(*result, Value::Bool(true));

        let args = vec![Rc::new(Value::Integer(1))];
        let result = builtin_null(&args).unwrap();
        assert_eq!(*result, Value::Bool(false));
    }
}
