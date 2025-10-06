/// Cracked Shell Library
///
/// This library implements a Lisp-based shell with Unix command integration.
/// See docs/architecture.md for design details and docs/lisp-reference.md
/// for language specification.

#![warn(missing_docs)]
#![warn(clippy::all)]

// Module declarations - to be implemented in Phase 2+
// pub mod repl;
// pub mod lexer;
// pub mod parser;
// pub mod eval;
// pub mod env;
// pub mod macro_expand;
// pub mod builtin;
// pub mod process;
// pub mod stream;
// pub mod value;
// pub mod error;

/// Placeholder module for early testing
pub mod placeholder {
    /// Returns the shell name
    pub fn shell_name() -> &'static str {
        "Cracked Shell"
    }

    /// Returns the shell version
    pub fn shell_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[cfg(test)]
mod tests {
    use super::placeholder::*;

    #[test]
    fn test_shell_name() {
        assert_eq!(shell_name(), "Cracked Shell");
    }

    #[test]
    fn test_shell_version() {
        assert_eq!(shell_version(), "0.1.0");
    }
}
