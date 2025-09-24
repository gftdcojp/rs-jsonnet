//! Lexical analyzer for Jsonnet

use crate::error::{JsonnetError, Result};

/// Token types for the Jsonnet lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Identifier
    Identifier(String),
    /// String literal
    String(String),
    /// Number literal
    Number(f64),
    /// Boolean literal
    Boolean(bool),
    /// Null literal
    Null,
    /// Left brace
    LeftBrace,
    /// Right brace
    RightBrace,
    /// Left bracket
    LeftBracket,
    /// Right bracket
    RightBracket,
    /// Left parenthesis
    LeftParen,
    /// Right parenthesis
    RightParen,
    /// Comma
    Comma,
    /// Colon
    Colon,
    /// Semicolon
    Semicolon,
    /// Dot
    Dot,
    /// Plus
    Plus,
    /// Minus
    Minus,
    /// Star
    Star,
    /// Slash
    Slash,
    /// Percent
    Percent,
    /// Equal
    Equal,
    /// Not equal
    NotEqual,
    /// Less than
    LessThan,
    /// Less than or equal
    LessThanEqual,
    /// Greater than
    GreaterThan,
    /// Greater than or equal
    GreaterThanEqual,
    /// And
    And,
    /// Or
    Or,
    /// Not
    Not,
    /// If
    If,
    /// Then
    Then,
    /// Else
    Else,
    /// For
    For,
    /// In
    In,
    /// Function
    Function,
    /// Local
    Local,
    /// Import
    Import,
    /// Importstr
    Importstr,
    /// Error
    Error,
    /// End of file
    Eof,
}

/// Lexer for Jsonnet source code
pub struct Lexer {
    /// Source code
    source: String,
    /// Current position
    position: usize,
    /// Current line
    line: usize,
    /// Current column
    column: usize,
}

impl Lexer {
    /// Create a new lexer
    pub fn new(source: String) -> Self {
        Self {
            source,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Get the current character
    fn current(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }

    /// Advance to the next character
    fn advance(&mut self) {
        if let Some(ch) = self.current() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }

    /// Peek at the next character
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.position + 1)
    }

    /// Check if the current position matches a string
    fn matches(&self, s: &str) -> bool {
        self.source[self.position..].starts_with(s)
    }

    /// Advance by a string
    fn advance_by(&mut self, s: &str) {
        for _ in 0..s.len() {
            self.advance();
        }
    }

    /// Tokenize the source code
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current() {
            match ch {
                // Whitespace
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                // Comments
                '#' => {
                    while let Some(ch) = self.current() {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = self.position;
                    while let Some(ch) = self.current() {
                        if ch.is_alphanumeric() || ch == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let ident = &self.source[start..self.position];
                    let token = match ident {
                        "null" => Token::Null,
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        "if" => Token::If,
                        "then" => Token::Then,
                        "else" => Token::Else,
                        "for" => Token::For,
                        "in" => Token::In,
                        "function" => Token::Function,
                        "local" => Token::Local,
                        "import" => Token::Import,
                        "importstr" => Token::Importstr,
                        "error" => Token::Error,
                        _ => Token::Identifier(ident.to_string()),
                    };
                    tokens.push(token);
                }
                // Numbers
                '0'..='9' => {
                    let start = self.position;
                    while let Some(ch) = self.current() {
                        if ch.is_digit(10) || ch == '.' || ch == 'e' || ch == 'E' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let num_str = &self.source[start..self.position];
                    let num = num_str.parse().map_err(|_| {
                        JsonnetError::Lexical(format!("Invalid number: {}", num_str))
                    })?;
                    tokens.push(Token::Number(num));
                }
                // String literals
                '"' => {
                    self.advance();
                    let start = self.position;
                    while let Some(ch) = self.current() {
                        if ch == '"' {
                            break;
                        }
                        if ch == '\\' {
                            self.advance(); // Skip escape character
                        }
                        self.advance();
                    }
                    if let Some('"') = self.current() {
                        self.advance();
                        let string = &self.source[start..self.position - 1];
                        tokens.push(Token::String(string.to_string()));
                    } else {
                        return Err(JsonnetError::Lexical("Unterminated string".to_string()));
                    }
                }
                // Operators and punctuation
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.advance();
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.advance();
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.advance();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                }
                '.' => {
                    tokens.push(Token::Dot);
                    self.advance();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }
                '%' => {
                    tokens.push(Token::Percent);
                    self.advance();
                }
                '=' => {
                    if self.matches("==") {
                        tokens.push(Token::Equal);
                        self.advance_by("==");
                    } else {
                        tokens.push(Token::Equal);
                        self.advance();
                    }
                }
                '!' => {
                    if self.matches("!=") {
                        tokens.push(Token::NotEqual);
                        self.advance_by("!=");
                    } else {
                        tokens.push(Token::Not);
                        self.advance();
                    }
                }
                '<' => {
                    if self.matches("<=") {
                        tokens.push(Token::LessThanEqual);
                        self.advance_by("<=");
                    } else {
                        tokens.push(Token::LessThan);
                        self.advance();
                    }
                }
                '>' => {
                    if self.matches(">=") {
                        tokens.push(Token::GreaterThanEqual);
                        self.advance_by(">=");
                    } else {
                        tokens.push(Token::GreaterThan);
                        self.advance();
                    }
                }
                '&' => {
                    if self.matches("&&") {
                        tokens.push(Token::And);
                        self.advance_by("&&");
                    } else {
                        return Err(JsonnetError::Lexical("Unexpected character: &".to_string()));
                    }
                }
                '|' => {
                    if self.matches("||") {
                        tokens.push(Token::Or);
                        self.advance_by("||");
                    } else {
                        return Err(JsonnetError::Lexical("Unexpected character: |".to_string()));
                    }
                }
                _ => {
                    return Err(JsonnetError::Lexical(format!("Unexpected character: {}", ch)));
                }
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }
}
