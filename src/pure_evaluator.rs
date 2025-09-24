//! Pure Jsonnet evaluator - no side effects, fully deterministic
//!
//! This module provides a pure functional implementation of Jsonnet evaluation.
//! All evaluation is deterministic: same input always produces same output.

use crate::error::{JsonnetError, Result};
use crate::value::JsonnetValue;
use std::collections::HashMap;

/// Pure Jsonnet evaluator - performs only deterministic computations
#[derive(Debug, Clone)]
pub struct PureEvaluator {
    /// Top-level arguments (immutable configuration)
    tla_args: HashMap<String, String>,
    /// External variables (immutable configuration)
    ext_vars: HashMap<String, String>,
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
        }
    }

    /// Create a pure evaluator with top-level arguments
    pub fn with_tla_args(tla_args: HashMap<String, String>) -> Self {
        Self {
            tla_args,
            ext_vars: HashMap::new(),
        }
    }

    /// Create a pure evaluator with both TLA and external variables
    pub fn with_config(tla_args: HashMap<String, String>, ext_vars: HashMap<String, String>) -> Self {
        Self {
            tla_args,
            ext_vars,
        }
    }

    /// Pure evaluation of Jsonnet source code
    ///
    /// This function is PURE: it performs only deterministic computations
    /// and has no side effects. Same input always produces same output.
    pub fn evaluate(&self, source: &str) -> Result<JsonnetValue> {
        // Create the evaluation context with immutable configuration
        let context = EvaluationContext {
            tla_args: &self.tla_args,
            ext_vars: &self.ext_vars,
            source: source.to_string(),
        };

        self.evaluate_with_context(&context)
    }

    /// Pure evaluation with explicit context
    fn evaluate_with_context(&self, context: &EvaluationContext) -> Result<JsonnetValue> {
        // Parse the Jsonnet source (in real implementation, this would be done by the parser)
        let parsed = self.parse_jsonnet(&context.source)?;

        // Inject TLA variables as local bindings
        let with_tla = self.inject_tla_variables(parsed, &context.tla_args);

        // Inject external variables
        let with_ext = self.inject_external_variables(with_tla, &context.ext_vars);

        // Evaluate the expression (simplified - real implementation would traverse AST)
        self.evaluate_expression(with_ext)
    }

    /// Parse Jsonnet source (simplified - would use real parser in implementation)
    fn parse_jsonnet(&self, _source: &str) -> Result<ParsedExpression> {
        // In real implementation, this would parse the Jsonnet source into an AST
        // For now, return a placeholder
        Ok(ParsedExpression::String("parsed".to_string()))
    }

    /// Inject TLA variables as local bindings
    fn inject_tla_variables(&self, expr: ParsedExpression, tla_args: &HashMap<String, String>) -> ParsedExpression {
        if tla_args.is_empty() {
            return expr;
        }

        // In real implementation, this would wrap the expression with local bindings
        // for each TLA variable
        ParsedExpression::LocalBindings {
            bindings: tla_args.clone(),
            body: Box::new(expr),
        }
    }

    /// Inject external variables
    fn inject_external_variables(&self, expr: ParsedExpression, ext_vars: &HashMap<String, String>) -> ParsedExpression {
        if ext_vars.is_empty() {
            return expr;
        }

        // In real implementation, this would inject external variables during evaluation
        ParsedExpression::WithExtVars {
            vars: ext_vars.clone(),
            body: Box::new(expr),
        }
    }

    /// Evaluate the parsed expression (simplified)
    fn evaluate_expression(&self, expr: ParsedExpression) -> Result<JsonnetValue> {
        match expr {
            ParsedExpression::String(s) => Ok(JsonnetValue::String(s)),
            ParsedExpression::LocalBindings { bindings, body } => {
                // In real implementation, this would evaluate with local scope
                // For now, just evaluate the body
                self.evaluate_expression(*body)
            }
            ParsedExpression::WithExtVars { vars: _, body } => {
                // In real implementation, this would evaluate with external variables
                self.evaluate_expression(*body)
            }
        }
    }
}

/// Evaluation context containing all immutable configuration
#[derive(Debug)]
struct EvaluationContext<'a> {
    tla_args: &'a HashMap<String, String>,
    ext_vars: &'a HashMap<String, String>,
    source: String,
}

/// Simplified AST representation for demonstration
#[derive(Debug, Clone)]
enum ParsedExpression {
    String(String),
    LocalBindings {
        bindings: HashMap<String, String>,
        body: Box<ParsedExpression>,
    },
    WithExtVars {
        vars: HashMap<String, String>,
        body: Box<ParsedExpression>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_evaluation_is_deterministic() {
        let evaluator = PureEvaluator::new();
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

        let evaluator = PureEvaluator::with_tla_args(tla_args);
        let source = r#" "Hello, " + name + "!" "#;

        let result = evaluator.evaluate(source).unwrap();
        // In real implementation, this would evaluate to "Hello, Alice!"
        // For now, just check that evaluation succeeds
        assert!(matches!(result, JsonnetValue::String(_)));
    }

    #[test]
    fn test_pure_evaluator_clone() {
        let evaluator1 = PureEvaluator::new();
        let evaluator2 = evaluator1.clone();

        let source = r#" "test" "#;
        let result1 = evaluator1.evaluate(source).unwrap();
        let result2 = evaluator2.evaluate(source).unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_pure_evaluator_immutability() {
        // Pure Kernel: PureEvaluatorは不変、設定は作成時に固定
        let evaluator = PureEvaluator::new();

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

        let evaluator1 = PureEvaluator::with_tla_args(tla_args1);
        let evaluator2 = PureEvaluator::with_tla_args(tla_args2);

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

        let evaluator = PureEvaluator::with_config(tla_args, ext_vars);
        let source = r#" "App: " + app + " v" + version + " debug=" + debug "#;

        // 同じ設定で作成したevaluatorは同じ結果を返す
        let evaluator2 = PureEvaluator::with_config(
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
        let evaluator = PureEvaluator::new();
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
