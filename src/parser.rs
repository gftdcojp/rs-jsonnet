//! Parser for Jsonnet AST

use crate::ast::{BinaryOp, Expr, Stmt, StringPart, UnaryOp};
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
        // Parse semicolon-separated expressions, return the last one
        let mut expr = self.parse_expression()?;

        while matches!(self.current(), Some(Token::Semicolon)) {
            self.advance(); // consume ';'
            if matches!(self.current(), Some(Token::Eof)) {
                break; // trailing semicolon is ok
            }
            expr = self.parse_expression()?;
        }

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

            loop {
                // Parse variable name
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

                // Check for comma (multiple bindings) or semicolon (end of bindings)
                if matches!(self.current(), Some(Token::Comma)) {
                    self.advance(); // consume ','
                } else {
                    break;
                }
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
                // Check if this string contains interpolation
                if s.contains("%(") && s.contains(")s") {
                    self.parse_string_interpolation(&s)
                } else {
                    Ok(Expr::String(s))
                }
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
            Some(Token::LeftBracket) => {
                self.advance(); // consume '['
                self.parse_array()
            },
            Some(Token::Function) => self.parse_function(),
            Some(Token::LeftParen) => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                self.expect_token(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err(JsonnetError::parse_error(0, 0, "Expected expression")),
        }
    }

    /// Parse postfix expressions (primary + indexing/field access/function calls)
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
                Some(Token::LeftParen) => {
                    // Function call
                    self.advance(); // consume '('
                    let mut args = Vec::new();

                    if !matches!(self.current(), Some(Token::RightParen)) {
                        loop {
                            let arg = self.parse_expression()?;
                            args.push(arg);

                            if !matches!(self.current(), Some(Token::Comma)) {
                                break;
                            }
                            self.advance(); // consume ','
                        }
                    }

                    self.expect_token(Token::RightParen)?;
                    expr = Expr::Call(Box::new(expr), args);
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
        // Note: LeftBracket is already consumed

        if matches!(self.current(), Some(Token::RightBracket)) {
            // Empty array
            self.advance(); // consume ']'
            return Ok(Expr::Array(Vec::new()));
        }

        // Parse first expression
        let expr = self.parse_expression()?;

        // Check if this is an array comprehension (next token is 'for')
        if matches!(self.current(), Some(Token::For)) {
            // Array comprehension
            self.advance(); // consume 'for'
            let var_name = match self.current().cloned() {
                Some(Token::Identifier(name)) => {
                    self.advance();
                    name
                }
                _ => return Err(JsonnetError::parse_error(0, 0, "Expected variable name after 'for'")),
            };

            self.expect_token(Token::In)?;
            let array_expr = self.parse_expression()?;

            // Optional condition
            let condition = if matches!(self.current(), Some(Token::If)) {
                self.advance(); // consume 'if'
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };

            self.expect_token(Token::RightBracket)?;

            return Ok(Expr::ArrayComprehension {
                expr: Box::new(expr),
                var_name,
                array_expr: Box::new(array_expr),
                condition,
            });
        }

        // Regular array - parse remaining elements
        let mut elements = vec![expr];
        while matches!(self.current(), Some(Token::Comma)) {
            self.advance(); // consume ','
            let expr = self.parse_expression()?;
            elements.push(expr);
        }

        self.expect_token(Token::RightBracket)?;
        Ok(Expr::Array(elements))
    }


    /// Parse function definition
    fn parse_function(&mut self) -> Result<Expr> {
        self.expect_token(Token::Function)?;

        // Parse parameters
        self.expect_token(Token::LeftParen)?;
        let mut params = Vec::new();

        if !matches!(self.current(), Some(Token::RightParen)) {
            loop {
                match self.current().cloned() {
                    Some(Token::Identifier(param)) => {
                        self.advance();
                        params.push(param);
                    }
                    _ => return Err(JsonnetError::parse_error(0, 0, "Expected parameter name")),
                }

                if !matches!(self.current(), Some(Token::Comma)) {
                    break;
                }
                self.advance(); // consume ','
            }
        }

        self.expect_token(Token::RightParen)?;

        // Parse function body
        let body = self.parse_expression()?;

        Ok(Expr::Function(params, Box::new(body)))
    }

    /// Parse string interpolation
    fn parse_string_interpolation(&mut self, s: &str) -> Result<Expr> {
        let mut parts = Vec::new();
        let mut remaining = s;

        while let Some(start) = remaining.find("%(") {
            // Add literal part before interpolation
            if start > 0 {
                parts.push(StringPart::Literal(remaining[..start].to_string()));
            }

            // Find the closing )s
            if let Some(end) = remaining[start..].find(")s") {
                let var_part = &remaining[start + 2..start + end];
                // For now, treat as identifier. In full implementation, this should be parsed as expression
                parts.push(StringPart::Interpolation(Expr::Identifier(var_part.to_string())));
                remaining = &remaining[start + end + 2..];
            } else {
                // Not a valid interpolation, treat rest as literal
                parts.push(StringPart::Literal(remaining.to_string()));
                break;
            }
        }

        // Add remaining literal part
        if !remaining.is_empty() {
            parts.push(StringPart::Literal(remaining.to_string()));
        }

        if parts.is_empty() {
            // No interpolation found, return as literal string
            return Ok(Expr::String(s.to_string()));
        }

        Ok(Expr::StringInterpolation(parts))
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
