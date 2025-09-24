//! Parser for Jsonnet AST

use crate::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::error::{JsonnetError, Result};
use crate::lexer::Token;

/// Parser for Jsonnet source code
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Get the current token
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Advance to the next token
    fn advance(&mut self) {
        self.position += 1;
    }

    /// Parse the tokens into an AST
    pub fn parse(&mut self) -> Result<Expr> {
        let expr = self.parse_expression()?;
        if !matches!(self.current(), Some(Token::Eof)) {
            return Err(JsonnetError::parse_error(
                0, 0, // TODO: track line/column properly
                "Unexpected tokens after expression"
            ));
        }
        Ok(expr)
    }

    /// Parse an expression with precedence
    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_local()
    }

    /// Parse local variable bindings
    fn parse_local(&mut self) -> Result<Expr> {
        if matches!(self.current(), Some(Token::Local)) {
            self.advance(); // consume 'local'

            let mut bindings = Vec::new();

            // Parse first binding
            let name = match self.current().cloned() {
                Some(Token::Identifier(id)) => {
                    self.advance();
                    id
                }
                _ => return Err(JsonnetError::parse_error(0, 0, "Expected variable name after 'local'")),
            };

            self.expect_token(Token::Equal)?;
            let value = self.parse_expression()?;
            bindings.push((name, value));

            // Parse additional bindings (comma-separated)
            while matches!(self.current(), Some(Token::Comma)) {
                self.advance(); // consume ','

                let name = match self.current().cloned() {
                    Some(Token::Identifier(id)) => {
                        self.advance();
                        id
                    }
                    _ => return Err(JsonnetError::parse_error(0, 0, "Expected variable name after ','")),
                };

                self.expect_token(Token::Equal)?;
                let value = self.parse_expression()?;
                bindings.push((name, value));
            }

            self.expect_token(Token::Semicolon)?;
            let body = self.parse_expression()?;

            Ok(Expr::Local(bindings, Box::new(body)))
        } else {
            self.parse_conditional()
        }
    }

    /// Parse conditional expressions (if then else)
    fn parse_conditional(&mut self) -> Result<Expr> {
        if matches!(self.current(), Some(Token::If)) {
            self.advance(); // consume 'if'

            let condition = self.parse_expression()?;

            self.expect_token(Token::Then)?;
            let then_branch = self.parse_expression()?;

            self.expect_token(Token::Else)?;
            let else_branch = self.parse_expression()?;

            Ok(Expr::Conditional(
                Box::new(condition),
                Box::new(then_branch),
                Box::new(else_branch),
            ))
        } else {
            self.parse_or()
        }
    }

    /// Parse or expressions (lowest precedence)
    fn parse_or(&mut self) -> Result<Expr> {
        let mut expr = self.parse_and()?;

        while let Some(Token::Or) = self.current() {
            self.advance();
            let right = self.parse_and()?;
            expr = Expr::BinaryOp(BinaryOp::Or, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    /// Parse and expressions
    fn parse_and(&mut self) -> Result<Expr> {
        let mut expr = self.parse_comparison()?;

        while let Some(Token::And) = self.current() {
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::BinaryOp(BinaryOp::And, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    /// Parse comparison expressions
    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut expr = self.parse_additive()?;

        loop {
            let op = match self.current() {
                Some(Token::Equal) => BinaryOp::Eq,
                Some(Token::NotEqual) => BinaryOp::Ne,
                Some(Token::LessThan) => BinaryOp::Lt,
                Some(Token::LessThanEqual) => BinaryOp::Le,
                Some(Token::GreaterThan) => BinaryOp::Gt,
                Some(Token::GreaterThanEqual) => BinaryOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            expr = Expr::BinaryOp(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    /// Parse additive expressions (+, -)
    fn parse_additive(&mut self) -> Result<Expr> {
        let mut expr = self.parse_multiplicative()?;

        loop {
            let op = match self.current() {
                Some(Token::Plus) => BinaryOp::Add,
                Some(Token::Minus) => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            expr = Expr::BinaryOp(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    /// Parse multiplicative expressions (*, /, %)
    fn parse_multiplicative(&mut self) -> Result<Expr> {
        let mut expr = self.parse_unary()?;

        loop {
            let op = match self.current() {
                Some(Token::Star) => BinaryOp::Mul,
                Some(Token::Slash) => BinaryOp::Div,
                Some(Token::Percent) => BinaryOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::BinaryOp(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    /// Parse unary expressions
    fn parse_unary(&mut self) -> Result<Expr> {
        if let Some(Token::Minus) = self.current() {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp(UnaryOp::Neg, Box::new(expr)));
        }
        if let Some(Token::Plus) = self.current() {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp(UnaryOp::Plus, Box::new(expr)));
        }
        if let Some(Token::Not) = self.current() {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp(UnaryOp::Not, Box::new(expr)));
        }

        self.parse_postfix()
    }

    /// Parse primary expressions (literals, identifiers, parentheses, objects, arrays)
    fn parse_primary(&mut self) -> Result<Expr> {
        match self.current().cloned() {
            Some(Token::String(s)) => {
                self.advance();
                Ok(Expr::String(s))
            }
            Some(Token::Number(n)) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Some(Token::Boolean(b)) => {
                self.advance();
                Ok(Expr::Boolean(b))
            }
            Some(Token::Null) => {
                self.advance();
                Ok(Expr::Null)
            }
            Some(Token::Identifier(id)) => {
                self.advance();
                Ok(Expr::Identifier(id))
            }
            Some(Token::LeftBrace) => self.parse_object(),
            Some(Token::LeftBracket) => self.parse_array(),
            Some(Token::LeftParen) => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                self.expect_token(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(JsonnetError::parse_error(0, 0, "Expected expression")),
        }
    }

    /// Parse postfix expressions (primary + indexing/field access)
    fn parse_postfix(&mut self) -> Result<Expr> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current() {
                Some(Token::LeftBracket) => {
                    self.advance(); // consume '['
                    let index = self.parse_expression()?;
                    self.expect_token(Token::RightBracket)?;
                    expr = Expr::ArrayAccess(Box::new(expr), Box::new(index));
                }
                Some(Token::Dot) => {
                    self.advance(); // consume '.'
                    match self.current().cloned() {
                        Some(Token::Identifier(field)) => {
                            self.advance();
                            expr = Expr::FieldAccess(Box::new(expr), field);
                        }
                        _ => return Err(JsonnetError::parse_error(0, 0, "Expected field name after '.'")),
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parse object literal { key: value, ... }
    fn parse_object(&mut self) -> Result<Expr> {
        self.expect_token(Token::LeftBrace)?;
        let mut fields = Vec::new();

        if !matches!(self.current(), Some(Token::RightBrace)) {
            loop {
                // For now, only support string keys
                let key = match self.current().cloned() {
                    Some(Token::String(s)) => {
                        self.advance();
                        s
                    }
                    Some(Token::Identifier(id)) => {
                        self.advance();
                        id
                    }
                    _ => return Err(JsonnetError::parse_error(0, 0, "Expected object key")),
                };

                self.expect_token(Token::Colon)?;
                let value = self.parse_expression()?;
                fields.push((key, value));

                if !matches!(self.current(), Some(Token::Comma)) {
                    break;
                }
                self.advance(); // consume ','
            }
        }

        self.expect_token(Token::RightBrace)?;
        Ok(Expr::Object(fields))
    }

    /// Parse array literal [ expr, expr, ... ]
    fn parse_array(&mut self) -> Result<Expr> {
        self.expect_token(Token::LeftBracket)?;
        let mut elements = Vec::new();

        if !matches!(self.current(), Some(Token::RightBracket)) {
            loop {
                let expr = self.parse_expression()?;
                elements.push(expr);

                if !matches!(self.current(), Some(Token::Comma)) {
                    break;
                }
                self.advance(); // consume ','
            }
        }

        self.expect_token(Token::RightBracket)?;
        Ok(Expr::Array(elements))
    }

    /// Expect a specific token, advance if found
    fn expect_token(&mut self, expected: Token) -> Result<()> {
        match self.current() {
            Some(token) if token == &expected => {
                self.advance();
                Ok(())
            }
            _ => Err(JsonnetError::parse_error(0, 0, format!("Expected {:?}", expected))),
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(vec![])
    }
}
