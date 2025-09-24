//! Error types for Jsonnet evaluation

use thiserror::Error;

/// Result type for Jsonnet operations
pub type Result<T> = std::result::Result<T, JsonnetError>;

/// Jsonnet evaluation errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum JsonnetError {
    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Runtime error: {message}")]
    RuntimeError { message: String },

    #[error("Type error: {message}")]
    TypeError { message: String },

    #[error("Undefined variable: {name}")]
    UndefinedVariable { name: String },

    #[error("Undefined field: {field}")]
    UndefinedField { field: String },

    #[error("Index out of bounds: {index}")]
    IndexOutOfBounds { index: i64 },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid function call: {message}")]
    InvalidFunctionCall { message: String },

    #[error("Import error: {path}")]
    ImportError { path: String },

    #[error("IO error: {message}")]
    IoError { message: String },

    #[error("Assertion failed: {message}")]
    AssertionFailed { message: String },

    #[error("Stack overflow")]
    StackOverflow,

    #[error("Maximum recursion depth exceeded")]
    MaxRecursionExceeded,

    #[error("Invalid UTF-8 sequence")]
    InvalidUtf8,

    #[error("Regex error: {message}")]
    RegexError { message: String },
}

impl JsonnetError {
    /// Create a parse error
    pub fn parse_error(line: usize, column: usize, message: impl Into<String>) -> Self {
        JsonnetError::ParseError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a runtime error
    pub fn runtime_error(message: impl Into<String>) -> Self {
        JsonnetError::RuntimeError {
            message: message.into(),
        }
    }

    /// Create a type error
    pub fn type_error(message: impl Into<String>) -> Self {
        JsonnetError::TypeError {
            message: message.into(),
        }
    }

    /// Create an undefined variable error
    pub fn undefined_variable(name: impl Into<String>) -> Self {
        JsonnetError::UndefinedVariable {
            name: name.into(),
        }
    }

    /// Create an undefined field error
    pub fn undefined_field(field: impl Into<String>) -> Self {
        JsonnetError::UndefinedField {
            field: field.into(),
        }
    }

    /// Create an index out of bounds error
    pub fn index_out_of_bounds(index: i64) -> Self {
        JsonnetError::IndexOutOfBounds { index }
    }

    /// Create an invalid function call error
    pub fn invalid_function_call(message: impl Into<String>) -> Self {
        JsonnetError::InvalidFunctionCall {
            message: message.into(),
        }
    }

    /// Create an import error
    pub fn import_error(path: impl Into<String>) -> Self {
        JsonnetError::ImportError {
            path: path.into(),
        }
    }

    /// Create an IO error
    pub fn io_error(message: impl Into<String>) -> Self {
        JsonnetError::IoError {
            message: message.into(),
        }
    }

    /// Create an assertion failed error
    pub fn assertion_failed(message: impl Into<String>) -> Self {
        JsonnetError::AssertionFailed {
            message: message.into(),
        }
    }
}

impl From<std::io::Error> for JsonnetError {
    fn from(err: std::io::Error) -> Self {
        JsonnetError::io_error(err.to_string())
    }
}

impl From<serde_json::Error> for JsonnetError {
    fn from(err: serde_json::Error) -> Self {
        JsonnetError::runtime_error(format!("JSON serialization error: {}", err))
    }
}

impl From<regex::Error> for JsonnetError {
    fn from(err: regex::Error) -> Self {
        JsonnetError::RegexError {
            message: err.to_string(),
        }
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for JsonnetError {
    fn from(err: serde_yaml::Error) -> Self {
        JsonnetError::runtime_error(format!("YAML serialization error: {}", err))
    }
}
