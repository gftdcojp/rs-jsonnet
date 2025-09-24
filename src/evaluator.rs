//! Effects Shell Jsonnet evaluator
//!
//! This module provides the Effects Shell wrapper around the Pure Jsonnet evaluator.
//! It handles I/O operations, external library loading, and mutable state management.

use crate::error::{JsonnetError, Result};
use crate::pure_evaluator::PureEvaluator;
use crate::value::JsonnetValue;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Effects Shell evaluator - handles I/O and mutable state
pub struct Evaluator {
    /// Pure evaluator instance (immutable after creation)
    pure_evaluator: PureEvaluator,
    /// Mutable TLA arguments (effects: can be modified)
    tla_args: HashMap<String, String>,
    /// External variables loaded from files (effects: file I/O)
    ext_vars: HashMap<String, String>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    /// Create a new effects-based evaluator
    pub fn new() -> Self {
        Self {
            pure_evaluator: PureEvaluator::new(),
            tla_args: HashMap::new(),
            ext_vars: HashMap::new(),
        }
    }

    /// Add a top-level argument (effects: modifies internal state)
    pub fn add_tla_code(&mut self, key: &str, value: &str) {
        self.tla_args.insert(key.to_string(), value.to_string());
        // Update the pure evaluator with new TLA args
        self.update_pure_evaluator();
    }

    /// Add multiple TLA arguments at once
    pub fn add_tla_args(&mut self, args: HashMap<String, String>) {
        self.tla_args.extend(args);
        self.update_pure_evaluator();
    }

    /// Load external variables from a file (effects: file I/O)
    pub fn load_ext_vars_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let content = fs::read_to_string(path)
            .map_err(|e| JsonnetError::IoError(format!("Failed to read ext vars file: {}", e)))?;

        // Parse as JSON (simplified - real implementation would handle Jsonnet format)
        let vars: HashMap<String, String> = serde_json::from_str(&content)
            .map_err(|e| JsonnetError::ParseError(format!("Invalid ext vars JSON: {}", e)))?;

        self.ext_vars.extend(vars);
        self.update_pure_evaluator();
        Ok(())
    }

    /// Evaluate a Jsonnet expression (effects: may involve external libraries)
    pub fn evaluate(&mut self, source: &str) -> Result<JsonnetValue> {
        // Use the pure evaluator for the actual computation
        self.pure_evaluator.evaluate(source)
    }

    /// Evaluate a Jsonnet file (effects: filename handling)
    pub fn evaluate_file(&mut self, source: &str, _filename: &str) -> Result<JsonnetValue> {
        self.evaluate(source)
    }

    /// Evaluate a file from disk (effects: file I/O)
    pub fn evaluate_file_from_path<P: AsRef<Path>>(&mut self, path: P) -> Result<JsonnetValue> {
        let source = fs::read_to_string(path)
            .map_err(|e| JsonnetError::IoError(format!("Failed to read file: {}", e)))?;

        self.evaluate(&source)
    }

    /// Get current TLA arguments (for debugging)
    pub fn get_tla_args(&self) -> &HashMap<String, String> {
        &self.tla_args
    }

    /// Get current external variables (for debugging)
    pub fn get_ext_vars(&self) -> &HashMap<String, String> {
        &self.ext_vars
    }

    /// Update the pure evaluator with current configuration
    fn update_pure_evaluator(&mut self) {
        self.pure_evaluator = PureEvaluator::with_config(
            self.tla_args.clone(),
            self.ext_vars.clone(),
        );
    }
}

/// Convenience functions for pure evaluation (no side effects)
pub mod pure {
    use super::*;

    /// Evaluate Jsonnet source with pure semantics
    pub fn evaluate(source: &str) -> Result<JsonnetValue> {
        PureEvaluator::new().evaluate(source)
    }

    /// Evaluate Jsonnet source with TLA arguments (pure)
    pub fn evaluate_with_tla(source: &str, tla_args: HashMap<String, String>) -> Result<JsonnetValue> {
        PureEvaluator::with_tla_args(tla_args).evaluate(source)
    }
}