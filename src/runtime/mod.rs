//! Runtime components for external function execution
//!
//! This module provides handlers for external functions like HTTP calls,
//! AI API calls, system commands, etc.

pub mod db;
pub mod external;
pub mod manager;

pub use db::*;
pub use external::*;
pub use manager::*;
