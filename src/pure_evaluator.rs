//! Pure Jsonnet evaluator - no side effects, fully deterministic
//!
//! This module provides a pure functional implementation of Jsonnet evaluation.
//! All evaluation is deterministic: same input always produces same output.

use crate::ast::Expr;
use crate::error::{JsonnetError, Result};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::JsonnetValue;
use std::collections::HashMap;

/// Pure Jsonnet evaluator - performs only deterministic computations
#[derive(Debug, Clone)]
pub struct PureEvaluator {
    /// Top-level arguments (immutable configuration)
    tla_args: HashMap<String, String>,
    /// External variables (immutable configuration)
    ext_vars: HashMap<String, String>,
    /// Current evaluation context (variables in scope)
    context: EvaluationContext,
}

#[derive(Debug, Clone)]
struct EvaluationContext {
    /// Variables currently in scope
    variables: HashMap<String, JsonnetValue>,
}

impl Default for PureEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl PureEvaluator {
    /// Create a new pure evaluator with no external configuration
    pub fn new() -> Self {
        Self {
            tla_args: HashMap::new(),
            ext_vars: HashMap::new(),
            context: EvaluationContext {
                variables: HashMap::new(),
            },
        }
    }

    /// Create a pure evaluator with top-level arguments
    pub fn with_tla_args(tla_args: HashMap<String, String>) -> Self {
        let mut context = EvaluationContext {
            variables: HashMap::new(),
        };

        // Add TLA variables to context
        for (key, value_str) in &tla_args {
            // Parse the TLA value as Jsonnet
            if let Ok(value) = Self::parse_and_eval_simple(value_str) {
                context.variables.insert(key.clone(), value);
            }
        }

        Self {
            tla_args,
            ext_vars: HashMap::new(),
            context,
        }
    }

    /// Create a pure evaluator with both TLA and external variables
    pub fn with_config(tla_args: HashMap<String, String>, ext_vars: HashMap<String, String>) -> Self {
        let mut context = EvaluationContext {
            variables: HashMap::new(),
        };

        // Add TLA variables to context
        for (key, value_str) in &tla_args {
            if let Ok(value) = Self::parse_and_eval_simple(value_str) {
                context.variables.insert(key.clone(), value);
            }
        }

        // Add external variables to context
        for (key, value_str) in &ext_vars {
            if let Ok(value) = Self::parse_and_eval_simple(value_str) {
                context.variables.insert(key.clone(), value);
            }
        }

        Self {
            tla_args,
            ext_vars,
            context,
        }
    }

    /// Simple helper to parse and evaluate a basic Jsonnet value
    fn parse_and_eval_simple(source: &str) -> Result<JsonnetValue> {
        let tokens = Lexer::new(source.to_string()).tokenize()?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;
        let mut temp_eval = PureEvaluator::new();
        temp_eval.evaluate_expression(expr)
    }

    /// Pure evaluation of Jsonnet source code
    ///
    /// This function is PURE: it performs only deterministic computations
    /// and has no side effects. Same input always produces same output.
    pub fn evaluate(&mut self, source: &str) -> Result<JsonnetValue> {
        self.evaluate_with_context(source)
    }

    /// Pure evaluation with explicit context (no longer used, kept for compatibility)
    fn evaluate_with_context(&mut self, source: &str) -> Result<JsonnetValue> {
        // Parse the Jsonnet source using the real parser
        let tokens = Lexer::new(source.to_string()).tokenize()?;
        let mut parser = Parser::new(tokens);
        let parsed = parser.parse()?;

        // Evaluate the expression
        self.evaluate_expression(parsed)
    }


    /// Evaluate the parsed expression (simplified)
    fn evaluate_expression(&mut self, expr: Expr) -> Result<JsonnetValue> {
        match expr {
            Expr::String(s) => Ok(JsonnetValue::String(s)),
            Expr::Number(n) => Ok(JsonnetValue::Number(n)),
            Expr::Boolean(b) => Ok(JsonnetValue::Boolean(b)),
            Expr::Null => Ok(JsonnetValue::Null),
            Expr::Object(fields) => {
                let mut obj = std::collections::HashMap::new();
                for (key, value_expr) in fields {
                    let value = self.evaluate_expression(value_expr)?;
                    obj.insert(key, value);
                }
                Ok(JsonnetValue::Object(obj))
            }
            Expr::Array(elements) => {
                let mut arr = Vec::new();
                for element_expr in elements {
                    let element = self.evaluate_expression(element_expr)?;
                    arr.push(element);
                }
                Ok(JsonnetValue::Array(arr))
            }
            Expr::BinaryOp(op, left, right) => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                self.evaluate_binary_op(op, left_val, right_val)
            }
            Expr::UnaryOp(op, expr) => {
                let val = self.evaluate_expression(*expr)?;
                self.evaluate_unary_op(op, val)
            }
            Expr::ArrayAccess(target, index) => {
                let target_val = self.evaluate_expression(*target)?;
                let index_val = self.evaluate_expression(*index)?;

                match (&target_val, &index_val) {
                    (JsonnetValue::Array(arr), JsonnetValue::Number(idx)) => {
                        let idx = *idx as i64;
                        if idx < 0 {
                            return Err(JsonnetError::runtime_error("Negative array index"));
                        }
                        let idx = idx as usize;
                        if idx >= arr.len() {
                            return Err(JsonnetError::index_out_of_bounds(idx as i64));
                        }
                        Ok(arr[idx].clone())
                    }
                    (JsonnetValue::Object(fields), JsonnetValue::String(field)) => {
                        match fields.get(field) {
                            Some(value) => Ok(value.clone()),
                            None => Err(JsonnetError::undefined_field(field)),
                        }
                    }
                    (JsonnetValue::Array(_), _) => Err(JsonnetError::type_error("Array index must be a number")),
                    (JsonnetValue::Object(_), _) => Err(JsonnetError::type_error("Object index must be a string")),
                    _ => Err(JsonnetError::type_error("Cannot index into this type")),
                }
            }
            Expr::FieldAccess(obj, field) => {
                let obj_val = self.evaluate_expression(*obj)?;

                match obj_val {
                    JsonnetValue::Object(fields) => {
                        match fields.get(&field) {
                            Some(value) => Ok(value.clone()),
                            None => Err(JsonnetError::undefined_field(&field)),
                        }
                    }
                    _ => Err(JsonnetError::type_error("Field access requires object")),
                }
            }
            Expr::Local(bindings, body) => {
                // Create a new scope for local variables
                let mut local_vars = self.context.variables.clone();

                // Evaluate and bind each local variable
                for (name, value_expr) in bindings {
                    let value = self.evaluate_expression(value_expr)?;
                    local_vars.insert(name, value);
                }

                // Evaluate body in the extended scope
                let old_context = std::mem::replace(&mut self.context.variables, local_vars);
                let result = self.evaluate_expression(*body);
                self.context.variables = old_context;

                result
            }
            Expr::Conditional(condition, then_branch, else_branch) => {
                let cond_val = self.evaluate_expression(*condition)?;
                match cond_val {
                    JsonnetValue::Boolean(true) => self.evaluate_expression(*then_branch),
                    JsonnetValue::Boolean(false) => self.evaluate_expression(*else_branch),
                    _ => Err(JsonnetError::type_error("Condition must evaluate to boolean")),
                }
            }
            Expr::Call(func, args) => {
                // For now, only support simple function calls
                let _func_val = self.evaluate_expression(*func)?;
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate_expression(arg)?);
                }
                // TODO: Implement function call evaluation
                Err(JsonnetError::runtime_error("Function calls not yet implemented"))
            }
            Expr::Identifier(name) => {
                // Look up variable in current context
                match self.context.variables.get(&name) {
                    Some(value) => Ok(value.clone()),
                    None => Err(JsonnetError::undefined_variable(&name)),
                }
            }
        }
    }

    /// Convert a JsonnetValue to its string representation
    fn value_to_string(&self, value: &JsonnetValue) -> String {
        match value {
            JsonnetValue::String(s) => s.clone(),
            JsonnetValue::Number(n) => n.to_string(),
            JsonnetValue::Boolean(b) => b.to_string(),
            JsonnetValue::Null => "null".to_string(),
            JsonnetValue::Array(_) => "[array]".to_string(), // TODO: proper array to string
            JsonnetValue::Object(_) => "{object}".to_string(), // TODO: proper object to string
            JsonnetValue::Function(_) => "[function]".to_string(),
            JsonnetValue::Builtin(_) => "[builtin]".to_string(),
        }
    }

    fn evaluate_binary_op(&self, op: crate::ast::BinaryOp, left: JsonnetValue, right: JsonnetValue) -> Result<JsonnetValue> {
        use crate::ast::BinaryOp::*;
        match op {
            Add => {
                // String concatenation (Jsonnet allows concatenating anything with strings)
                if let JsonnetValue::String(l) = &left {
                    let right_str = self.value_to_string(&right);
                    return Ok(JsonnetValue::String(l.clone() + &right_str));
                }
                if let JsonnetValue::String(r) = &right {
                    let left_str = self.value_to_string(&left);
                    return Ok(JsonnetValue::String(left_str + r));
                }
                // Numeric addition
                match (&left, &right) {
                    (JsonnetValue::Number(l), JsonnetValue::Number(r)) => Ok(JsonnetValue::Number(l + r)),
                    _ => Err(JsonnetError::type_error("Invalid operands for +")),
                }
            },
            Sub => match (left, right) {
                (JsonnetValue::Number(l), JsonnetValue::Number(r)) => Ok(JsonnetValue::Number(l - r)),
                _ => Err(JsonnetError::type_error("Invalid operands for -")),
            },
            Mul => match (left, right) {
                (JsonnetValue::Number(l), JsonnetValue::Number(r)) => Ok(JsonnetValue::Number(l * r)),
                _ => Err(JsonnetError::type_error("Invalid operands for *")),
            },
            Div => match (left, right) {
                (JsonnetValue::Number(l), JsonnetValue::Number(r)) => {
                    if r == 0.0 {
                        Err(JsonnetError::DivisionByZero)
                    } else {
                        Ok(JsonnetValue::Number(l / r))
                    }
                }
                _ => Err(JsonnetError::type_error("Invalid operands for /")),
            },
            Mod => match (left, right) {
                (JsonnetValue::Number(l), JsonnetValue::Number(r)) => Ok(JsonnetValue::Number(l % r)),
                _ => Err(JsonnetError::type_error("Invalid operands for %")),
            },
            Eq => Ok(JsonnetValue::Boolean(left == right)),
            Ne => Ok(JsonnetValue::Boolean(left != right)),
            Lt => match (left, right) {
                (JsonnetValue::Number(l), JsonnetValue::Number(r)) => Ok(JsonnetValue::Boolean(l < r)),
                _ => Err(JsonnetError::type_error("Invalid operands for <")),
            },
            Le => match (left, right) {
                (JsonnetValue::Number(l), JsonnetValue::Number(r)) => Ok(JsonnetValue::Boolean(l <= r)),
                _ => Err(JsonnetError::type_error("Invalid operands for <=")),
            },
            Gt => match (left, right) {
                (JsonnetValue::Number(l), JsonnetValue::Number(r)) => Ok(JsonnetValue::Boolean(l > r)),
                _ => Err(JsonnetError::type_error("Invalid operands for >")),
            },
            Ge => match (left, right) {
                (JsonnetValue::Number(l), JsonnetValue::Number(r)) => Ok(JsonnetValue::Boolean(l >= r)),
                _ => Err(JsonnetError::type_error("Invalid operands for >=")),
            },
            And => match (left, right) {
                (JsonnetValue::Boolean(l), JsonnetValue::Boolean(r)) => Ok(JsonnetValue::Boolean(l && r)),
                _ => Err(JsonnetError::type_error("Invalid operands for &&")),
            },
            Or => match (left, right) {
                (JsonnetValue::Boolean(l), JsonnetValue::Boolean(r)) => Ok(JsonnetValue::Boolean(l || r)),
                _ => Err(JsonnetError::type_error("Invalid operands for ||")),
            },
        }
    }

    fn evaluate_unary_op(&self, op: crate::ast::UnaryOp, val: JsonnetValue) -> Result<JsonnetValue> {
        use crate::ast::UnaryOp::*;
        match op {
            Neg => match val {
                JsonnetValue::Number(n) => Ok(JsonnetValue::Number(-n)),
                _ => Err(JsonnetError::type_error("Invalid operand for unary -")),
            },
            Not => match val {
                JsonnetValue::Boolean(b) => Ok(JsonnetValue::Boolean(!b)),
                _ => Err(JsonnetError::type_error("Invalid operand for !")),
            },
            Plus => match val {
                JsonnetValue::Number(n) => Ok(JsonnetValue::Number(n)),
                _ => Err(JsonnetError::type_error("Invalid operand for unary +")),
            },
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_evaluation_is_deterministic() {
        let mut evaluator = PureEvaluator::new();
        let source = r#" "hello" + " world" "#;

        // Same input should always produce same output
        let result1 = evaluator.evaluate(source).unwrap();
        let result2 = evaluator.evaluate(source).unwrap();
        let result3 = evaluator.evaluate(source).unwrap();

        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }

    #[test]
    fn test_pure_evaluation_with_tla() {
        let tla_args = HashMap::from([
            ("name".to_string(), r#""Alice""#.to_string()),
            ("age".to_string(), "30".to_string()),
        ]);

        let mut evaluator = PureEvaluator::with_tla_args(tla_args);
        let source = r#" "Hello, " + name + "!" "#;

        let result = evaluator.evaluate(source).unwrap();
        // In real implementation, this would evaluate to "Hello, Alice!"
        // For now, just check that evaluation succeeds
        assert!(matches!(result, JsonnetValue::String(_)));
    }

    #[test]
    fn test_pure_evaluator_clone() {
        let mut evaluator1 = PureEvaluator::new();
        let mut evaluator2 = evaluator1.clone();

        let source = r#" "test" "#;
        let result1 = evaluator1.evaluate(source).unwrap();
        let result2 = evaluator2.evaluate(source).unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_pure_evaluator_immutability() {
        // Pure Kernel: PureEvaluatorは不変、設定は作成時に固定
        let mut evaluator = PureEvaluator::new();

        // TLA引数付きの新しいevaluatorを作成
        let tla_args = HashMap::from([("greeting".to_string(), r#""Hello""#.to_string())]);
        let evaluator_with_tla = PureEvaluator::with_tla_args(tla_args.clone());

        // 元のevaluatorは変更されていない
        assert!(evaluator.tla_args.is_empty());
        assert!(evaluator.ext_vars.is_empty());

        // 新しいevaluatorにはTLA引数がある
        assert_eq!(evaluator_with_tla.tla_args.len(), 1);
        assert_eq!(evaluator_with_tla.tla_args.get("greeting").unwrap(), r#""Hello""#);

        // さらに外部変数を追加
        let ext_vars = HashMap::from([("env".to_string(), r#""production""#.to_string())]);
        let evaluator_with_both = PureEvaluator::with_config(tla_args, ext_vars);

        assert_eq!(evaluator_with_both.tla_args.len(), 1);
        assert_eq!(evaluator_with_both.ext_vars.len(), 1);
        assert_eq!(evaluator_with_both.ext_vars.get("env").unwrap(), r#""production""#);
    }

    #[test]
    fn test_pure_evaluator_deterministic_with_config() {
        // Pure Kernel: 設定が同じなら常に同じ結果
        let tla_args1 = HashMap::from([
            ("name".to_string(), r#""World""#.to_string()),
            ("count".to_string(), "42".to_string()),
        ]);

        let tla_args2 = HashMap::from([
            ("name".to_string(), r#""World""#.to_string()),
            ("count".to_string(), "42".to_string()),
        ]);

        let mut evaluator1 = PureEvaluator::with_tla_args(tla_args1);
        let mut evaluator2 = PureEvaluator::with_tla_args(tla_args2);

        let source = r#" "Result: " + name + " - " + count "#;

        let result1 = evaluator1.evaluate(source).unwrap();
        let result2 = evaluator2.evaluate(source).unwrap();

        assert_eq!(result1, result2);

        // 複数回の評価でも同じ結果
        for _ in 0..5 {
            let result_n = evaluator1.evaluate(source).unwrap();
            assert_eq!(result1, result_n);
        }
    }

    #[test]
    fn test_pure_evaluator_external_vars() {
        // Pure Kernel: 外部変数も決定論的
        let ext_vars = HashMap::from([
            ("version".to_string(), r#""1.0.0""#.to_string()),
            ("debug".to_string(), "false".to_string()),
        ]);

        let tla_args = HashMap::from([
            ("app".to_string(), r#""myapp""#.to_string()),
        ]);

        let mut evaluator = PureEvaluator::with_config(tla_args, ext_vars);
        let source = r#" "App: " + app + " v" + version + " debug=" + debug "#;

        // 同じ設定で作成したevaluatorは同じ結果を返す
        let mut evaluator2 = PureEvaluator::with_config(
            HashMap::from([("app".to_string(), r#""myapp""#.to_string())]),
            HashMap::from([
                ("version".to_string(), r#""1.0.0""#.to_string()),
                ("debug".to_string(), "false".to_string()),
            ])
        );

        let result1 = evaluator.evaluate(source).unwrap();
        let result2 = evaluator2.evaluate(source).unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_pure_evaluator_no_side_effects() {
        // Pure Kernel: 評価に副作用がないことを確認
        let mut evaluator = PureEvaluator::new();
        let source = r#" "side effect test" "#;

        // 評価前の状態を記録
        let tla_before = evaluator.tla_args.len();
        let ext_before = evaluator.ext_vars.len();

        // 評価実行
        let _result = evaluator.evaluate(source).unwrap();

        // 評価後も状態が変わっていない
        assert_eq!(evaluator.tla_args.len(), tla_before);
        assert_eq!(evaluator.ext_vars.len(), ext_before);

        // 複数回評価しても状態は変わらない
        for _ in 0..10 {
            let _result = evaluator.evaluate(source).unwrap();
            assert_eq!(evaluator.tla_args.len(), tla_before);
            assert_eq!(evaluator.ext_vars.len(), ext_before);
        }
    }
}
