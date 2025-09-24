//! Jsonnet value representation

use crate::ast::Expr;
use crate::error::{JsonnetError, Result};
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, ser::{SerializeSeq, SerializeMap}};

/// Jsonnet value types
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum JsonnetValue {
    /// Null value
    #[default]
    Null,
    /// Boolean value
    Boolean(bool),
    /// Number value (f64)
    Number(f64),
    /// String value
    String(String),
    /// Array value
    Array(Vec<JsonnetValue>),
    /// Object value
    Object(HashMap<String, JsonnetValue>),
    /// Function value
    Function(JsonnetFunction),
    /// Builtin function value
    Builtin(JsonnetBuiltin),
}

impl JsonnetValue {
    /// Create a null value
    pub fn null() -> Self {
        JsonnetValue::Null
    }

    /// Create a boolean value
    pub fn boolean(b: bool) -> Self {
        JsonnetValue::Boolean(b)
    }

    /// Create a number value
    pub fn number(n: f64) -> Self {
        JsonnetValue::Number(n)
    }

    /// Create a string value
    pub fn string(s: impl Into<String>) -> Self {
        JsonnetValue::String(s.into())
    }

    /// Create an array value
    pub fn array(values: Vec<JsonnetValue>) -> Self {
        JsonnetValue::Array(values)
    }

    /// Create an object value
    pub fn object(fields: HashMap<String, JsonnetValue>) -> Self {
        JsonnetValue::Object(fields)
    }

    /// Check if the value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            JsonnetValue::Null => false,
            JsonnetValue::Boolean(b) => *b,
            JsonnetValue::Number(n) => *n != 0.0,
            JsonnetValue::String(s) => !s.is_empty(),
            JsonnetValue::Array(a) => !a.is_empty(),
            JsonnetValue::Object(o) => !o.is_empty(),
            JsonnetValue::Function(_) => true,
            JsonnetValue::Builtin(_) => true,
        }
    }

    /// Get the type name of this value
    pub fn type_name(&self) -> &'static str {
        match self {
            JsonnetValue::Null => "null",
            JsonnetValue::Boolean(_) => "boolean",
            JsonnetValue::Number(_) => "number",
            JsonnetValue::String(_) => "string",
            JsonnetValue::Array(_) => "array",
            JsonnetValue::Object(_) => "object",
            JsonnetValue::Function(_) => "function",
            JsonnetValue::Builtin(_) => "function",
        }
    }

    /// Convert to serde_json::Value for serialization
    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            JsonnetValue::Null => serde_json::Value::Null,
            JsonnetValue::Boolean(b) => serde_json::Value::Bool(*b),
            JsonnetValue::Number(n) => serde_json::json!(*n),
            JsonnetValue::String(s) => serde_json::Value::String(s.clone()),
            JsonnetValue::Array(arr) => {
                let json_arr: Vec<serde_json::Value> = arr.iter().map(|v| v.to_json_value()).collect();
                serde_json::Value::Array(json_arr)
            }
            JsonnetValue::Object(obj) => {
                let mut json_obj = serde_json::Map::new();
                for (k, v) in obj {
                    json_obj.insert(k.clone(), v.to_json_value());
                }
                serde_json::Value::Object(json_obj)
            }
            JsonnetValue::Function(_) => {
                serde_json::Value::Null
            }
            JsonnetValue::Builtin(_) => {
                // Functions cannot be serialized to JSON
                serde_json::Value::String("<function>".to_string())
            }
        }
    }

    /// Try to convert to a string
    pub fn as_string(&self) -> Result<&str> {
        match self {
            JsonnetValue::String(s) => Ok(s),
            _ => Err(JsonnetError::type_error(format!("Expected string, got {}", self.type_name()))),
        }
    }

    /// Try to convert to a number
    pub fn as_number(&self) -> Result<f64> {
        match self {
            JsonnetValue::Number(n) => Ok(*n),
            _ => Err(JsonnetError::type_error(format!("Expected number, got {}", self.type_name()))),
        }
    }

    /// Try to convert to a boolean
    pub fn as_boolean(&self) -> Result<bool> {
        match self {
            JsonnetValue::Boolean(b) => Ok(*b),
            _ => Err(JsonnetError::type_error(format!("Expected boolean, got {}", self.type_name()))),
        }
    }

    /// Try to convert to an array
    pub fn as_array(&self) -> Result<&Vec<JsonnetValue>> {
        match self {
            JsonnetValue::Array(arr) => Ok(arr),
            _ => Err(JsonnetError::type_error(format!("Expected array, got {}", self.type_name()))),
        }
    }

    /// Try to convert to an object
    pub fn as_object(&self) -> Result<&HashMap<String, JsonnetValue>> {
        match self {
            JsonnetValue::Object(obj) => Ok(obj),
            _ => Err(JsonnetError::type_error(format!("Expected object, got {}", self.type_name()))),
        }
    }

    /// Get a field from an object
    pub fn get_field(&self, field: &str) -> Result<&JsonnetValue> {
        match self {
            JsonnetValue::Object(obj) => {
                obj.get(field)
                    .ok_or_else(|| JsonnetError::undefined_field(field.to_string()))
            }
            _ => Err(JsonnetError::type_error(format!("Expected object, got {}", self.type_name()))),
        }
    }

    /// Get an element from an array by index
    pub fn get_index(&self, index: i64) -> Result<&JsonnetValue> {
        match self {
            JsonnetValue::Array(arr) => {
                let idx = if index < 0 {
                    (arr.len() as i64 + index) as usize
                } else {
                    index as usize
                };

                arr.get(idx)
                    .ok_or_else(|| JsonnetError::index_out_of_bounds(index))
            }
            _ => Err(JsonnetError::type_error(format!("Expected array, got {}", self.type_name()))),
        }
    }

    /// Check if two values are equal
    pub fn equals(&self, other: &JsonnetValue) -> bool {
        match (self, other) {
            (JsonnetValue::Null, JsonnetValue::Null) => true,
            (JsonnetValue::Boolean(a), JsonnetValue::Boolean(b)) => a == b,
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => (a - b).abs() < f64::EPSILON,
            (JsonnetValue::String(a), JsonnetValue::String(b)) => a == b,
            (JsonnetValue::Array(a), JsonnetValue::Array(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.equals(y))
            }
            (JsonnetValue::Object(a), JsonnetValue::Object(b)) => {
                a.len() == b.len() && a.iter().all(|(k, v)| {
                    b.get(k).is_some_and(|bv| v.equals(bv))
                })
            }
            _ => false,
        }
    }

    // Binary operations
    pub fn add(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        match (self, other) {
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => Ok(JsonnetValue::number(a + b)),
            (JsonnetValue::String(a), JsonnetValue::String(b)) => Ok(JsonnetValue::string(format!("{}{}", a, b))),
            (JsonnetValue::Array(a), JsonnetValue::Array(b)) => {
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(JsonnetValue::Array(result))
            }
            _ => Err(JsonnetError::runtime_error("Cannot add these types")),
        }
    }

    pub fn sub(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        match (self, other) {
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => Ok(JsonnetValue::number(a - b)),
            _ => Err(JsonnetError::runtime_error("Cannot subtract these types")),
        }
    }

    pub fn mul(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        match (self, other) {
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => Ok(JsonnetValue::number(a * b)),
            _ => Err(JsonnetError::runtime_error("Cannot multiply these types")),
        }
    }

    pub fn div(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        match (self, other) {
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => {
                if *b == 0.0 {
                    Err(JsonnetError::runtime_error("Division by zero"))
                } else {
                    Ok(JsonnetValue::number(a / b))
                }
            }
            _ => Err(JsonnetError::runtime_error("Cannot divide these types")),
        }
    }

    pub fn modulo(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        match (self, other) {
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => {
                if *b == 0.0 {
                    Err(JsonnetError::runtime_error("Modulo by zero"))
                } else {
                    Ok(JsonnetValue::number(a % b))
                }
            }
            _ => Err(JsonnetError::runtime_error("Cannot modulo these types")),
        }
    }

    // Comparison operations
    pub fn lt(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        match (self, other) {
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => Ok(JsonnetValue::boolean(a < b)),
            (JsonnetValue::String(a), JsonnetValue::String(b)) => Ok(JsonnetValue::boolean(a < b)),
            _ => Err(JsonnetError::runtime_error("Cannot compare these types")),
        }
    }

    pub fn le(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        match (self, other) {
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => Ok(JsonnetValue::boolean(a <= b)),
            (JsonnetValue::String(a), JsonnetValue::String(b)) => Ok(JsonnetValue::boolean(a <= b)),
            _ => Err(JsonnetError::runtime_error("Cannot compare these types")),
        }
    }

    pub fn gt(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        match (self, other) {
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => Ok(JsonnetValue::boolean(a > b)),
            (JsonnetValue::String(a), JsonnetValue::String(b)) => Ok(JsonnetValue::boolean(a > b)),
            _ => Err(JsonnetError::runtime_error("Cannot compare these types")),
        }
    }

    pub fn ge(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        match (self, other) {
            (JsonnetValue::Number(a), JsonnetValue::Number(b)) => Ok(JsonnetValue::boolean(a >= b)),
            (JsonnetValue::String(a), JsonnetValue::String(b)) => Ok(JsonnetValue::boolean(a >= b)),
            _ => Err(JsonnetError::runtime_error("Cannot compare these types")),
        }
    }

    pub fn eq(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        Ok(JsonnetValue::boolean(self.equals(other)))
    }

    pub fn ne(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        Ok(JsonnetValue::boolean(!self.equals(other)))
    }

    // Logical operations
    pub fn and(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        if self.is_truthy() {
            Ok(other.clone())
        } else {
            Ok(self.clone())
        }
    }

    pub fn or(&self, other: &JsonnetValue) -> Result<JsonnetValue> {
        if self.is_truthy() {
            Ok(self.clone())
        } else {
            Ok(other.clone())
        }
    }

    // Unary operations
    pub fn not(&self) -> Result<JsonnetValue> {
        Ok(JsonnetValue::boolean(!self.is_truthy()))
    }

    pub fn neg(&self) -> Result<JsonnetValue> {
        match self {
            JsonnetValue::Number(n) => Ok(JsonnetValue::number(-n)),
            _ => Err(JsonnetError::runtime_error("Cannot negate this type")),
        }
    }
}

impl fmt::Display for JsonnetValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonnetValue::Null => write!(f, "null"),
            JsonnetValue::Boolean(b) => write!(f, "{}", b),
            JsonnetValue::Number(n) => write!(f, "{}", n),
            JsonnetValue::String(s) => write!(f, "{:?}", s),
            JsonnetValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonnetValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
            JsonnetValue::Function(_) => write!(f, "<function>"),
            JsonnetValue::Builtin(_) => write!(f, "<builtin>"),
        }
    }
}


/// Jsonnet builtin function types
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum JsonnetBuiltin {
    Length,
    StdLibFunction(String),
    // Add more builtins as needed
}

impl JsonnetBuiltin {
    pub fn call(&self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        match self {
            JsonnetBuiltin::Length => crate::stdlib::StdLib::length(args),
            JsonnetBuiltin::StdLibFunction(func_name) => crate::stdlib::StdLib::call_function(func_name, args),
        }
    }

    pub fn call_with_callback(&self, callback: &mut dyn crate::stdlib::FunctionCallback, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        match self {
            JsonnetBuiltin::Length => crate::stdlib::StdLib::length(args),
            JsonnetBuiltin::StdLibFunction(func_name) => {
                let mut stdlib_with_callback = crate::stdlib::StdLibWithCallback::new(callback);
                stdlib_with_callback.call_function(func_name, args)
            }
        }
    }
}

/// Jsonnet function representation
#[derive(Debug, Clone)]
pub struct JsonnetFunction {
    pub parameters: Vec<String>,
    pub body: Box<Expr>,
    pub environment: HashMap<String, JsonnetValue>,
}

impl PartialEq for JsonnetFunction {
    fn eq(&self, _other: &JsonnetFunction) -> bool {
        // Functions are never equal for comparison purposes
        false
    }
}

impl JsonnetFunction {
    /// Create a new function
    pub fn new(parameters: Vec<String>, body: Box<Expr>, environment: HashMap<String, JsonnetValue>) -> Self {
        JsonnetFunction {
            parameters,
            body,
            environment,
        }
    }
}

impl Serialize for JsonnetValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            JsonnetValue::Null => serializer.serialize_none(),
            JsonnetValue::Boolean(b) => serializer.serialize_bool(*b),
            JsonnetValue::Number(n) => serializer.serialize_f64(*n),
            JsonnetValue::String(s) => serializer.serialize_str(s),
            JsonnetValue::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in arr {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            JsonnetValue::Object(obj) => {
                let mut map = serializer.serialize_map(Some(obj.len()))?;
                for (key, value) in obj {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
            JsonnetValue::Function(_) => {
                // Functions serialize as a placeholder object
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "function")?;
                map.end()
            }
            JsonnetValue::Builtin(builtin) => {
                // Builtins serialize as a placeholder object
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "builtin")?;
                let name = match builtin {
                    JsonnetBuiltin::Length => "length",
                    JsonnetBuiltin::StdLibFunction(func_name) => func_name,
                };
                map.serialize_entry("name", name)?;
                map.end()
            }
        }
    }
}

