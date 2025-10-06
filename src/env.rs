/// Environment and Symbol Resolution for Cracked Shell
///
/// Implements lexical scope with parent chain, supporting define, lookup, and shadowing

use crate::error::{Error, Result};
use crate::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

/// Environment for variable bindings
#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, Rc<Value>>,
    parent: Option<Rc<Environment>>,
}

impl Environment {
    /// Create a new empty environment with no parent
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    /// Create a new environment with a parent scope
    pub fn with_parent(parent: Rc<Environment>) -> Self {
        Environment {
            bindings: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Define a variable in the current scope (no parent lookup)
    pub fn define(&mut self, name: impl Into<String>, value: Rc<Value>) {
        self.bindings.insert(name.into(), value);
    }

    /// Set a variable (updates existing binding or creates new one in current scope)
    /// This is different from define - it searches parent scopes first
    pub fn set(&mut self, name: impl Into<String>, value: Rc<Value>) -> Result<()> {
        let name = name.into();

        // Check if variable exists in current or parent scopes
        if self.get(&name).is_ok() {
            // If it exists, update in the scope where it's defined
            if self.bindings.contains_key(&name) {
                self.bindings.insert(name, value);
            } else {
                // It's in a parent scope - we can't mutate parent through Rc
                // For now, just shadow it in current scope
                self.bindings.insert(name, value);
            }
            Ok(())
        } else {
            // Variable doesn't exist, define it in current scope
            self.bindings.insert(name, value);
            Ok(())
        }
    }

    /// Get a variable's value, searching parent scopes if needed
    pub fn get(&self, name: &str) -> Result<Rc<Value>> {
        // First check current scope
        if let Some(value) = self.bindings.get(name) {
            return Ok(Rc::clone(value));
        }

        // Then check parent scopes
        if let Some(parent) = &self.parent {
            return parent.get(name);
        }

        // Not found in any scope
        Err(Error::UndefinedSymbol(name.to_string()))
    }

    /// Check if a variable is defined in current or parent scopes
    pub fn contains(&self, name: &str) -> bool {
        self.bindings.contains_key(name) || self.parent.as_ref().map_or(false, |p| p.contains(name))
    }

    /// Get the parent environment
    pub fn parent(&self) -> Option<Rc<Environment>> {
        self.parent.as_ref().map(Rc::clone)
    }

    /// Create a child environment (for let bindings, function calls, etc.)
    pub fn child(self: &Rc<Self>) -> Environment {
        Environment::with_parent(Rc::clone(self))
    }

    /// Get all bindings in current scope (for debugging)
    pub fn bindings(&self) -> &HashMap<String, Rc<Value>> {
        &self.bindings
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_get() {
        let mut env = Environment::new();
        let value = Rc::new(Value::Integer(42));

        env.define("x", Rc::clone(&value));

        let retrieved = env.get("x").unwrap();
        assert_eq!(*retrieved, Value::Integer(42));
    }

    #[test]
    fn test_undefined_symbol() {
        let env = Environment::new();

        let result = env.get("undefined");
        assert!(result.is_err());

        match result {
            Err(Error::UndefinedSymbol(name)) => assert_eq!(name, "undefined"),
            _ => panic!("Expected UndefinedSymbol error"),
        }
    }

    #[test]
    fn test_parent_scope_lookup() {
        let mut parent = Environment::new();
        parent.define("x", Rc::new(Value::Integer(10)));

        let parent_rc = Rc::new(parent);
        let child = Environment::with_parent(Rc::clone(&parent_rc));

        let value = child.get("x").unwrap();
        assert_eq!(*value, Value::Integer(10));
    }

    #[test]
    fn test_shadowing() {
        let mut parent = Environment::new();
        parent.define("x", Rc::new(Value::Integer(10)));

        let parent_rc = Rc::new(parent);
        let mut child = Environment::with_parent(Rc::clone(&parent_rc));

        // Shadow x in child scope
        child.define("x", Rc::new(Value::Integer(20)));

        // Child sees shadowed value
        let child_value = child.get("x").unwrap();
        assert_eq!(*child_value, Value::Integer(20));

        // Parent still has original value
        let parent_value = parent_rc.get("x").unwrap();
        assert_eq!(*parent_value, Value::Integer(10));
    }

    #[test]
    fn test_nested_scopes() {
        let mut global = Environment::new();
        global.define("a", Rc::new(Value::Integer(1)));

        let global_rc = Rc::new(global);
        let mut middle = Environment::with_parent(Rc::clone(&global_rc));
        middle.define("b", Rc::new(Value::Integer(2)));

        let middle_rc = Rc::new(middle);
        let mut inner = Environment::with_parent(Rc::clone(&middle_rc));
        inner.define("c", Rc::new(Value::Integer(3)));

        // Inner can see all scopes
        assert_eq!(*inner.get("a").unwrap(), Value::Integer(1));
        assert_eq!(*inner.get("b").unwrap(), Value::Integer(2));
        assert_eq!(*inner.get("c").unwrap(), Value::Integer(3));

        // Middle can't see inner
        assert!(middle_rc.get("c").is_err());
    }

    #[test]
    fn test_contains() {
        let mut parent = Environment::new();
        parent.define("x", Rc::new(Value::Integer(10)));

        let parent_rc = Rc::new(parent);
        let mut child = Environment::with_parent(Rc::clone(&parent_rc));
        child.define("y", Rc::new(Value::Integer(20)));

        assert!(child.contains("x")); // From parent
        assert!(child.contains("y")); // From current
        assert!(!child.contains("z")); // Not defined
    }

    #[test]
    fn test_child_creation() {
        let mut parent = Environment::new();
        parent.define("x", Rc::new(Value::Integer(42)));

        let parent_rc = Rc::new(parent);
        let child = parent_rc.child();

        assert_eq!(*child.get("x").unwrap(), Value::Integer(42));
    }

    #[test]
    fn test_set() {
        let mut env = Environment::new();

        // Set creates new binding if doesn't exist
        env.set("x", Rc::new(Value::Integer(10))).unwrap();
        assert_eq!(*env.get("x").unwrap(), Value::Integer(10));

        // Set updates existing binding
        env.set("x", Rc::new(Value::Integer(20))).unwrap();
        assert_eq!(*env.get("x").unwrap(), Value::Integer(20));
    }

    #[test]
    fn test_multiple_types() {
        let mut env = Environment::new();

        env.define("int", Rc::new(Value::Integer(42)));
        env.define("float", Rc::new(Value::Float(3.14)));
        env.define("string", Rc::new(Value::String("hello".to_string())));
        env.define("bool", Rc::new(Value::Bool(true)));
        env.define("nil", Rc::new(Value::Nil));

        assert!(matches!(*env.get("int").unwrap(), Value::Integer(42)));
        assert!(matches!(*env.get("float").unwrap(), Value::Float(_)));
        assert!(matches!(*env.get("string").unwrap(), Value::String(_)));
        assert!(matches!(*env.get("bool").unwrap(), Value::Bool(true)));
        assert!(matches!(*env.get("nil").unwrap(), Value::Nil));
    }

    #[test]
    fn test_bindings_access() {
        let mut env = Environment::new();
        env.define("x", Rc::new(Value::Integer(1)));
        env.define("y", Rc::new(Value::Integer(2)));

        let bindings = env.bindings();
        assert_eq!(bindings.len(), 2);
        assert!(bindings.contains_key("x"));
        assert!(bindings.contains_key("y"));
    }
}
