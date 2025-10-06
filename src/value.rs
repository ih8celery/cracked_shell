/// Runtime value representation for Cracked Shell
///
/// Defines the Value enum which represents all runtime values in the interpreter

use crate::error::{Error, Result};
use std::fmt;
use std::rc::Rc;

/// Built-in function type
pub type BuiltinFn = fn(&[Rc<Value>]) -> Result<Rc<Value>>;

/// Runtime value type
#[derive(Debug, Clone)]
pub enum Value {
    /// Integer value (i64)
    Integer(i64),

    /// Floating point value (f64)
    Float(f64),

    /// String value
    String(String),

    /// Boolean value
    Bool(bool),

    /// Symbol (unevaluated identifier)
    Symbol(String),

    /// List of values
    List(Vec<Rc<Value>>),

    /// Nil (empty list / null)
    Nil,

    /// Built-in function
    Builtin {
        name: String,
        func: BuiltinFn,
    },

    /// User-defined lambda (to be expanded in evaluator phase)
    Lambda {
        params: Vec<String>,
        body: String, // Placeholder - will be Expr later
    },
}

impl Value {
    /// Check if value is truthy (everything except #f is truthy)
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Bool(false))
    }

    /// Get type name as string
    pub fn type_name(&self) -> &str {
        match self {
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "boolean",
            Value::Symbol(_) => "symbol",
            Value::List(_) => "list",
            Value::Nil => "nil",
            Value::Builtin { .. } => "builtin-function",
            Value::Lambda { .. } => "lambda",
        }
    }

    /// Try to convert to integer
    pub fn as_integer(&self) -> Result<i64> {
        match self {
            Value::Integer(n) => Ok(*n),
            _ => Err(Error::type_error("integer", self.type_name())),
        }
    }

    /// Try to convert to float
    pub fn as_float(&self) -> Result<f64> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Integer(n) => Ok(*n as f64),
            _ => Err(Error::type_error("number", self.type_name())),
        }
    }

    /// Try to convert to string
    pub fn as_string(&self) -> Result<&str> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(Error::type_error("string", self.type_name())),
        }
    }

    /// Try to convert to list
    pub fn as_list(&self) -> Result<&[Rc<Value>]> {
        match self {
            Value::List(lst) => Ok(lst),
            Value::Nil => Ok(&[]),
            _ => Err(Error::type_error("list", self.type_name())),
        }
    }

    /// Check if value is a number (integer or float)
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Integer(_) | Value::Float(_))
    }

    /// Check if value is nil
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }
}

/// Implement Display for REPL output
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "()"),
            Value::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
            Value::Builtin { name, .. } => write!(f, "<builtin:{}>", name),
            Value::Lambda { params, .. } => {
                write!(f, "<lambda (")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") ...>")
            }
        }
    }
}

/// Implement PartialEq for Value (compare by value, builtins by name)
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Builtin { name: a, .. }, Value::Builtin { name: b, .. }) => a == b,
            (Value::Lambda { params: p1, body: b1 }, Value::Lambda { params: p2, body: b2 }) => {
                p1 == p2 && b1 == b2
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_types() {
        assert_eq!(Value::Integer(42).type_name(), "integer");
        assert_eq!(Value::Float(3.14).type_name(), "float");
        assert_eq!(Value::String("hello".to_string()).type_name(), "string");
        assert_eq!(Value::Bool(true).type_name(), "boolean");
        assert_eq!(Value::Symbol("x".to_string()).type_name(), "symbol");
        assert_eq!(Value::Nil.type_name(), "nil");
    }

    #[test]
    fn test_truthy() {
        assert!(Value::Integer(0).is_truthy());
        assert!(Value::String("".to_string()).is_truthy());
        assert!(Value::Nil.is_truthy());
        assert!(Value::Bool(true).is_truthy());
        assert!(!Value::Bool(false).is_truthy());
    }

    #[test]
    fn test_as_integer() {
        assert_eq!(Value::Integer(42).as_integer().unwrap(), 42);
        assert!(Value::String("x".to_string()).as_integer().is_err());
    }

    #[test]
    fn test_as_float() {
        assert_eq!(Value::Float(3.14).as_float().unwrap(), 3.14);
        assert_eq!(Value::Integer(5).as_float().unwrap(), 5.0);
        assert!(Value::String("x".to_string()).as_float().is_err());
    }

    #[test]
    fn test_display_integer() {
        assert_eq!(Value::Integer(42).to_string(), "42");
    }

    #[test]
    fn test_display_float() {
        assert_eq!(Value::Float(3.14).to_string(), "3.14");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Value::String("hello".to_string()).to_string(), "\"hello\"");
    }

    #[test]
    fn test_display_bool() {
        assert_eq!(Value::Bool(true).to_string(), "#t");
        assert_eq!(Value::Bool(false).to_string(), "#f");
    }

    #[test]
    fn test_display_nil() {
        assert_eq!(Value::Nil.to_string(), "()");
    }

    #[test]
    fn test_display_list() {
        let list = Value::List(vec![
            Rc::new(Value::Integer(1)),
            Rc::new(Value::Integer(2)),
            Rc::new(Value::Integer(3)),
        ]);
        assert_eq!(list.to_string(), "(1 2 3)");
    }

    #[test]
    fn test_display_symbol() {
        assert_eq!(Value::Symbol("foo".to_string()).to_string(), "foo");
    }

    #[test]
    fn test_is_number() {
        assert!(Value::Integer(42).is_number());
        assert!(Value::Float(3.14).is_number());
        assert!(!Value::String("x".to_string()).is_number());
    }

    #[test]
    fn test_is_nil() {
        assert!(Value::Nil.is_nil());
        assert!(!Value::Integer(0).is_nil());
        assert!(!Value::List(vec![]).is_nil());
    }
}
