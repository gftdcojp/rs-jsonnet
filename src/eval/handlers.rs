//! Handler traits for extensible Jsonnet evaluation

use crate::ast::{BinaryOp, UnaryOp, Expr};
use crate::error::Result;
use crate::value::JsonnetValue;
use crate::eval::Context;

/// Handler for binary and unary operations
pub trait OpHandler {
    /// Evaluate a binary operation (e.g., 2 + 3, "hello" + "world")
    fn eval_binary_op(&mut self, context: &mut Context, left: JsonnetValue, op: BinaryOp, right: JsonnetValue) -> Result<JsonnetValue>;

    /// Evaluate a unary operation (e.g., -5, !true)
    fn eval_unary_op(&mut self, context: &mut Context, op: UnaryOp, operand: JsonnetValue) -> Result<JsonnetValue>;
}

/// Handler for function calls (user-defined, builtin, external)
pub trait FuncallHandler {
    /// Call a function with given arguments
    fn call_function(&mut self, context: &mut Context, func: JsonnetValue, args: Vec<JsonnetValue>) -> Result<JsonnetValue>;

    /// Check if a function name refers to a builtin function
    fn is_builtin_function(&self, name: &str) -> bool;

    /// Call a builtin function
    fn call_builtin_function(&mut self, context: &mut Context, name: &str, args: Vec<JsonnetValue>) -> Result<JsonnetValue>;

    /// Check if a function name refers to an external function
    fn is_external_function(&self, name: &str) -> bool;

    /// Call an external function (HTTP, AI API, system commands, etc.)
    fn call_external_function(&mut self, context: &mut Context, name: &str, args: Vec<JsonnetValue>) -> Result<JsonnetValue>;
}

/// Handler for comprehensions (list/dict comprehensions)
pub trait ComprehensionHandler {
    /// Evaluate a list comprehension [expr for var in collection if condition]
    fn eval_list_comprehension(&mut self, context: &mut Context, expr: &Expr, var: &str, collection: &Expr, condition: Option<&Expr>) -> Result<JsonnetValue>;

    /// Evaluate a dict comprehension {key: value for var in collection if condition}
    fn eval_dict_comprehension(&mut self, context: &mut Context, key_expr: &Expr, value_expr: &Expr, var: &str, collection: &Expr, condition: Option<&Expr>) -> Result<JsonnetValue>;
}

/// Default implementations for handlers

/// Default operation handler with standard Jsonnet semantics
pub struct DefaultOpHandler;

impl OpHandler for DefaultOpHandler {
    fn eval_binary_op(&mut self, _context: &mut Context, left: JsonnetValue, op: BinaryOp, right: JsonnetValue) -> Result<JsonnetValue> {
        // This will be moved from the current evaluator implementation
        // For now, return a placeholder
        Err(crate::error::JsonnetError::runtime_error(format!("Binary operation {:?} not implemented", op)))
    }

    fn eval_unary_op(&mut self, _context: &mut Context, op: UnaryOp, _operand: JsonnetValue) -> Result<JsonnetValue> {
        // This will be moved from the current evaluator implementation
        // For now, return a placeholder
        Err(crate::error::JsonnetError::runtime_error(format!("Unary operation {:?} not implemented", op)))
    }
}

/// Default function call handler
pub struct DefaultFuncallHandler;

impl FuncallHandler for DefaultFuncallHandler {
    fn call_function(&mut self, _context: &mut Context, _func: JsonnetValue, _args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Err(crate::error::JsonnetError::runtime_error("Function call not implemented"))
    }

    fn is_builtin_function(&self, name: &str) -> bool {
        // Check if it's a std.* function
        name.starts_with("std.")
    }

    fn call_builtin_function(&mut self, _context: &mut Context, name: &str, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        // This will delegate to the current StdLib implementation
        Err(crate::error::JsonnetError::runtime_error(format!("Builtin function {} not implemented", name)))
    }

    fn is_external_function(&self, name: &str) -> bool {
        // Check for external functions like ai.*, tool.*, memory.*, agent.*, chain.*
        name.starts_with("ai.") ||
        name.starts_with("tool.") ||
        name.starts_with("memory.") ||
        name.starts_with("agent.") ||
        name.starts_with("chain.")
    }

    fn call_external_function(&mut self, _context: &mut Context, name: &str, _args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        // This will delegate to external function handlers
        Err(crate::error::JsonnetError::runtime_error(format!("External function {} not implemented", name)))
    }
}

/// Default comprehension handler
pub struct DefaultComprehensionHandler;

impl ComprehensionHandler for DefaultComprehensionHandler {
    fn eval_list_comprehension(&mut self, _context: &mut Context, _expr: &Expr, _var: &str, _collection: &Expr, _condition: Option<&Expr>) -> Result<JsonnetValue> {
        Err(crate::error::JsonnetError::runtime_error("List comprehension not implemented"))
    }

    fn eval_dict_comprehension(&mut self, _context: &mut Context, _key_expr: &Expr, _value_expr: &Expr, _var: &str, _collection: &Expr, _condition: Option<&Expr>) -> Result<JsonnetValue> {
        Err(crate::error::JsonnetError::runtime_error("Dict comprehension not implemented"))
    }
}
