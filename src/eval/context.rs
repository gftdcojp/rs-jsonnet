//! Evaluation context for managing variable scopes and evaluation state

use crate::error::{JsonnetError, Result};
use crate::value::JsonnetValue;
use std::collections::HashMap;

/// A single variable scope
#[derive(Debug, Clone)]
pub struct Scope {
    variables: HashMap<String, JsonnetValue>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&JsonnetValue> {
        self.variables.get(name)
    }

    pub fn set(&mut self, name: String, value: JsonnetValue) {
        self.variables.insert(name, value);
    }

    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn variables(&self) -> &HashMap<String, JsonnetValue> {
        &self.variables
    }
}

/// Evaluation context that manages variable scopes and evaluation state
#[derive(Debug)]
pub struct Context {
    /// Global scope for top-level variables and functions
    global_scope: Scope,
    /// Stack of local scopes for function calls and blocks
    local_scopes: Vec<Scope>,
    /// Current evaluation depth (for recursion detection)
    depth: usize,
    /// Maximum allowed evaluation depth
    max_depth: usize,
}

impl Context {
    pub fn new() -> Self {
        Context {
            global_scope: Scope::new(),
            local_scopes: Vec::new(),
            depth: 0,
            max_depth: 100, // Default recursion limit
        }
    }

    pub fn with_max_depth(max_depth: usize) -> Self {
        Context {
            global_scope: Scope::new(),
            local_scopes: Vec::new(),
            depth: 0,
            max_depth,
        }
    }

    /// Get the current evaluation depth
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Check if we've exceeded the maximum depth
    pub fn check_depth(&self) -> Result<()> {
        if self.depth >= self.max_depth {
            return Err(JsonnetError::runtime_error(
                format!("Maximum evaluation depth ({}) exceeded", self.max_depth)
            ));
        }
        Ok(())
    }

    /// Increment evaluation depth
    pub fn push_depth(&mut self) -> Result<()> {
        self.depth += 1;
        self.check_depth()
    }

    /// Decrement evaluation depth
    pub fn pop_depth(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }

    /// Get a variable by name, searching from local to global scope
    pub fn get_variable(&self, name: &str) -> Option<&JsonnetValue> {
        // First check local scopes (from innermost to outermost)
        for scope in self.local_scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        // Then check global scope
        self.global_scope.get(name)
    }

    /// Set a variable in the current scope (local if available, otherwise global)
    pub fn set_variable(&mut self, name: String, value: JsonnetValue) {
        if let Some(scope) = self.local_scopes.last_mut() {
            scope.set(name, value);
        } else {
            self.global_scope.set(name, value);
        }
    }

    /// Set a variable in the global scope specifically
    pub fn set_global(&mut self, name: String, value: JsonnetValue) {
        self.global_scope.set(name, value);
    }

    /// Check if a variable exists in any scope
    pub fn has_variable(&self, name: &str) -> bool {
        // Check local scopes first
        for scope in self.local_scopes.iter().rev() {
            if scope.contains(name) {
                return true;
            }
        }
        // Then check global scope
        self.global_scope.contains(name)
    }

    /// Push a new local scope
    pub fn push_scope(&mut self) {
        self.local_scopes.push(Scope::new());
    }

    /// Pop the current local scope
    pub fn pop_scope(&mut self) -> Option<Scope> {
        self.local_scopes.pop()
    }

    /// Get the current scope (for setting variables)
    pub fn current_scope(&mut self) -> Option<&mut Scope> {
        self.local_scopes.last_mut()
    }

    /// Get the global scope
    pub fn global_scope(&self) -> &Scope {
        &self.global_scope
    }

    /// Get the global scope mutably
    pub fn global_scope_mut(&mut self) -> &mut Scope {
        &mut self.global_scope
    }

    /// Get all variables from all scopes (for debugging)
    pub fn all_variables(&self) -> HashMap<String, &JsonnetValue> {
        let mut result = HashMap::new();

        // Add global variables
        for (name, value) in self.global_scope.variables() {
            result.insert(name.clone(), value);
        }

        // Add local variables (later scopes override earlier ones)
        for scope in &self.local_scopes {
            for (name, value) in scope.variables() {
                result.insert(name.clone(), value);
            }
        }

        result
    }

    /// Create a new context with the same global scope but empty local scopes
    pub fn fork(&self) -> Self {
        Context {
            global_scope: self.global_scope.clone(),
            local_scopes: Vec::new(),
            depth: 0,
            max_depth: self.max_depth,
        }
    }

    /// Merge local scopes back into global scope (for top-level evaluation)
    pub fn merge_locals_to_global(&mut self) {
        for scope in &self.local_scopes {
            for (name, value) in scope.variables() {
                self.global_scope.set(name.clone(), value.clone());
            }
        }
        self.local_scopes.clear();
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
