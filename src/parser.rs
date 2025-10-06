/// S-Expression Parser for Cracked Shell
///
/// Builds AST from token stream, supports atoms, lists, and quote sugar

use crate::error::{Error, Result};
use crate::lexer::{LocatedToken, Token};
use crate::value::Value;
use std::rc::Rc;

/// Parser state
pub struct Parser {
    tokens: Vec<LocatedToken>,
    pos: usize,
}

impl Parser {
    /// Create a new parser from tokens
    pub fn new(tokens: Vec<LocatedToken>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Parse tokens into a single expression
    pub fn parse(tokens: Vec<LocatedToken>) -> Result<Rc<Value>> {
        let mut parser = Parser::new(tokens);
        parser.parse_expr()
    }

    /// Parse tokens into multiple expressions (for a file or REPL input)
    pub fn parse_all(tokens: Vec<LocatedToken>) -> Result<Vec<Rc<Value>>> {
        let mut parser = Parser::new(tokens);
        let mut exprs = Vec::new();

        while !parser.is_eof() {
            exprs.push(parser.parse_expr()?);
        }

        Ok(exprs)
    }

    /// Parse a single expression
    fn parse_expr(&mut self) -> Result<Rc<Value>> {
        if self.is_eof() {
            return Err(Error::runtime("Unexpected end of input"));
        }

        let token = self.peek().clone();
        match token.token {
            Token::LParen => self.parse_list(),
            Token::RParen => Err(Error::parse_error(
                token.location.line,
                token.location.column,
                "Unexpected ')'",
            )),
            Token::Quote => self.parse_quote(),
            Token::Quasiquote => self.parse_quasiquote(),
            Token::Unquote => Err(Error::parse_error(
                token.location.line,
                token.location.column,
                "Unquote outside of quasiquote",
            )),
            Token::UnquoteSplicing => Err(Error::parse_error(
                token.location.line,
                token.location.column,
                "Unquote-splicing outside of quasiquote",
            )),
            Token::Symbol(s) => {
                self.advance();
                Ok(Rc::new(Value::Symbol(s)))
            }
            Token::Integer(n) => {
                self.advance();
                Ok(Rc::new(Value::Integer(n)))
            }
            Token::Float(f) => {
                self.advance();
                Ok(Rc::new(Value::Float(f)))
            }
            Token::String(s) => {
                self.advance();
                Ok(Rc::new(Value::String(s)))
            }
            Token::Bool(b) => {
                self.advance();
                Ok(Rc::new(Value::Bool(b)))
            }
        }
    }

    /// Parse a list (s-expression)
    fn parse_list(&mut self) -> Result<Rc<Value>> {
        let open_token = self.peek();
        let open_loc = open_token.location.clone();
        self.advance(); // Skip (

        let mut elements = Vec::new();

        while !self.is_eof() {
            let token = self.peek();
            if matches!(token.token, Token::RParen) {
                self.advance(); // Skip )
                return Ok(Rc::new(Value::List(elements)));
            }

            elements.push(self.parse_expr()?);
        }

        Err(Error::parse_error(
            open_loc.line,
            open_loc.column,
            "Unclosed list (missing ')')",
        ))
    }

    /// Parse a quoted expression: 'x -> (quote x)
    fn parse_quote(&mut self) -> Result<Rc<Value>> {
        self.advance(); // Skip '
        let quoted = self.parse_expr()?;

        Ok(Rc::new(Value::List(vec![
            Rc::new(Value::Symbol("quote".to_string())),
            quoted,
        ])))
    }

    /// Parse a quasiquoted expression: `x -> (quasiquote x)
    fn parse_quasiquote(&mut self) -> Result<Rc<Value>> {
        self.advance(); // Skip `
        let quoted = self.parse_quasiquote_expr()?;

        Ok(Rc::new(Value::List(vec![
            Rc::new(Value::Symbol("quasiquote".to_string())),
            quoted,
        ])))
    }

    /// Parse expression within quasiquote (allows unquote and unquote-splicing)
    fn parse_quasiquote_expr(&mut self) -> Result<Rc<Value>> {
        if self.is_eof() {
            return Err(Error::runtime("Unexpected end of input in quasiquote"));
        }

        let token = self.peek().clone();
        match token.token {
            Token::Unquote => {
                self.advance(); // Skip ,
                let unquoted = self.parse_expr()?;
                Ok(Rc::new(Value::List(vec![
                    Rc::new(Value::Symbol("unquote".to_string())),
                    unquoted,
                ])))
            }
            Token::UnquoteSplicing => {
                self.advance(); // Skip ,@
                let unquoted = self.parse_expr()?;
                Ok(Rc::new(Value::List(vec![
                    Rc::new(Value::Symbol("unquote-splicing".to_string())),
                    unquoted,
                ])))
            }
            Token::LParen => self.parse_quasiquote_list(),
            _ => self.parse_expr(),
        }
    }

    /// Parse a list within quasiquote
    fn parse_quasiquote_list(&mut self) -> Result<Rc<Value>> {
        let open_token = self.peek();
        let open_loc = open_token.location.clone();
        self.advance(); // Skip (

        let mut elements = Vec::new();

        while !self.is_eof() {
            let token = self.peek();
            if matches!(token.token, Token::RParen) {
                self.advance(); // Skip )
                return Ok(Rc::new(Value::List(elements)));
            }

            elements.push(self.parse_quasiquote_expr()?);
        }

        Err(Error::parse_error(
            open_loc.line,
            open_loc.column,
            "Unclosed list in quasiquote (missing ')')",
        ))
    }

    /// Peek at current token
    fn peek(&self) -> &LocatedToken {
        &self.tokens[self.pos]
    }

    /// Advance to next token
    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    /// Check if at end of tokens
    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_str(input: &str) -> Result<Rc<Value>> {
        let tokens = Lexer::tokenize(input)?;
        Parser::parse(tokens)
    }

    fn parse_all_str(input: &str) -> Result<Vec<Rc<Value>>> {
        let tokens = Lexer::tokenize(input)?;
        Parser::parse_all(tokens)
    }

    #[test]
    fn test_parse_integer() {
        let result = parse_str("42").unwrap();
        assert_eq!(*result, Value::Integer(42));
    }

    #[test]
    fn test_parse_float() {
        let result = parse_str("3.14").unwrap();
        assert_eq!(*result, Value::Float(3.14));
    }

    #[test]
    fn test_parse_string() {
        let result = parse_str(r#""hello""#).unwrap();
        assert_eq!(*result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_parse_bool() {
        let result = parse_str("#t").unwrap();
        assert_eq!(*result, Value::Bool(true));

        let result = parse_str("#f").unwrap();
        assert_eq!(*result, Value::Bool(false));
    }

    #[test]
    fn test_parse_symbol() {
        let result = parse_str("foo").unwrap();
        assert_eq!(*result, Value::Symbol("foo".to_string()));
    }

    #[test]
    fn test_parse_simple_list() {
        let result = parse_str("(+ 1 2)").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(*items[0], Value::Symbol("+".to_string()));
                assert_eq!(*items[1], Value::Integer(1));
                assert_eq!(*items[2], Value::Integer(2));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_parse_nested_list() {
        let result = parse_str("(if (> x 0) x (- x))").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 4);
                assert_eq!(*items[0], Value::Symbol("if".to_string()));

                // Check nested condition
                match &*items[1] {
                    Value::List(cond) => {
                        assert_eq!(cond.len(), 3);
                        assert_eq!(*cond[0], Value::Symbol(">".to_string()));
                    }
                    _ => panic!("Expected nested list for condition"),
                }
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_parse_quote() {
        let result = parse_str("'x").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(*items[0], Value::Symbol("quote".to_string()));
                assert_eq!(*items[1], Value::Symbol("x".to_string()));
            }
            _ => panic!("Expected list for quote"),
        }
    }

    #[test]
    fn test_parse_quoted_list() {
        let result = parse_str("'(1 2 3)").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(*items[0], Value::Symbol("quote".to_string()));

                match &*items[1] {
                    Value::List(quoted) => {
                        assert_eq!(quoted.len(), 3);
                        assert_eq!(*quoted[0], Value::Integer(1));
                    }
                    _ => panic!("Expected quoted list"),
                }
            }
            _ => panic!("Expected list for quote"),
        }
    }

    #[test]
    fn test_parse_quasiquote() {
        let result = parse_str("`x").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(*items[0], Value::Symbol("quasiquote".to_string()));
                assert_eq!(*items[1], Value::Symbol("x".to_string()));
            }
            _ => panic!("Expected list for quasiquote"),
        }
    }

    #[test]
    fn test_parse_unquote() {
        let result = parse_str("`(,x)").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(*items[0], Value::Symbol("quasiquote".to_string()));

                match &*items[1] {
                    Value::List(inner) => {
                        assert_eq!(inner.len(), 1);
                        match &*inner[0] {
                            Value::List(unquote) => {
                                assert_eq!(unquote.len(), 2);
                                assert_eq!(*unquote[0], Value::Symbol("unquote".to_string()));
                                assert_eq!(*unquote[1], Value::Symbol("x".to_string()));
                            }
                            _ => panic!("Expected unquote list"),
                        }
                    }
                    _ => panic!("Expected inner list"),
                }
            }
            _ => panic!("Expected list for quasiquote"),
        }
    }

    #[test]
    fn test_parse_unquote_splicing() {
        let result = parse_str("`(,@xs)").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 2);
                match &*items[1] {
                    Value::List(inner) => {
                        assert_eq!(inner.len(), 1);
                        match &*inner[0] {
                            Value::List(unquote) => {
                                assert_eq!(*unquote[0], Value::Symbol("unquote-splicing".to_string()));
                            }
                            _ => panic!("Expected unquote-splicing list"),
                        }
                    }
                    _ => panic!("Expected inner list"),
                }
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_parse_empty_list() {
        let result = parse_str("()").unwrap();
        match &*result {
            Value::List(items) => {
                assert_eq!(items.len(), 0);
            }
            _ => panic!("Expected empty list"),
        }
    }

    #[test]
    fn test_parse_multiple_exprs() {
        let results = parse_all_str("(+ 1 2) (* 3 4)").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_unclosed_list() {
        let result = parse_str("(+ 1 2");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unclosed list"));
    }

    #[test]
    fn test_unexpected_rparen() {
        let result = parse_str(")");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unexpected ')'"));
    }

    #[test]
    fn test_unquote_outside_quasiquote() {
        let result = parse_str(",x");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unquote outside"));
    }

    #[test]
    fn test_deeply_nested() {
        let result = parse_str("(((((x)))))").unwrap();
        match &*result {
            Value::List(l1) => {
                assert_eq!(l1.len(), 1);
                match &*l1[0] {
                    Value::List(l2) => {
                        assert_eq!(l2.len(), 1);
                        // Verify deep nesting works
                        assert!(matches!(&*l2[0], Value::List(_)));
                    }
                    _ => panic!("Expected nested list"),
                }
            }
            _ => panic!("Expected list"),
        }
    }
}
