//! Abstract Syntax Tree for Jsonnet

use serde::{Deserialize, Serialize};

/// Expression in Jsonnet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
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
    /// Object literal
    Object(Vec<(String, Expr)>),
    /// Array literal
    Array(Vec<Expr>),
    /// Function call
    Call(Box<Expr>, Vec<Expr>),
    /// Binary operation
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
    /// Unary operation
    UnaryOp(UnaryOp, Box<Expr>),
}

/// Statement in Jsonnet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    /// Expression statement
    Expr(Expr),
    /// Local variable declaration
    Local(String, Expr),
    /// Assignment
    Assign(String, Expr),
}

/// Binary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Modulo
    Mod,
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Less than
    Lt,
    /// Less than or equal
    Le,
    /// Greater than
    Gt,
    /// Greater than or equal
    Ge,
    /// And
    And,
    /// Or
    Or,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    /// Negation
    Neg,
    /// Not
    Not,
    /// Plus
    Plus,
}
