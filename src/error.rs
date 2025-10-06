/// Error types for Cracked Shell
///
/// Provides structured error handling throughout the interpreter

use std::fmt;
use thiserror::Error;

/// Source location for error reporting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Main error type for Cracked Shell
#[derive(Debug, Error)]
pub enum Error {
    /// Parse error with location and message
    #[error("Parse error at {location}: {message}")]
    ParseError {
        location: SourceLocation,
        message: String,
    },

    /// Undefined symbol error
    #[error("Undefined symbol: {0}")]
    UndefinedSymbol(String),

    /// Type error
    #[error("Type error: expected {expected}, got {actual}")]
    TypeError { expected: String, actual: String },

    /// Arity error (wrong number of arguments)
    #[error("Arity error: {func} expects {expected} args, got {actual}")]
    ArityError {
        func: String,
        expected: usize,
        actual: usize,
    },

    /// Division by zero
    #[error("Division by zero")]
    DivisionByZero,

    /// Runtime error
    #[error("Runtime error: {0}")]
    RuntimeError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type alias for Cracked Shell operations
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a parse error
    pub fn parse_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        Error::ParseError {
            location: SourceLocation { line, column },
            message: message.into(),
        }
    }

    /// Create a type error
    pub fn type_error(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Error::TypeError {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Create an arity error
    pub fn arity_error(func: impl Into<String>, expected: usize, actual: usize) -> Self {
        Error::ArityError {
            func: func.into(),
            expected,
            actual,
        }
    }

    /// Create a runtime error
    pub fn runtime(message: impl Into<String>) -> Self {
        Error::RuntimeError(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_location_display() {
        let loc = SourceLocation { line: 5, column: 10 };
        assert_eq!(loc.to_string(), "line 5, column 10");
    }

    #[test]
    fn test_parse_error() {
        let err = Error::parse_error(1, 5, "unexpected token");
        assert!(err.to_string().contains("line 1"));
        assert!(err.to_string().contains("column 5"));
        assert!(err.to_string().contains("unexpected token"));
    }

    #[test]
    fn test_undefined_symbol() {
        let err = Error::UndefinedSymbol("foo".to_string());
        assert_eq!(err.to_string(), "Undefined symbol: foo");
    }

    #[test]
    fn test_type_error() {
        let err = Error::type_error("integer", "string");
        assert!(err.to_string().contains("expected integer"));
        assert!(err.to_string().contains("got string"));
    }

    #[test]
    fn test_arity_error() {
        let err = Error::arity_error("+", 2, 1);
        assert!(err.to_string().contains("+ expects 2 args"));
        assert!(err.to_string().contains("got 1"));
    }
}
