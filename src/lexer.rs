/// Tokenizer/Lexer for Cracked Shell
///
/// Tokenizes input into atoms, parentheses, quotes, and handles string escaping and comments.

use crate::error::{Error, Result, SourceLocation};

/// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Left parenthesis
    LParen,
    /// Right parenthesis
    RParen,
    /// Symbol/identifier
    Symbol(String),
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f64),
    /// String literal
    String(String),
    /// Boolean literal
    Bool(bool),
    /// Quote '
    Quote,
    /// Quasiquote `
    Quasiquote,
    /// Unquote ,
    Unquote,
    /// Unquote-splicing ,@
    UnquoteSplicing,
}

/// Token with source location
#[derive(Debug, Clone, PartialEq)]
pub struct LocatedToken {
    pub token: Token,
    pub location: SourceLocation,
}

impl LocatedToken {
    fn new(token: Token, line: usize, column: usize) -> Self {
        LocatedToken {
            token,
            location: SourceLocation { line, column },
        }
    }
}

/// Tokenizer state
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer from input string
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize entire input
    pub fn tokenize(input: &str) -> Result<Vec<LocatedToken>> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next_token()? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    /// Get next token
    fn next_token(&mut self) -> Result<Option<LocatedToken>> {
        self.skip_whitespace_and_comments();

        if self.is_eof() {
            return Ok(None);
        }

        let start_line = self.line;
        let start_col = self.column;

        let ch = self.peek();
        let token = match ch {
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            '\'' => {
                self.advance();
                Token::Quote
            }
            '`' => {
                self.advance();
                Token::Quasiquote
            }
            ',' => {
                self.advance();
                if self.peek() == '@' {
                    self.advance();
                    Token::UnquoteSplicing
                } else {
                    Token::Unquote
                }
            }
            '"' => self.read_string()?,
            '#' => self.read_bool_or_symbol()?,
            _ if ch.is_ascii_digit() || (ch == '-' && self.peek_ahead(1).map_or(false, |c| c.is_ascii_digit())) => {
                self.read_number()?
            }
            _ if is_symbol_start(ch) => self.read_symbol(),
            _ => {
                return Err(Error::parse_error(
                    start_line,
                    start_col,
                    format!("Unexpected character: '{}'", ch),
                ))
            }
        };

        Ok(Some(LocatedToken::new(token, start_line, start_col)))
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_eof() {
            let ch = self.peek();
            if ch.is_whitespace() {
                self.advance();
            } else if ch == ';' {
                // Skip line comment
                while !self.is_eof() && self.peek() != '\n' {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    /// Read a string literal
    fn read_string(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_col = self.column;

        self.advance(); // Skip opening "
        let mut result = String::new();

        while !self.is_eof() && self.peek() != '"' {
            let ch = self.peek();
            if ch == '\\' {
                self.advance();
                if self.is_eof() {
                    return Err(Error::parse_error(
                        start_line,
                        start_col,
                        "Unclosed string literal",
                    ));
                }
                let escaped = match self.peek() {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    _ => {
                        return Err(Error::parse_error(
                            self.line,
                            self.column,
                            format!("Invalid escape sequence: \\{}", self.peek()),
                        ))
                    }
                };
                result.push(escaped);
                self.advance();
            } else {
                result.push(ch);
                self.advance();
            }
        }

        if self.is_eof() {
            return Err(Error::parse_error(
                start_line,
                start_col,
                "Unclosed string literal",
            ));
        }

        self.advance(); // Skip closing "
        Ok(Token::String(result))
    }

    /// Read a number (integer or float)
    fn read_number(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_col = self.column;
        let mut num_str = String::new();

        // Handle negative sign
        if self.peek() == '-' {
            num_str.push('-');
            self.advance();
        }

        // Read digits
        while !self.is_eof() && (self.peek().is_ascii_digit() || self.peek() == '.') {
            num_str.push(self.peek());
            self.advance();
        }

        // Check if it's a float or integer
        if num_str.contains('.') {
            num_str
                .parse::<f64>()
                .map(Token::Float)
                .map_err(|_| Error::parse_error(start_line, start_col, format!("Invalid float: {}", num_str)))
        } else {
            num_str
                .parse::<i64>()
                .map(Token::Integer)
                .map_err(|_| Error::parse_error(start_line, start_col, format!("Invalid integer: {}", num_str)))
        }
    }

    /// Read a boolean or symbol starting with #
    fn read_bool_or_symbol(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_col = self.column;

        self.advance(); // Skip #

        if self.is_eof() {
            return Ok(Token::Symbol("#".to_string()));
        }

        let ch = self.peek();
        match ch {
            't' => {
                self.advance();
                Ok(Token::Bool(true))
            }
            'f' => {
                self.advance();
                Ok(Token::Bool(false))
            }
            _ => {
                // It's a symbol starting with #
                let mut sym = String::from("#");
                sym.push_str(&self.read_symbol_chars());
                if sym == "#" {
                    Err(Error::parse_error(start_line, start_col, "Invalid symbol: #"))
                } else {
                    Ok(Token::Symbol(sym))
                }
            }
        }
    }

    /// Read a symbol
    fn read_symbol(&mut self) -> Token {
        Token::Symbol(self.read_symbol_chars())
    }

    /// Read symbol characters
    fn read_symbol_chars(&mut self) -> String {
        let mut result = String::new();

        while !self.is_eof() && is_symbol_char(self.peek()) {
            result.push(self.peek());
            self.advance();
        }

        result
    }

    /// Peek at current character
    fn peek(&self) -> char {
        self.input[self.pos]
    }

    /// Peek ahead n characters
    fn peek_ahead(&self, n: usize) -> Option<char> {
        self.input.get(self.pos + n).copied()
    }

    /// Advance to next character
    fn advance(&mut self) {
        if self.pos < self.input.len() {
            if self.input[self.pos] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.pos += 1;
        }
    }

    /// Check if at end of input
    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

/// Check if character can start a symbol
fn is_symbol_start(ch: char) -> bool {
    ch.is_alphabetic() || "!$%&*+-/<=>?@^_~".contains(ch)
}

/// Check if character can be in a symbol
fn is_symbol_char(ch: char) -> bool {
    ch.is_alphanumeric() || "!$%&*+-/<=>?@^_~:.".contains(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_list() {
        let tokens = Lexer::tokenize("(define x 42)").unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token, Token::LParen);
        assert_eq!(tokens[1].token, Token::Symbol("define".to_string()));
        assert_eq!(tokens[2].token, Token::Symbol("x".to_string()));
        assert_eq!(tokens[3].token, Token::Integer(42));
        assert_eq!(tokens[4].token, Token::RParen);
    }

    #[test]
    fn test_tokenize_string_with_escapes() {
        let tokens = Lexer::tokenize(r#""hello \"world\"""#).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::String(r#"hello "world""#.to_string()));
    }

    #[test]
    fn test_tokenize_comment() {
        let tokens = Lexer::tokenize("; This is a comment\n(+ 1 2)").unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token, Token::LParen);
        assert_eq!(tokens[1].token, Token::Symbol("+".to_string()));
    }

    #[test]
    fn test_tokenize_quote() {
        let tokens = Lexer::tokenize("'x").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::Quote);
        assert_eq!(tokens[1].token, Token::Symbol("x".to_string()));
    }

    #[test]
    fn test_tokenize_quasiquote() {
        let tokens = Lexer::tokenize("`(,x ,@y)").unwrap();
        assert_eq!(tokens[0].token, Token::Quasiquote);
        assert_eq!(tokens[2].token, Token::Unquote);
        assert_eq!(tokens[4].token, Token::UnquoteSplicing);
    }

    #[test]
    fn test_tokenize_bool() {
        let tokens = Lexer::tokenize("#t #f").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::Bool(true));
        assert_eq!(tokens[1].token, Token::Bool(false));
    }

    #[test]
    fn test_tokenize_float() {
        let tokens = Lexer::tokenize("3.14 -2.5").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::Float(3.14));
        assert_eq!(tokens[1].token, Token::Float(-2.5));
    }

    #[test]
    fn test_tokenize_nested() {
        let tokens = Lexer::tokenize("(if (> x 0) x (- x))").unwrap();
        assert!(tokens.len() > 10);
        assert_eq!(tokens[0].token, Token::LParen);
        assert_eq!(tokens[1].token, Token::Symbol("if".to_string()));
    }

    #[test]
    fn test_unclosed_string_error() {
        let result = Lexer::tokenize(r#""unclosed"#);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unclosed string"));
    }

    #[test]
    fn test_unclosed_paren() {
        // This just tests that we can tokenize without matching parens
        // The parser will catch mismatched parens
        let tokens = Lexer::tokenize("(+ 1 2").unwrap();
        assert_eq!(tokens[0].token, Token::LParen);
        assert_eq!(tokens[tokens.len() - 1].token, Token::Integer(2));
    }

    #[test]
    fn test_empty_input() {
        let tokens = Lexer::tokenize("").unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_unicode() {
        let tokens = Lexer::tokenize("\"hello 世界\"").unwrap();
        assert_eq!(tokens[0].token, Token::String("hello 世界".to_string()));
    }

    #[test]
    fn test_symbol_chars() {
        let tokens = Lexer::tokenize("+- foo-bar? baz!").unwrap();
        assert_eq!(tokens[0].token, Token::Symbol("+-".to_string()));
        assert_eq!(tokens[1].token, Token::Symbol("foo-bar?".to_string()));
        assert_eq!(tokens[2].token, Token::Symbol("baz!".to_string()));
    }

    #[test]
    fn test_line_column_tracking() {
        let tokens = Lexer::tokenize("(+\n  1\n  2)").unwrap();
        assert_eq!(tokens[0].location.line, 1);
        assert_eq!(tokens[0].location.column, 1);
        assert_eq!(tokens[2].location.line, 2);
        assert_eq!(tokens[3].location.line, 3);
    }
}
