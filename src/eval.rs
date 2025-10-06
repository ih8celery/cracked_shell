/// Evaluator for Cracked Shell
///
/// Implements eval function for atoms, lists, and special forms (if, define, lambda, let, quote)

use crate::builtin::*;
use crate::env::Environment;
use crate::error::{Error, Result};
use crate::value::{BuiltinFn, Value};
use std::rc::Rc;

/// Evaluator context
pub struct Evaluator {
    global_env: Rc<Environment>,
}

impl Evaluator {
    /// Create a new evaluator with standard builtins
    pub fn new() -> Self {
        let mut env = Environment::new();

        // Register built-in functions
        env.define("+", Rc::new(Value::Builtin { name: "+".to_string(), func: builtin_add as BuiltinFn }));
        env.define("-", Rc::new(Value::Builtin { name: "-".to_string(), func: builtin_sub as BuiltinFn }));
        env.define("*", Rc::new(Value::Builtin { name: "*".to_string(), func: builtin_mul as BuiltinFn }));
        env.define("/", Rc::new(Value::Builtin { name: "/".to_string(), func: builtin_div as BuiltinFn }));
        env.define("<", Rc::new(Value::Builtin { name: "<".to_string(), func: builtin_lt as BuiltinFn }));
        env.define(">", Rc::new(Value::Builtin { name: ">".to_string(), func: builtin_gt as BuiltinFn }));
        env.define("=", Rc::new(Value::Builtin { name: "=".to_string(), func: builtin_eq as BuiltinFn }));
        env.define("car", Rc::new(Value::Builtin { name: "car".to_string(), func: builtin_car as BuiltinFn }));
        env.define("cdr", Rc::new(Value::Builtin { name: "cdr".to_string(), func: builtin_cdr as BuiltinFn }));
        env.define("cons", Rc::new(Value::Builtin { name: "cons".to_string(), func: builtin_cons as BuiltinFn }));
        env.define("list", Rc::new(Value::Builtin { name: "list".to_string(), func: builtin_list as BuiltinFn }));
        env.define("length", Rc::new(Value::Builtin { name: "length".to_string(), func: builtin_length as BuiltinFn }));
        env.define("null?", Rc::new(Value::Builtin { name: "null?".to_string(), func: builtin_null as BuiltinFn }));

        Evaluator {
            global_env: Rc::new(env),
        }
    }

    /// Evaluate an expression in the global environment
    pub fn eval(&self, expr: Rc<Value>) -> Result<Rc<Value>> {
        self.eval_in_env(expr, &self.global_env)
    }

    /// Evaluate an expression in a specific environment
    pub fn eval_in_env(&self, expr: Rc<Value>, env: &Rc<Environment>) -> Result<Rc<Value>> {
        match &*expr {
            // Self-evaluating values
            Value::Integer(_) | Value::Float(_) | Value::String(_) | Value::Bool(_) | Value::Nil => {
                Ok(expr)
            }

            // Symbol lookup
            Value::Symbol(name) => env.get(name),

            // List evaluation (function application or special form)
            Value::List(items) if !items.is_empty() => {
                // Check for special forms
                if let Value::Symbol(s) = &*items[0] {
                    match s.as_str() {
                        "quote" => return self.eval_quote(&items[1..]),
                        "if" => return self.eval_if(&items[1..], env),
                        "define" => return self.eval_define(&items[1..], env),
                        "lambda" => return self.eval_lambda(&items[1..]),
                        "let" => return self.eval_let(&items[1..], env),
                        _ => {}
                    }
                }

                // Normal function application
                self.eval_application(items, env)
            }

            // Empty list evaluates to nil
            Value::List(_) => Ok(Rc::new(Value::Nil)),

            // Functions, builtins, and lambdas are self-evaluating
            Value::Builtin { .. } | Value::Lambda { .. } => Ok(expr),
        }
    }

    /// Evaluate quote special form: (quote expr)
    fn eval_quote(&self, args: &[Rc<Value>]) -> Result<Rc<Value>> {
        if args.len() != 1 {
            return Err(Error::arity_error("quote", 1, args.len()));
        }
        Ok(Rc::clone(&args[0]))
    }

    /// Evaluate if special form: (if condition then else?)
    fn eval_if(&self, args: &[Rc<Value>], env: &Rc<Environment>) -> Result<Rc<Value>> {
        if args.len() < 2 || args.len() > 3 {
            return Err(Error::arity_error("if", 3, args.len()));
        }

        let condition = self.eval_in_env(Rc::clone(&args[0]), env)?;

        if condition.is_truthy() {
            self.eval_in_env(Rc::clone(&args[1]), env)
        } else if args.len() == 3 {
            self.eval_in_env(Rc::clone(&args[2]), env)
        } else {
            Ok(Rc::new(Value::Nil))
        }
    }

    /// Evaluate define special form: (define name value)
    fn eval_define(&self, args: &[Rc<Value>], env: &Rc<Environment>) -> Result<Rc<Value>> {
        if args.len() != 2 {
            return Err(Error::arity_error("define", 2, args.len()));
        }

        let name = match &*args[0] {
            Value::Symbol(s) => s.clone(),
            _ => return Err(Error::type_error("symbol", args[0].type_name())),
        };

        let value = self.eval_in_env(Rc::clone(&args[1]), env)?;

        // We need to mutate the environment, but env is Rc<Environment>
        // For now, we'll just return an error - this needs to be addressed
        // with a RefCell or similar interior mutability pattern
        Err(Error::runtime(
            "define is not yet supported in this evaluator (requires mutable environment)",
        ))
    }

    /// Evaluate lambda special form: (lambda (params...) body)
    fn eval_lambda(&self, args: &[Rc<Value>]) -> Result<Rc<Value>> {
        if args.len() != 2 {
            return Err(Error::arity_error("lambda", 2, args.len()));
        }

        // Extract parameter names
        let params = match &*args[0] {
            Value::List(items) => {
                let mut param_names = Vec::new();
                for item in items {
                    match &**item {
                        Value::Symbol(s) => param_names.push(s.clone()),
                        _ => return Err(Error::type_error("symbol", item.type_name())),
                    }
                }
                param_names
            }
            Value::Nil => Vec::new(),
            _ => return Err(Error::type_error("list", args[0].type_name())),
        };

        // For now, we'll store the body as a string representation
        // In a complete implementation, we'd store the actual expression
        let body = format!("{}", args[1]);

        Ok(Rc::new(Value::Lambda { params, body }))
    }

    /// Evaluate let special form: (let ((name value)...) body)
    fn eval_let(&self, args: &[Rc<Value>], env: &Rc<Environment>) -> Result<Rc<Value>> {
        if args.len() != 2 {
            return Err(Error::arity_error("let", 2, args.len()));
        }

        // Create new child environment
        let mut child_env = env.child();

        // Process bindings
        let bindings = match &*args[0] {
            Value::List(items) => items,
            _ => return Err(Error::type_error("list", args[0].type_name())),
        };

        for binding in bindings {
            match &**binding {
                Value::List(pair) if pair.len() == 2 => {
                    let name = match &*pair[0] {
                        Value::Symbol(s) => s.clone(),
                        _ => return Err(Error::type_error("symbol", pair[0].type_name())),
                    };

                    let value = self.eval_in_env(Rc::clone(&pair[1]), env)?;
                    child_env.define(name, value);
                }
                _ => {
                    return Err(Error::runtime(
                        "let binding must be a list of (name value) pairs",
                    ))
                }
            }
        }

        // Evaluate body in child environment
        let child_env_rc = Rc::new(child_env);
        self.eval_in_env(Rc::clone(&args[1]), &child_env_rc)
    }

    /// Evaluate function application
    fn eval_application(&self, items: &[Rc<Value>], env: &Rc<Environment>) -> Result<Rc<Value>> {
        // Evaluate the function
        let func = self.eval_in_env(Rc::clone(&items[0]), env)?;

        // Evaluate the arguments
        let mut args = Vec::new();
        for arg in &items[1..] {
            args.push(self.eval_in_env(Rc::clone(arg), env)?);
        }

        // Apply the function
        match &*func {
            Value::Builtin { func, .. } => func(&args),
            Value::Lambda { .. } => Err(Error::runtime(
                "Lambda application not yet implemented (requires closure support)",
            )),
            _ => Err(Error::type_error("function", func.type_name())),
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn eval_str(input: &str) -> Result<Rc<Value>> {
        let tokens = Lexer::tokenize(input)?;
        let expr = Parser::parse(tokens)?;
        let evaluator = Evaluator::new();
        evaluator.eval(expr)
    }

    #[test]
    fn test_eval_integer() {
        let result = eval_str("42").unwrap();
        assert_eq!(*result, Value::Integer(42));
    }

    #[test]
    fn test_eval_string() {
        let result = eval_str(r#""hello""#).unwrap();
        assert_eq!(*result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_eval_add() {
        let result = eval_str("(+ 1 2)").unwrap();
        assert_eq!(*result, Value::Integer(3));
    }

    #[test]
    fn test_eval_nested_arithmetic() {
        let result = eval_str("(+ (* 2 3) (- 10 5))").unwrap();
        assert_eq!(*result, Value::Integer(11));
    }

    #[test]
    fn test_eval_comparison() {
        let result = eval_str("(< 1 2)").unwrap();
        assert_eq!(*result, Value::Bool(true));

        let result = eval_str("(> 1 2)").unwrap();
        assert_eq!(*result, Value::Bool(false));
    }

    #[test]
    fn test_eval_quote() {
        let result = eval_str("'x").unwrap();
        assert_eq!(*result, Value::Symbol("x".to_string()));

        let result = eval_str("'(1 2 3)").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(*items[0], Value::Integer(1));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_eval_if_true() {
        let result = eval_str("(if #t 1 2)").unwrap();
        assert_eq!(*result, Value::Integer(1));
    }

    #[test]
    fn test_eval_if_false() {
        let result = eval_str("(if #f 1 2)").unwrap();
        assert_eq!(*result, Value::Integer(2));
    }

    #[test]
    fn test_eval_if_no_else() {
        let result = eval_str("(if #f 1)").unwrap();
        assert_eq!(*result, Value::Nil);
    }

    #[test]
    fn test_eval_if_with_condition() {
        let result = eval_str("(if (< 1 2) 10 20)").unwrap();
        assert_eq!(*result, Value::Integer(10));
    }

    #[test]
    fn test_eval_let() {
        let result = eval_str("(let ((x 1) (y 2)) (+ x y))").unwrap();
        assert_eq!(*result, Value::Integer(3));
    }

    #[test]
    fn test_eval_let_shadowing() {
        // This would require defining x in global scope first
        // For now, just test basic let
        let result = eval_str("(let ((x 10)) (let ((x 20)) x))").unwrap();
        assert_eq!(*result, Value::Integer(20));
    }

    #[test]
    fn test_eval_lambda_creation() {
        let result = eval_str("(lambda (x) x)").unwrap();
        match &*result {
            Value::Lambda { params, .. } => {
                assert_eq!(params.len(), 1);
                assert_eq!(params[0], "x");
            }
            _ => panic!("Expected lambda"),
        }
    }

    #[test]
    fn test_eval_list_ops() {
        let result = eval_str("(car '(1 2 3))").unwrap();
        assert_eq!(*result, Value::Integer(1));

        let result = eval_str("(cdr '(1 2 3))").unwrap();
        match &*result {
            Value::List(items) => assert_eq!(items.len(), 2),
            _ => panic!("Expected list"),
        }

        let result = eval_str("(cons 1 '(2 3))").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(*items[0], Value::Integer(1));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_eval_empty_list() {
        let result = eval_str("()").unwrap();
        assert_eq!(*result, Value::Nil);
    }

    #[test]
    fn test_undefined_symbol() {
        let result = eval_str("undefined");
        assert!(result.is_err());
    }
}
