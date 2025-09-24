//! Runtime manager for coordinating external function handlers

use crate::error::Result;
use crate::value::JsonnetValue;

/// Simple runtime manager (placeholder implementation)
pub struct RuntimeManager {
    // Placeholder - will be expanded later
}

impl RuntimeManager {
    /// Create a new runtime manager
    pub fn new() -> Self {
        RuntimeManager {}
    }

    /// Check if a function name belongs to an external handler
    pub fn is_external_function(&self, _name: &str) -> bool {
        // Placeholder - always return false for now
        false
    }

    /// Call an external function (placeholder)
    pub fn call_external_function(&self, name: &str, _args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        // Placeholder - return error for unimplemented functions
        Err(crate::error::JsonnetError::runtime_error(format!("External function '{}' not implemented", name)))
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new()
    }
}
