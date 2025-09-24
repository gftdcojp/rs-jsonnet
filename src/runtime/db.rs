//! Database handler for Jsonnet evaluator
//!
//! Provides std.ext.db functions for database operations:
//! - std.ext.db.query: Execute GQL queries
//! - std.ext.db.rewrite: Execute graph rewrite rules
//! - std.ext.db.patch: Apply patches to the graph

use crate::error::JsonnetError;
use crate::value::JsonnetValue;

/// Database handler trait for external database operations
pub trait DatabaseHandler {
    /// Execute a GQL query and return results
    fn query(&self, query: &str, params: Option<&JsonnetValue>) -> Result<JsonnetValue, JsonnetError>;

    /// Execute a graph rewrite rule
    fn rewrite(&self, rule: &str, params: Option<&JsonnetValue>) -> Result<JsonnetValue, JsonnetError>;

    /// Apply a patch to the graph
    fn patch(&self, patch: &JsonnetValue) -> Result<JsonnetValue, JsonnetError>;
}

/// Default database handler that returns errors for unimplemented operations
pub struct DefaultDatabaseHandler;

impl DatabaseHandler for DefaultDatabaseHandler {
    fn query(&self, _query: &str, _params: Option<&JsonnetValue>) -> Result<JsonnetValue, JsonnetError> {
        Err(JsonnetError::runtime_error("Database operations not implemented"))
    }

    fn rewrite(&self, _rule: &str, _params: Option<&JsonnetValue>) -> Result<JsonnetValue, JsonnetError> {
        Err(JsonnetError::runtime_error("Rewrite operations not implemented"))
    }

    fn patch(&self, _patch: &JsonnetValue) -> Result<JsonnetValue, JsonnetError> {
        Err(JsonnetError::runtime_error("Patch operations not implemented"))
    }
}
