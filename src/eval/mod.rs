//! Evaluation components for Jsonnet
//!
//! This module contains the core evaluation infrastructure that can be
//! extended through handler-based architecture.

pub mod context;
pub mod handlers;

pub use context::{Context, Scope};
pub use handlers::*;
