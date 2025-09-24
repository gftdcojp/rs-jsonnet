//! Jsonnet standard library implementation

use crate::error::{JsonnetError, Result};
use crate::value::JsonnetValue;
use serde_json::json;

/// Callback trait for function calling from stdlib
pub trait FunctionCallback {
    fn call_function(&mut self, func: JsonnetValue, args: Vec<JsonnetValue>) -> Result<JsonnetValue>;
    fn call_external_function(&mut self, func: &str, args: Vec<JsonnetValue>) -> Result<JsonnetValue>;
}
use sha1::Sha1;
use sha2::{Sha256, Sha512, Digest};
use sha3::Sha3_256;
use std::collections::HashMap;

/// Standard library function implementations
pub struct StdLib;

/// Standard library with function callback support
pub struct StdLibWithCallback<'a> {
    callback: &'a mut dyn FunctionCallback,
}

impl<'a> StdLibWithCallback<'a> {
    pub fn new(callback: &'a mut dyn FunctionCallback) -> Self {
        StdLibWithCallback { callback }
    }

    /// Call a standard library function with function callback support
    pub fn call_function(&mut self, name: &str, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        match name {
            "length" => StdLib::length(args),
            "type" => StdLib::type_of(args),
            "makeArray" => StdLib::make_array(args),
            "filter" => self.filter(args),
            "map" => self.map(args),
            "foldl" => self.foldl(args),
            "foldr" => self.foldr(args),
            // ... other functions that don't need callback
            "range" => StdLib::range(args),
            "join" => self.join_variadic(args),
            "split" => StdLib::split(args),
            "contains" => self.contains_variadic(args),
            "startsWith" => StdLib::starts_with(args),
            "endsWith" => StdLib::ends_with(args),
            "toLower" => StdLib::to_lower(args),
            "toUpper" => StdLib::to_upper(args),
            "trim" => StdLib::trim(args),
            "substr" => StdLib::substr(args),
            "char" => StdLib::char_fn(args),
            "codepoint" => StdLib::codepoint(args),
            "toString" => StdLib::to_string(args),
            "parseInt" => StdLib::parse_int(args),
            "parseJson" => StdLib::parse_json(args),
            "encodeUTF8" => StdLib::encode_utf8(args),
            "decodeUTF8" => StdLib::decode_utf8(args),
            "md5" => StdLib::md5(args),
            "base64" => StdLib::base64(args),
            "base64Decode" => StdLib::base64_decode(args),
            "manifestJson" => StdLib::manifest_json(args),
            "manifestJsonEx" => StdLib::manifest_json_ex(args),
            "manifestYaml" => StdLib::manifest_yaml(args),
            "escapeStringJson" => StdLib::escape_string_json(args),
            "escapeStringYaml" => StdLib::escape_string_yaml(args),
            "escapeStringPython" => StdLib::escape_string_python(args),
            "escapeStringBash" => StdLib::escape_string_bash(args),
            "escapeStringDollars" => StdLib::escape_string_dollars(args),
            "stringChars" => StdLib::string_chars(args),
            "stringBytes" => StdLib::string_bytes(args),
            "format" => StdLib::format(args),
            "isArray" => StdLib::is_array(args),
            "isBoolean" => StdLib::is_boolean(args),
            "isFunction" => StdLib::is_function(args),
            "isNumber" => StdLib::is_number(args),
            "isObject" => StdLib::is_object(args),
            "isString" => StdLib::is_string(args),
            "count" => StdLib::count(args),
            "find" => StdLib::find(args),
            "member" => StdLib::member(args),
            "modulo" => StdLib::modulo(args),
            "pow" => StdLib::pow(args),
            "exp" => StdLib::exp(args),
            "log" => StdLib::log(args),
            "sqrt" => StdLib::sqrt(args),
            "sin" => StdLib::sin(args),
            "cos" => StdLib::cos(args),
            "tan" => StdLib::tan(args),
            "asin" => StdLib::asin(args),
            "acos" => StdLib::acos(args),
            "atan" => StdLib::atan(args),
            "floor" => StdLib::floor(args),
            "ceil" => StdLib::ceil(args),
            "round" => StdLib::round(args),
            "abs" => StdLib::abs(args),
            "max" => StdLib::max(args),
            "min" => StdLib::min(args),
            "clamp" => StdLib::clamp(args),
            "assertEqual" => StdLib::assert_equal(args),
            "trace" => StdLib::trace(args),
            "sort" => StdLib::sort(args),
            "uniq" => StdLib::uniq(args),
            "reverse" => StdLib::reverse(args),
            "mergePatch" => StdLib::merge_patch(args),
            "get" => StdLib::get(args),
            "id" => StdLib::id(args),
            "equals" => StdLib::equals(args),
            "lines" => StdLib::lines(args),
            "strReplace" => StdLib::str_replace(args),
            "sha1" => StdLib::sha1(args),
            "sha256" => StdLib::sha256(args),
            "sha3" => StdLib::sha3(args),
            "sha512" => StdLib::sha512(args),
            "asciiLower" => StdLib::ascii_lower(args),
            "asciiUpper" => StdLib::ascii_upper(args),
            "set" => StdLib::set(args),
            "flatMap" => StdLib::flat_map(args),
            "mapWithIndex" => StdLib::map_with_index(args),
            "lstripChars" => StdLib::lstrip_chars(args),
            "rstripChars" => StdLib::rstrip_chars(args),
            "stripChars" => StdLib::strip_chars(args),
            "findSubstr" => StdLib::find_substr(args),
            "repeat" => StdLib::repeat(args),
            "setMember" => StdLib::set_member(args),
            "setUnion" => StdLib::set_union(args),
            "setInter" => StdLib::set_inter(args),
            "setDiff" => StdLib::set_diff(args),
            "objectFields" => StdLib::object_fields(args),
            "objectFieldsAll" => StdLib::object_fields_all(args),
            "objectHas" => StdLib::object_has(args),
            "objectHasAll" => StdLib::object_has_all(args),
            "objectValues" => StdLib::object_values(args),
            "objectValuesAll" => StdLib::object_values_all(args),
            "objectFieldsEx" => StdLib::object_fields_ex(args),
            "objectValuesEx" => StdLib::object_values_ex(args),
            "prune" => StdLib::prune(args),
            "mapWithKey" => StdLib::map_with_key(args),
            "manifestIni" => StdLib::manifest_ini(args),
            "manifestPython" => StdLib::manifest_python(args),
            "manifestCpp" => StdLib::manifest_cpp(args),
            "manifestXmlJsonml" => StdLib::manifest_xml_jsonml(args),
            "log2" => StdLib::log2(args),
            "log10" => StdLib::log10(args),
            "log1p" => StdLib::log1p(args),
            "expm1" => StdLib::expm1(args),
            "remove" => StdLib::remove(args),
            "removeAt" => StdLib::remove_at(args),
            "flattenArrays" => StdLib::flatten_arrays(args),
            "objectKeysValues" => StdLib::object_keys_values(args),
            "objectRemoveKey" => StdLib::object_remove_key(args),
            "isInteger" => StdLib::is_integer(args),
            "isDecimal" => StdLib::is_decimal(args),
            "isEven" => StdLib::is_even(args),
            "isOdd" => StdLib::is_odd(args),
            // New functions to implement
            "slice" => self.slice(args),
            "zip" => self.zip(args),
            "transpose" => self.transpose(args),
            "flatten" => self.flatten(args),
            "sum" => self.sum(args),
            "product" => self.product(args),
            "all" => self.all(args),
            "any" => self.any(args),
            "sortBy" => self.sort_by(args),
            "groupBy" => self.group_by(args),
            "partition" => self.partition(args),
            "chunk" => self.chunk(args),
            "unique" => self.unique(args),
            "difference" => self.difference(args),
            "intersection" => self.intersection(args),
            "symmetricDifference" => self.symmetric_difference(args),
            "isSubset" => self.is_subset(args),
            "isSuperset" => self.is_superset(args),
            "isDisjoint" => self.is_disjoint(args),
            "cartesian" => self.cartesian(args),
            "cross" => self.cross(args),
            "dot" => self.dot(args),
            "norm" => self.norm(args),
            "normalize" => self.normalize(args),
            "distance" => self.distance(args),
            "angle" => self.angle(args),
            "rotate" => self.rotate(args),
            "scale" => self.scale(args),
            "translate" => self.translate(args),
            "reflect" => self.reflect(args),
            "affine" => self.affine(args),
            "splitLimit" => self.split_limit(args),
            "replace" => self.replace(args),

            // ==========================================
            // AI Agent Functions (Manimani)
            // ==========================================

            // HTTP functions
            "ai.httpGet" => StdLib::ai_http_get(args),
            "ai.httpPost" => StdLib::ai_http_post(args),

            // AI model functions
            "ai.callModel" => StdLib::ai_call_model(args),

            // Tool functions
            "tool.execute" => StdLib::tool_execute(args),

            // Memory functions
            "memory.get" => StdLib::memory_get(args),
            "memory.set" => StdLib::memory_set(args),

            // Agent functions
            "agent.create" => StdLib::agent_create(args),
            "agent.execute" => self.callback.call_external_function(name, args),

            // Chain functions
            "chain.create" => self.callback.call_external_function(name, args),
            "chain.execute" => self.callback.call_external_function(name, args),
            "db.query" => self.callback.call_external_function(name, args),
            "db.rewrite" => self.callback.call_external_function(name, args),
            "db.patch" => self.callback.call_external_function(name, args),

            _ => Err(JsonnetError::runtime_error(format!(
                "Unknown function: {}",
                name
            ))),
        }
    }

    // Higher-order functions that use function callbacks
    fn filter(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "filter")?;
        let _func = &args[0];
        let arr = args[1].as_array()?;

        let mut result = Vec::new();
        for item in arr {
            // Call func(item) and check if result is truthy
            let call_result = self.callback.call_function(_func.clone(), vec![item.clone()])?;
            if call_result.is_truthy() {
                result.push(item.clone());
            }
        }

        Ok(JsonnetValue::array(result))
    }

    fn map(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "map")?;
        let _func = &args[0];
        let arr = args[1].as_array()?;

        let mut result = Vec::new();
        for item in arr {
            // Call func(item) and collect results
            let call_result = self.callback.call_function(_func.clone(), vec![item.clone()])?;
            result.push(call_result);
        }

        Ok(JsonnetValue::array(result))
    }

    fn foldl(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 3, "foldl")?;
        let _func = &args[0];
        let arr = args[1].as_array()?;
        let mut accumulator = args[2].clone();

        for item in arr {
            // Call func(accumulator, item)
            accumulator = self.callback.call_function(_func.clone(), vec![accumulator, item.clone()])?;
        }

        Ok(accumulator)
    }

    fn foldr(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 3, "foldr")?;
        let _func = &args[0];
        let arr = args[1].as_array()?;
        let mut accumulator = args[2].clone();

        for item in arr.iter().rev() {
            // Call func(item, accumulator)
            accumulator = self.callback.call_function(_func.clone(), vec![item.clone(), accumulator])?;
        }

        Ok(accumulator)
    }

    // New utility functions
    fn slice(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        if args.len() < 2 {
            return Err(JsonnetError::invalid_function_call("slice() expects at least 2 arguments".to_string()));
        }
        let start = args[1].as_number()? as usize;

        match &args[0] {
            JsonnetValue::Array(arr) => {
                let end = if args.len() > 2 {
                    args[2].as_number()? as usize
                } else {
                    arr.len()
                };
                let start = start.min(arr.len());
                let end = end.min(arr.len());
                if start > end {
                    Ok(JsonnetValue::array(vec![]))
                } else {
                    Ok(JsonnetValue::array(arr[start..end].to_vec()))
                }
            }
            JsonnetValue::String(s) => {
                let end = if args.len() > 2 {
                    args[2].as_number()? as usize
                } else {
                    s.chars().count()
                };
                let chars: Vec<char> = s.chars().collect();
                let start = start.min(chars.len());
                let end = end.min(chars.len());
                if start > end {
                    Ok(JsonnetValue::string("".to_string()))
                } else {
                    let sliced: String = chars[start..end].iter().collect();
                    Ok(JsonnetValue::string(sliced))
                }
            }
            _ => Err(JsonnetError::invalid_function_call("slice() expects array or string as first argument".to_string())),
        }
    }

    fn zip(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        if args.is_empty() {
            return Err(JsonnetError::invalid_function_call("zip() expects at least one argument".to_string()));
        }

        // Convert all arguments to arrays
        let arrays: Result<Vec<Vec<JsonnetValue>>> = args.into_iter()
            .map(|arg| arg.as_array().cloned())
            .collect();

        let arrays = arrays?;
        if arrays.is_empty() {
            return Ok(JsonnetValue::array(vec![]));
        }

        // Find minimum length
        let min_len = arrays.iter().map(|arr| arr.len()).min().unwrap_or(0);

        // Create zipped result
        let mut result = Vec::new();
        for i in 0..min_len {
            let mut tuple = Vec::new();
            for arr in &arrays {
                tuple.push(arr[i].clone());
            }
            result.push(JsonnetValue::array(tuple));
        }

        Ok(JsonnetValue::array(result))
    }

    fn transpose(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "transpose")?;
        let matrix = args[0].as_array()?;

        if matrix.is_empty() {
            return Ok(JsonnetValue::array(vec![]));
        }

        // Check if all elements are arrays and get dimensions
        let mut max_len = 0;
        for row in matrix {
            match row {
                JsonnetValue::Array(arr) => {
                    max_len = max_len.max(arr.len());
                }
                _ => return Err(JsonnetError::invalid_function_call("transpose() expects array of arrays".to_string())),
            }
        }

        if max_len == 0 {
            return Ok(JsonnetValue::array(vec![]));
        }

        // Create transposed matrix
        let mut result = Vec::new();
        for col in 0..max_len {
            let mut new_row = Vec::new();
            for row in matrix {
                if let JsonnetValue::Array(arr) = row {
                    if col < arr.len() {
                        new_row.push(arr[col].clone());
                    } else {
                        new_row.push(JsonnetValue::Null);
                    }
                }
            }
            result.push(JsonnetValue::array(new_row));
        }

        Ok(JsonnetValue::array(result))
    }

    fn flatten(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "flatten")?;
        let depth = if args.len() > 1 {
            args[1].as_number()? as usize
        } else {
            usize::MAX
        };

        fn flatten_recursive(arr: &Vec<JsonnetValue>, current_depth: usize, max_depth: usize) -> Vec<JsonnetValue> {
            let mut result = Vec::new();
            for item in arr {
                match item {
                    JsonnetValue::Array(nested) if current_depth < max_depth => {
                        result.extend(flatten_recursive(nested, current_depth + 1, max_depth));
                    }
                    _ => result.push(item.clone()),
                }
            }
            result
        }

        let arr = args[0].as_array()?;
        let flattened = flatten_recursive(arr, 0, depth);
        Ok(JsonnetValue::array(flattened))
    }

    fn sum(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "sum")?;
        let arr = args[0].as_array()?;

        let mut total = 0.0;
        for item in arr {
            total += item.as_number()?;
        }

        Ok(JsonnetValue::number(total))
    }

    fn product(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "product")?;
        let arr = args[0].as_array()?;

        let mut result = 1.0;
        for item in arr {
            result *= item.as_number()?;
        }

        Ok(JsonnetValue::number(result))
    }

    fn all(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "all")?;
        let arr = args[0].as_array()?;

        for item in arr {
            if !item.is_truthy() {
                return Ok(JsonnetValue::boolean(false));
            }
        }

        Ok(JsonnetValue::boolean(true))
    }

    fn any(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "any")?;
        let arr = args[0].as_array()?;

        for item in arr {
            if item.is_truthy() {
                return Ok(JsonnetValue::boolean(true));
            }
        }

        Ok(JsonnetValue::boolean(false))
    }

    fn chunk(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "chunk")?;
        let arr = args[0].as_array()?;
        let size = args[1].as_number()? as usize;

        if size == 0 {
            return Err(JsonnetError::invalid_function_call("chunk() size must be positive".to_string()));
        }

        let mut result = Vec::new();
        for chunk in arr.chunks(size) {
            result.push(JsonnetValue::array(chunk.to_vec()));
        }

        Ok(JsonnetValue::array(result))
    }

    fn unique(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "unique")?;
        let arr = args[0].as_array()?;

        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();

        for item in arr {
            // Simple equality check - in real Jsonnet this uses deep equality
            if !seen.contains(&format!("{:?}", item)) {
                seen.insert(format!("{:?}", item));
                result.push(item.clone());
            }
        }

        Ok(JsonnetValue::array(result))
    }

    fn difference(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        if args.is_empty() {
            return Ok(JsonnetValue::array(vec![]));
        }

        let first = args[0].as_array()?;
        let mut result = first.clone();

        for arg in &args[1..] {
            let other = arg.as_array()?;
            let other_set: std::collections::HashSet<String> = other.iter()
                .map(|v| format!("{:?}", v))
                .collect();

            result.retain(|item| !other_set.contains(&format!("{:?}", item)));
        }

        Ok(JsonnetValue::array(result))
    }

    fn intersection(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        if args.is_empty() {
            return Ok(JsonnetValue::array(vec![]));
        }

        let first = args[0].as_array()?;
        let mut result = first.clone();

        for arg in &args[1..] {
            let other = arg.as_array()?;
            let other_set: std::collections::HashSet<String> = other.iter()
                .map(|v| format!("{:?}", v))
                .collect();

            result.retain(|item| other_set.contains(&format!("{:?}", item)));
        }

        Ok(JsonnetValue::array(result))
    }

    fn symmetric_difference(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "symmetricDifference")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let a_set: std::collections::HashSet<String> = a.iter()
            .map(|v| format!("{:?}", v))
            .collect();
        let b_set: std::collections::HashSet<String> = b.iter()
            .map(|v| format!("{:?}", v))
            .collect();

        let symmetric_diff: std::collections::HashSet<_> = a_set.symmetric_difference(&b_set).cloned().collect();

        let result: Vec<JsonnetValue> = a.iter()
            .filter(|item| symmetric_diff.contains(&format!("{:?}", item)))
            .chain(b.iter().filter(|item| symmetric_diff.contains(&format!("{:?}", item))))
            .cloned()
            .collect();

        Ok(JsonnetValue::array(result))
    }

    fn is_subset(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "isSubset")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let b_set: std::collections::HashSet<String> = b.iter()
            .map(|v| format!("{:?}", v))
            .collect();

        let is_subset = a.iter().all(|item| b_set.contains(&format!("{:?}", item)));

        Ok(JsonnetValue::boolean(is_subset))
    }

    fn is_superset(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "isSuperset")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let a_set: std::collections::HashSet<String> = a.iter()
            .map(|v| format!("{:?}", v))
            .collect();

        let is_superset = b.iter().all(|item| a_set.contains(&format!("{:?}", item)));

        Ok(JsonnetValue::boolean(is_superset))
    }

    fn is_disjoint(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "isDisjoint")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let a_set: std::collections::HashSet<String> = a.iter()
            .map(|v| format!("{:?}", v))
            .collect();
        let b_set: std::collections::HashSet<String> = b.iter()
            .map(|v| format!("{:?}", v))
            .collect();

        let is_disjoint = a_set.intersection(&b_set).count() == 0;

        Ok(JsonnetValue::boolean(is_disjoint))
    }

    fn cartesian(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "cartesian")?;
        let arrays = args[0].as_array()?;

        if arrays.is_empty() {
            return Ok(JsonnetValue::array(vec![JsonnetValue::array(vec![])]));
        }

        // Convert to vectors
        let mut vec_arrays = Vec::new();
        for arr in arrays {
            vec_arrays.push(arr.as_array()?.clone());
        }

        fn cartesian_product(arrays: &[Vec<JsonnetValue>]) -> Vec<Vec<JsonnetValue>> {
            if arrays.is_empty() {
                return vec![vec![]];
            }

            let mut result = Vec::new();
            let first = &arrays[0];
            let rest = &arrays[1..];

            for item in first {
                for mut combo in cartesian_product(rest) {
                    combo.insert(0, item.clone());
                    result.push(combo);
                }
            }

            result
        }

        let products = cartesian_product(&vec_arrays);
        let result: Vec<JsonnetValue> = products.into_iter()
            .map(|combo| JsonnetValue::array(combo))
            .collect();

        Ok(JsonnetValue::array(result))
    }

    fn cross(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "cross")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let mut result = Vec::new();
        for item_a in a {
            for item_b in b {
                result.push(JsonnetValue::array(vec![item_a.clone(), item_b.clone()]));
            }
        }

        Ok(JsonnetValue::array(result))
    }

    fn dot(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "dot")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        if a.len() != b.len() {
            return Err(JsonnetError::invalid_function_call("dot() arrays must have same length".to_string()));
        }

        let mut sum = 0.0;
        for (x, y) in a.iter().zip(b.iter()) {
            sum += x.as_number()? * y.as_number()?;
        }

        Ok(JsonnetValue::number(sum))
    }

    fn norm(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "norm")?;
        let arr = args[0].as_array()?;

        let mut sum_squares = 0.0;
        for item in arr {
            let val = item.as_number()?;
            sum_squares += val * val;
        }

        Ok(JsonnetValue::number(sum_squares.sqrt()))
    }

    fn normalize(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "normalize")?;
        let arr = args[0].as_array()?;

        // Calculate norm directly to avoid recursion
        let mut sum_squares = 0.0;
        for item in arr {
            let val = item.as_number()?;
            sum_squares += val * val;
        }
        let norm_val = sum_squares.sqrt();

        if norm_val == 0.0 {
            return Ok(args[0].clone());
        }

        let mut result = Vec::new();
        for item in arr {
            let val = item.as_number()?;
            result.push(JsonnetValue::number(val / norm_val));
        }

        Ok(JsonnetValue::array(result))
    }

    fn distance(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "distance")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        if a.len() != b.len() {
            return Err(JsonnetError::invalid_function_call("distance() arrays must have same length".to_string()));
        }

        let mut sum_squares = 0.0;
        for (x, y) in a.iter().zip(b.iter()) {
            let diff = x.as_number()? - y.as_number()?;
            sum_squares += diff * diff;
        }

        Ok(JsonnetValue::number(sum_squares.sqrt()))
    }

    fn angle(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "angle")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        if a.len() != b.len() {
            return Err(JsonnetError::invalid_function_call("angle() arrays must have same length".to_string()));
        }

        // Calculate dot product directly
        let mut dot_product = 0.0;
        for (x, y) in a.iter().zip(b.iter()) {
            dot_product += x.as_number()? * y.as_number()?;
        }

        // Calculate norms directly
        let mut norm_a_sq = 0.0;
        for item in a {
            let val = item.as_number()?;
            norm_a_sq += val * val;
        }
        let norm_a = norm_a_sq.sqrt();

        let mut norm_b_sq = 0.0;
        for item in b {
            let val = item.as_number()?;
            norm_b_sq += val * val;
        }
        let norm_b = norm_b_sq.sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return Ok(JsonnetValue::number(0.0));
        }

        let cos_theta = dot_product / (norm_a * norm_b);
        let cos_theta = cos_theta.max(-1.0).min(1.0); // Clamp to avoid floating point errors

        Ok(JsonnetValue::number(cos_theta.acos()))
    }

    fn rotate(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "rotate")?;
        let point = args[0].as_array()?;
        let angle = args[1].as_number()?;

        if point.len() != 2 {
            return Err(JsonnetError::invalid_function_call("rotate() point must be 2D".to_string()));
        }

        let center = if args.len() > 2 {
            args[2].as_array()?.to_vec()
        } else {
            vec![JsonnetValue::number(0.0), JsonnetValue::number(0.0)]
        };

        if center.len() != 2 {
            return Err(JsonnetError::invalid_function_call("rotate() center must be 2D".to_string()));
        }

        let x = point[0].as_number()? - center[0].as_number()?;
        let y = point[1].as_number()? - center[1].as_number()?;

        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let new_x = x * cos_a - y * sin_a + center[0].as_number()?;
        let new_y = x * sin_a + y * cos_a + center[1].as_number()?;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    fn scale(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "scale")?;
        let point = args[0].as_array()?;
        let factor = args[1].as_number()?;

        if point.len() != 2 {
            return Err(JsonnetError::invalid_function_call("scale() point must be 2D".to_string()));
        }

        let center = if args.len() > 2 {
            args[2].as_array()?.to_vec()
        } else {
            vec![JsonnetValue::number(0.0), JsonnetValue::number(0.0)]
        };

        if center.len() != 2 {
            return Err(JsonnetError::invalid_function_call("scale() center must be 2D".to_string()));
        }

        let x = point[0].as_number()? - center[0].as_number()?;
        let y = point[1].as_number()? - center[1].as_number()?;

        let new_x = x * factor + center[0].as_number()?;
        let new_y = y * factor + center[1].as_number()?;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    fn translate(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "translate")?;
        let point = args[0].as_array()?;
        let offset = args[1].as_array()?;

        if point.len() != 2 || offset.len() != 2 {
            return Err(JsonnetError::invalid_function_call("translate() requires 2D point and offset".to_string()));
        }

        let new_x = point[0].as_number()? + offset[0].as_number()?;
        let new_y = point[1].as_number()? + offset[1].as_number()?;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    fn reflect(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "reflect")?;
        let point = args[0].as_array()?;
        let axis = args[1].as_number()?; // angle of reflection axis in radians

        if point.len() != 2 {
            return Err(JsonnetError::invalid_function_call("reflect() point must be 2D".to_string()));
        }

        let x = point[0].as_number()?;
        let y = point[1].as_number()?;

        let cos_2a = (2.0 * axis).cos();
        let sin_2a = (2.0 * axis).sin();

        let new_x = x * cos_2a + y * sin_2a;
        let new_y = x * sin_2a - y * cos_2a;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    fn affine(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "affine")?;
        let point = args[0].as_array()?;
        let matrix = args[1].as_array()?;

        if point.len() != 2 {
            return Err(JsonnetError::invalid_function_call("affine() point must be 2D".to_string()));
        }

        if matrix.len() != 6 {
            return Err(JsonnetError::invalid_function_call("affine() matrix must be 6 elements [a,b,c,d,e,f]".to_string()));
        }

        let x = point[0].as_number()?;
        let y = point[1].as_number()?;

        let a = matrix[0].as_number()?;
        let b = matrix[1].as_number()?;
        let c = matrix[2].as_number()?;
        let d = matrix[3].as_number()?;
        let e = matrix[4].as_number()?;
        let f = matrix[5].as_number()?;

        let new_x = a * x + b * y + e;
        let new_y = c * x + d * y + f;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    fn split_limit(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 3, "splitLimit")?;
        let s = args[0].as_string()?;
        let sep = args[1].as_string()?;
        let limit = args[2].as_number()? as usize;

        if sep.is_empty() {
            // Split into characters
            let chars: Vec<String> = s.chars().take(limit).map(|c| c.to_string()).collect();
            let result: Vec<JsonnetValue> = chars.into_iter().map(JsonnetValue::string).collect();
            return Ok(JsonnetValue::array(result));
        }

        let mut parts: Vec<&str> = s.splitn(limit + 1, &sep).collect();
        if parts.len() > limit {
            // Join the remaining parts
            let remaining = parts.split_off(limit);
            parts.push(&s[(s.len() - remaining.join(&sep).len())..]);
        }

        let result: Vec<JsonnetValue> = parts.into_iter().map(|s| JsonnetValue::string(s.to_string())).collect();
        Ok(JsonnetValue::array(result))
    }

    fn join_variadic(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        if args.is_empty() {
            return Err(JsonnetError::invalid_function_call("join() expects at least one argument".to_string()));
        }

        let sep = args[0].as_string()?;
        let arrays: Result<Vec<Vec<JsonnetValue>>> = args[1..].iter()
            .map(|arg| arg.as_array().cloned())
            .collect();

        let arrays = arrays?;
        let mut result = Vec::new();

        for (i, arr) in arrays.iter().enumerate() {
            if i > 0 && !sep.is_empty() {
                result.push(JsonnetValue::string(sep.clone()));
            }
            result.extend(arr.iter().cloned());
        }

        Ok(JsonnetValue::array(result))
    }

    fn replace(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 3, "replace")?;
        let s = args[0].as_string()?;
        let old = args[1].as_string()?;
        let new = args[2].as_string()?;

        let result = s.replace(&old, &new);
        Ok(JsonnetValue::string(result))
    }

    fn contains_variadic(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "contains")?;

        match &args[0] {
            JsonnetValue::Array(arr) => {
                // Simple linear search with string comparison
                let target = format!("{:?}", &args[1]);
                for item in arr {
                    if format!("{:?}", item) == target {
                        return Ok(JsonnetValue::boolean(true));
                    }
                }
                Ok(JsonnetValue::boolean(false))
            }
            JsonnetValue::String(s) => {
                let substr = args[1].as_string()?;
                Ok(JsonnetValue::boolean(s.contains(&substr)))
            }
            JsonnetValue::Object(obj) => {
                let key = args[1].as_string()?;
                Ok(JsonnetValue::boolean(obj.contains_key(&*key)))
            }
            _ => Err(JsonnetError::invalid_function_call("contains() expects array, string, or object".to_string())),
        }
    }

    // Placeholder implementations for functions requiring function callbacks
    fn sort_by(&mut self, _args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Err(JsonnetError::runtime_error("sortBy() requires function calling mechanism - placeholder implementation".to_string()))
    }

    fn group_by(&mut self, _args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Err(JsonnetError::runtime_error("groupBy() requires function calling mechanism - placeholder implementation".to_string()))
    }

    fn partition(&mut self, _args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Err(JsonnetError::runtime_error("partition() requires function calling mechanism - placeholder implementation".to_string()))
    }
}

impl StdLib {
    /// Dispatches a standard library function call.
    pub fn dispatch(
        &self,
        func_name: &str,
        args: &[JsonnetValue],
    ) -> Result<JsonnetValue> {
        match func_name {
            // AI Functions (Stubbed)
            "ai.httpGet" => Ok(JsonnetValue::string("ai.httpGet stub")),
            "ai.httpPost" => Ok(JsonnetValue::string("ai.httpPost stub")),
            "ai.callModel" => Ok(JsonnetValue::string("ai.callModel stub")),

            // Tool Functions (Stubbed)
            "tool.execute" => Ok(JsonnetValue::string("tool.execute stub")),

            // Memory Functions (Stubbed)
            "memory.get" => Ok(JsonnetValue::string("memory.get stub")),
            "memory.set" => Ok(JsonnetValue::string("memory.set stub")),

            // Agent Functions (Stubbed)
            "agent.create" => Ok(JsonnetValue::string("agent.create stub")),
            "agent.execute" => Ok(JsonnetValue::string("agent.execute stub")),

            // Chain Functions (Stubbed)
            "chain.create" => Ok(JsonnetValue::string("chain.create stub")),
            "chain.execute" => Ok(JsonnetValue::string("chain.execute stub")),

            // Existing functions...
            "std.extVar" => self.std_ext_var(args.to_vec()),
            "std.manifestJson" => self.std_manifest_json(args.to_vec()),
            // ... existing code ...

            _ => Err(JsonnetError::runtime_error(format!("Unknown std function: {}", func_name))),
        }
    }

    // ==========================================
    // Missing AI Agent Functions Implementation
    // ==========================================

    /// ai.httpGet(url, headers?) - HTTP GET request
    pub fn ai_http_get(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "ai.httpGet")?;
        let url = args[0].as_string()?;
        let headers = if args.len() > 1 {
            args[1].as_object()?
        } else {
            &std::collections::HashMap::new()
        };

        // Stub implementation - return mock response
        let result = json!({
            "url": url,
            "method": "GET",
            "headers": headers,
            "status": "pending",
            "response": "HTTP GET will be handled by AI runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// ai.httpPost(url, body, headers?) - HTTP POST request
    pub fn ai_http_post(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "ai.httpPost")?;
        let url = args[0].as_string()?;
        let body = &args[1];
        let headers = if args.len() > 2 {
            args[2].as_object()?
        } else {
            &std::collections::HashMap::new()
        };

        // Stub implementation - return mock response
        let result = json!({
            "url": url,
            "method": "POST",
            "body": body,
            "headers": headers,
            "status": "pending",
            "response": "HTTP POST will be handled by AI runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// ai.callModel(model, prompt, options?) - Call AI model
    pub fn ai_call_model(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "ai.callModel")?;
        let model = args[0].as_string()?;
        let prompt = args[1].as_string()?;
        let options = if args.len() > 2 {
            args[2].as_object()?
        } else {
            &std::collections::HashMap::new()
        };

        // Stub implementation - return mock response
        let result = json!({
            "model": model,
            "prompt": prompt,
            "options": options,
            "status": "pending",
            "response": "AI model call will be handled by AI runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// tool.execute(name, args) - Execute external tool
    pub fn tool_execute(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "tool.execute")?;
        let name = args[0].as_string()?;
        let tool_args = &args[1];

        // Stub implementation - return mock response
        let result = json!({
            "tool": name,
            "args": tool_args,
            "status": "pending",
            "output": "Tool execution will be handled by runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// memory.get(key) - Get value from memory
    pub fn memory_get(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "memory.get")?;
        let key = args[0].as_string()?;

        // Stub implementation - return mock response
        let result = json!({
            "key": key,
            "operation": "get",
            "status": "pending",
            "value": "Memory access will be handled by runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// memory.set(key, value) - Set value in memory
    pub fn memory_set(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "memory.set")?;
        let key = args[0].as_string()?;
        let value = &args[1];

        // Stub implementation - return mock response
        let result = json!({
            "key": key,
            "value": value,
            "operation": "set",
            "status": "success"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// agent.create(config) - Create AI agent
    pub fn agent_create(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "agent.create")?;
        let config = args[0].as_object()?;

        // Stub implementation - return mock response
        let result = json!({
            "config": config,
            "operation": "create",
            "status": "pending",
            "agent_id": "mock-agent-id"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// std.extVar(name) - Get external variable
    pub fn std_ext_var(&self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "std.extVar")?;
        let name = args[0].as_string()?;

        // Stub implementation - return mock external variable
        let result = json!({
            "variable": name,
            "value": "External variable will be resolved by runtime",
            "status": "pending"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// std.manifestJson(value, indent?) - JSON manifest with optional indentation
    pub fn std_manifest_json(&self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "std.manifestJson")?;
        let value = &args[0];
        let indent = if args.len() > 1 {
            args[1].as_number()? as usize
        } else {
            0
        };

        // Convert JsonnetValue to JSON string
        match serde_json::to_string_pretty(&value) {
            Ok(json_str) => {
                if indent > 0 {
                    Ok(JsonnetValue::string(json_str))
                } else {
                    Ok(JsonnetValue::string(serde_json::to_string(&value).unwrap_or_default()))
                }
            }
            Err(_) => Err(JsonnetError::runtime_error("Failed to serialize to JSON")),
        }
    }

    /// std.length(x) - returns length of array, string, or object
    pub fn length(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "length")?;
        match &args[0] {
            JsonnetValue::Array(arr) => Ok(JsonnetValue::number(arr.len() as f64)),
            JsonnetValue::String(s) => Ok(JsonnetValue::number(s.len() as f64)),
            JsonnetValue::Object(obj) => Ok(JsonnetValue::number(obj.len() as f64)),
            _ => Err(JsonnetError::type_error("length() requires array, string, or object")),
        }
    }

    /// std.type(x) - returns type of value as string
    fn type_of(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "type")?;
        let type_str = args[0].type_name();
        Ok(JsonnetValue::string(type_str))
    }

    /// std.makeArray(n, func) - creates array by calling func n times
    fn make_array(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "makeArray")?;
        let n = args[0].as_number()? as usize;
        let _func = &args[1];

        // For now, create a simple array [0, 1, 2, ..., n-1]
        // TODO: Implement proper function calling
        let mut result = Vec::new();
        for i in 0..n {
            // Since we can't call functions yet, just create an array of indices
            result.push(JsonnetValue::number(i as f64));
        }

        Ok(JsonnetValue::array(result))
    }

    /// std.filter(func, arr) - filters array using predicate function
    fn filter(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "filter")?;
        let _func = &args[0];
        let _arr = args[1].as_array()?;
        // TODO: Implement function calling for higher-order functions
        // For now, return original array
        Ok(args[1].clone())
    }

    /// std.map(func, arr) - maps function over array
    fn map(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "map")?;
        let _func = &args[0];
        let _arr = args[1].as_array()?;
        // TODO: Implement function calling for higher-order functions
        // For now, return original array
        Ok(args[1].clone())
    }

    /// std.foldl(func, arr, init) - left fold
    fn foldl(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 3, "foldl")?;
        let _func = &args[0];
        let _arr = args[1].as_array()?;
        // TODO: Implement function calling for higher-order functions
        // For now, return initial value
        Ok(args[2].clone())
    }

    /// std.foldr(func, arr, init) - right fold
    fn foldr(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 3, "foldr")?;
        let _func = &args[0];
        let _arr = args[1].as_array()?;
        // TODO: Implement function calling for higher-order functions
        // For now, return initial value
        Ok(args[2].clone())
    }

    /// std.range(n) - creates array [0, 1, ..., n-1]
    fn range(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "range")?;
        let n = args[0].as_number()? as usize;
        let arr: Vec<JsonnetValue> = (0..n).map(|i| JsonnetValue::number(i as f64)).collect();
        Ok(JsonnetValue::array(arr))
    }

    /// std.join(sep, arr) - joins array elements with separator
    fn join(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "join")?;
        let sep = args[0].as_string()?;
        let arr = args[1].as_array()?;

        let mut result = String::new();
        for (i, item) in arr.iter().enumerate() {
            if i > 0 {
                result.push_str(sep);
            }
            result.push_str(&item.to_string());
        }

        Ok(JsonnetValue::string(result))
    }

    /// std.split(str, sep) - splits string by separator
    fn split(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "split")?;
        let s = args[0].as_string()?;
        let sep = args[1].as_string()?;

        let parts: Vec<JsonnetValue> = s.split(sep).map(JsonnetValue::string).collect();
        Ok(JsonnetValue::array(parts))
    }

    /// std.contains(arr, elem) - checks if array contains element
    fn contains(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "contains")?;
        let arr = args[0].as_array()?;
        let contains = arr.iter().any(|item| item.equals(&args[1]));
        Ok(JsonnetValue::boolean(contains))
    }

    /// std.startsWith(str, prefix) - checks if string starts with prefix
    fn starts_with(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "startsWith")?;
        let s = args[0].as_string()?;
        let prefix = args[1].as_string()?;
        Ok(JsonnetValue::boolean(s.starts_with(prefix)))
    }

    /// std.endsWith(str, suffix) - checks if string ends with suffix
    fn ends_with(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "endsWith")?;
        let s = args[0].as_string()?;
        let suffix = args[1].as_string()?;
        Ok(JsonnetValue::boolean(s.ends_with(suffix)))
    }

    /// std.substr(str, from, len) - extracts substring
    fn substr(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 3, "substr")?;
        let s = args[0].as_string()?;
        let from = args[1].as_number()? as usize;
        let len = args[2].as_number()? as usize;

        let substr = if from >= s.len() {
            ""
        } else {
            let end = (from + len).min(s.len());
            &s[from..end]
        };

        Ok(JsonnetValue::string(substr))
    }

    /// std.char(n) - returns character for codepoint
    fn char_fn(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "char")?;
        let n = args[0].as_number()? as u32;
        match char::from_u32(n) {
            Some(c) => Ok(JsonnetValue::string(c.to_string())),
            None => Err(JsonnetError::runtime_error("Invalid codepoint")),
        }
    }

    /// std.codepoint(str) - returns codepoint of first character
    fn codepoint(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "codepoint")?;
        let s = args[0].as_string()?;
        match s.chars().next() {
            Some(c) => Ok(JsonnetValue::number(c as u32 as f64)),
            None => Err(JsonnetError::runtime_error("Empty string")),
        }
    }

    /// std.toString(x) - converts value to string
    fn to_string(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "toString")?;
        Ok(JsonnetValue::string(args[0].to_string()))
    }

    /// std.parseInt(str) - parses string as integer
    fn parse_int(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "parseInt")?;
        let s = args[0].as_string()?;
        match s.parse::<f64>() {
            Ok(n) => Ok(JsonnetValue::number(n)),
            Err(_) => Err(JsonnetError::runtime_error("Invalid number format")),
        }
    }

    /// std.parseJson(str) - parses JSON string
    fn parse_json(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "parseJson")?;
        let s = args[0].as_string()?;
        match serde_json::from_str::<serde_json::Value>(s) {
            Ok(value) => Ok(JsonnetValue::from_json_value(value)),
            Err(_) => Err(JsonnetError::runtime_error("Invalid JSON")),
        }
    }

    /// std.encodeUTF8(str) - encodes string as UTF-8 bytes
    fn encode_utf8(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "encodeUTF8")?;
        let s = args[0].as_string()?;
        let bytes: Vec<JsonnetValue> = s.as_bytes().iter().map(|&b| JsonnetValue::number(b as f64)).collect();
        Ok(JsonnetValue::array(bytes))
    }

    /// std.decodeUTF8(arr) - decodes UTF-8 bytes to string
    fn decode_utf8(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "decodeUTF8")?;
        let arr = args[0].as_array()?;
        let mut bytes = Vec::new();
        for item in arr {
            let b = item.as_number()? as u8;
            bytes.push(b);
        }
        match String::from_utf8(bytes) {
            Ok(s) => Ok(JsonnetValue::string(s)),
            Err(_) => Err(JsonnetError::runtime_error("Invalid UTF-8 sequence")),
        }
    }

    /// std.md5(str) - computes MD5 hash
    fn md5(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "md5")?;
        let s = args[0].as_string()?;
        use md5::{Md5, Digest};
        let mut hasher = Md5::new();
        hasher.update(s.as_bytes());
        let result = hasher.finalize();
        Ok(JsonnetValue::string(format!("{:x}", result)))
    }

    /// std.base64(str) - base64 encodes string
    fn base64(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "base64")?;
        let s = args[0].as_string()?;
        use base64::{Engine as _, engine::general_purpose};
        let encoded = general_purpose::STANDARD.encode(s.as_bytes());
        Ok(JsonnetValue::string(encoded))
    }

    /// std.base64Decode(str) - base64 decodes string
    fn base64_decode(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "base64Decode")?;
        let s = args[0].as_string()?;
        use base64::{Engine as _, engine::general_purpose};
        match general_purpose::STANDARD.decode(s.as_bytes()) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(decoded) => Ok(JsonnetValue::string(decoded)),
                Err(_) => Err(JsonnetError::runtime_error("Invalid UTF-8 in decoded data")),
            },
            Err(_) => Err(JsonnetError::runtime_error("Invalid base64")),
        }
    }

    /// std.manifestJson(x) - pretty prints value as JSON
    fn manifest_json(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "manifestJson")?;
        let json = serde_json::to_string_pretty(&args[0].to_json_value())?;
        Ok(JsonnetValue::string(json))
    }

    /// std.manifestJsonEx(x, indent) - pretty prints value as JSON with custom indent
    fn manifest_json_ex(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "manifestJsonEx")?;
        let value = &args[0];
        let indent = args[1].as_string()?;

        // Simple implementation with custom indentation
        // For now, just use serde_json with the indent string
        match serde_json::to_string_pretty(&value.to_json_value()) {
            Ok(json) => {
                if indent.is_empty() {
                    Ok(JsonnetValue::string(json))
                } else {
                    // Replace default 2-space indentation with custom indent
                    let indented = json.lines()
                        .map(|line| {
                            let leading_spaces = line.chars().take_while(|c| *c == ' ').count();
                            if leading_spaces > 0 {
                                let indent_level = leading_spaces / 2;
                                format!("{}{}", indent.repeat(indent_level), &line[leading_spaces..])
                            } else {
                                line.to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    Ok(JsonnetValue::string(indented))
                }
            }
            Err(_) => Err(JsonnetError::runtime_error("Failed to serialize to JSON")),
        }
    }

    /// std.manifestYaml(x) - pretty prints value as YAML
    #[cfg(feature = "yaml")]
    fn manifest_yaml(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "manifestYaml")?;
        let yaml = serde_yaml::to_string(&args[0].to_json_value())?;
        Ok(JsonnetValue::string(yaml))
    }

    /// std.manifestYaml(x) - pretty prints value as YAML (fallback when yaml feature disabled)
    #[cfg(not(feature = "yaml"))]
    fn manifest_yaml(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "manifestYaml")?;
        // Fallback to JSON when YAML feature is disabled
        Self::manifest_json(args)
    }

    // String escaping functions
    fn escape_string_json(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "escapeStringJson")?;
        let s = args[0].as_string()?;
        let escaped = serde_json::to_string(s)?;
        Ok(JsonnetValue::string(escaped))
    }

    fn escape_string_yaml(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "escapeStringYaml")?;
        let s = args[0].as_string()?;

        // Basic YAML string escaping
        // YAML requires escaping certain characters in strings
        let mut escaped = String::new();
        for ch in s.chars() {
            match ch {
                '"' => escaped.push_str("\\\""),
                '\\' => escaped.push_str("\\\\"),
                '\n' => escaped.push_str("\\n"),
                '\r' => escaped.push_str("\\r"),
                '\t' => escaped.push_str("\\t"),
                '\0' => escaped.push_str("\\0"),
                _ => escaped.push(ch),
            }
        }

        // Wrap in quotes if the string contains special characters
        let needs_quotes = s.contains(' ') || s.contains('\t') || s.contains('\n') ||
                          s.contains(':') || s.contains('#') || s.contains('-') ||
                          s.starts_with('[') || s.starts_with('{') ||
                          s.starts_with('"') || s.starts_with('\'');

        if needs_quotes {
            Ok(JsonnetValue::string(format!("\"{}\"", escaped)))
        } else {
            Ok(JsonnetValue::string(escaped))
        }
    }

    fn escape_string_python(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "escapeStringPython")?;
        let s = args[0].as_string()?;
        let escaped = s.escape_default().to_string();
        Ok(JsonnetValue::string(format!("'{}'", escaped)))
    }

    fn escape_string_bash(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "escapeStringBash")?;
        let s = args[0].as_string()?;
        let escaped = s.replace("'", "'\"'\"'").replace("\\", "\\\\");
        Ok(JsonnetValue::string(format!("'{}'", escaped)))
    }

    fn escape_string_dollars(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "escapeStringDollars")?;
        let s = args[0].as_string()?;
        let escaped = s.replace("$$", "$").replace("$", "$$");
        Ok(JsonnetValue::string(escaped))
    }

    // Additional string functions
    fn string_chars(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "stringChars")?;
        let s = args[0].as_string()?;
        let chars: Vec<JsonnetValue> = s.chars().map(|c| JsonnetValue::string(c.to_string())).collect();
        Ok(JsonnetValue::array(chars))
    }

    fn string_bytes(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "stringBytes")?;
        let s = args[0].as_string()?;
        let bytes: Vec<JsonnetValue> = s.as_bytes().iter().map(|&b| JsonnetValue::number(b as f64)).collect();
        Ok(JsonnetValue::array(bytes))
    }

    fn format(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "format")?;
        let format_str = args[0].as_string()?;
        let values = args[1].as_array()?;

        // Simple format implementation - replace %1, %2, etc. with values
        let mut result = format_str.to_string();
        for (i, value) in values.iter().enumerate() {
            let placeholder = format!("%{}", i + 1);
            let value_str = match value {
                JsonnetValue::String(s) => s.clone(),
                JsonnetValue::Number(n) => n.to_string(),
                JsonnetValue::Boolean(b) => b.to_string(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &value_str);
        }

        Ok(JsonnetValue::string(result))
    }

    // Type checking functions
    fn is_array(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isArray")?;
        Ok(JsonnetValue::boolean(matches!(args[0], JsonnetValue::Array(_))))
    }

    fn is_boolean(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isBoolean")?;
        Ok(JsonnetValue::boolean(matches!(args[0], JsonnetValue::Boolean(_))))
    }

    fn is_function(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isFunction")?;
        Ok(JsonnetValue::boolean(matches!(args[0], JsonnetValue::Function(_))))
    }

    fn is_number(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isNumber")?;
        Ok(JsonnetValue::boolean(matches!(args[0], JsonnetValue::Number(_))))
    }

    fn is_object(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isObject")?;
        Ok(JsonnetValue::boolean(matches!(args[0], JsonnetValue::Object(_))))
    }

    fn is_string(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isString")?;
        Ok(JsonnetValue::boolean(matches!(args[0], JsonnetValue::String(_))))
    }

    // Array functions
    fn count(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "count")?;
        let arr = args[0].as_array()?;
        let elem = &args[1];
        let count = arr.iter().filter(|item| item.equals(elem)).count() as f64;
        Ok(JsonnetValue::number(count))
    }

    fn find(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "find")?;
        match (&args[0], &args[1]) {
            (JsonnetValue::Array(arr), value) => {
                let mut indices = Vec::new();
                for (i, item) in arr.iter().enumerate() {
                    if item == value {
                        indices.push(JsonnetValue::Number(i as f64));
                    }
                }
                Ok(JsonnetValue::array(indices))
            }
            _ => Err(JsonnetError::runtime_error("find expects array and search value")),
        }
    }

    fn member(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::contains(args)
    }

    // Math functions
    fn modulo(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "modulo")?;
        let a = args[0].as_number()?;
        let b = args[1].as_number()?;
        if b == 0.0 {
            return Err(JsonnetError::DivisionByZero);
        }
        Ok(JsonnetValue::number(a % b))
    }

    fn pow(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "pow")?;
        let a = args[0].as_number()?;
        let b = args[1].as_number()?;
        Ok(JsonnetValue::number(a.powf(b)))
    }

    fn exp(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "exp")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.exp()))
    }

    fn log(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "log")?;
        let x = args[0].as_number()?;
        if x <= 0.0 {
            return Err(JsonnetError::runtime_error("log of non-positive number"));
        }
        Ok(JsonnetValue::number(x.ln()))
    }

    fn sqrt(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "sqrt")?;
        let x = args[0].as_number()?;
        if x < 0.0 {
            return Err(JsonnetError::runtime_error("sqrt of negative number"));
        }
        Ok(JsonnetValue::number(x.sqrt()))
    }

    fn sin(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "sin")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.sin()))
    }

    fn cos(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "cos")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.cos()))
    }

    fn tan(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "tan")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.tan()))
    }

    fn asin(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "asin")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.asin()))
    }

    fn acos(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "acos")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.acos()))
    }

    fn atan(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "atan")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.atan()))
    }

    fn floor(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "floor")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.floor()))
    }

    fn ceil(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "ceil")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.ceil()))
    }

    fn round(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "round")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.round()))
    }

    fn abs(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "abs")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.abs()))
    }

    fn max(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "max")?;
        let arr = args[0].as_array()?;
        if arr.is_empty() {
            return Err(JsonnetError::runtime_error("max() called on empty array"));
        }
        let mut max_val = f64::NEG_INFINITY;
        for item in arr {
            let val = item.as_number()?;
            if val > max_val {
                max_val = val;
            }
        }
        Ok(JsonnetValue::number(max_val))
    }

    fn min(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "min")?;
        let arr = args[0].as_array()?;
        if arr.is_empty() {
            return Err(JsonnetError::runtime_error("min() called on empty array"));
        }
        let mut min_val = f64::INFINITY;
        for item in arr {
            let val = item.as_number()?;
            if val < min_val {
                min_val = val;
            }
        }
        Ok(JsonnetValue::number(min_val))
    }

    fn clamp(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 3, "clamp")?;
        let x = args[0].as_number()?;
        let min = args[1].as_number()?;
        let max = args[2].as_number()?;
        let clamped = x.max(min).min(max);
        Ok(JsonnetValue::number(clamped))
    }

    fn assert_equal(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "assertEqual")?;
        if !args[0].equals(&args[1]) {
            return Err(JsonnetError::assertion_failed(format!(
                "Assertion failed: {} != {}\n  Left: {:?}\n  Right: {:?}",
                args[0], args[1], args[0], args[1]
            )));
        }
        Ok(JsonnetValue::boolean(true))
    }

    fn trace(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "trace")?;
        // Print the second argument to stderr for tracing
        eprintln!("TRACE: {:?}", args[1]);
        // Return the first argument
        Ok(args[0].clone())
    }

    // Array manipulation functions
    fn sort(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "sort")?;
        let arr = args[0].as_array()?;

        if arr.is_empty() {
            return Ok(JsonnetValue::array(vec![]));
        }

        // Implement proper Jsonnet sorting
        // Jsonnet sorts by comparing values directly, not by string representation
        let mut sorted = arr.clone();
        sorted.sort_by(|a, b| Self::compare_values(a, b));

        Ok(JsonnetValue::array(sorted))
    }

    fn compare_values(a: &JsonnetValue, b: &JsonnetValue) -> std::cmp::Ordering {
        match (a, b) {
            (JsonnetValue::Null, JsonnetValue::Null) => std::cmp::Ordering::Equal,
            (JsonnetValue::Null, _) => std::cmp::Ordering::Less,
            (_, JsonnetValue::Null) => std::cmp::Ordering::Greater,
            (JsonnetValue::Boolean(x), JsonnetValue::Boolean(y)) => x.cmp(y),
            (JsonnetValue::Boolean(_), _) => std::cmp::Ordering::Less,
            (_, JsonnetValue::Boolean(_)) => std::cmp::Ordering::Greater,
            (JsonnetValue::Number(x), JsonnetValue::Number(y)) => {
                x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
            }
            (JsonnetValue::Number(_), _) => std::cmp::Ordering::Less,
            (_, JsonnetValue::Number(_)) => std::cmp::Ordering::Greater,
            (JsonnetValue::String(x), JsonnetValue::String(y)) => x.cmp(y),
            (JsonnetValue::String(_), _) => std::cmp::Ordering::Less,
            (_, JsonnetValue::String(_)) => std::cmp::Ordering::Greater,
            (JsonnetValue::Array(x), JsonnetValue::Array(y)) => {
                for (_i, (a_item, b_item)) in x.iter().zip(y.iter()).enumerate() {
                    let cmp = Self::compare_values(a_item, b_item);
                    if cmp != std::cmp::Ordering::Equal {
                        return cmp;
                    }
                }
                x.len().cmp(&y.len())
            }
            (JsonnetValue::Array(_), _) => std::cmp::Ordering::Less,
            (_, JsonnetValue::Array(_)) => std::cmp::Ordering::Greater,
            (JsonnetValue::Object(x), JsonnetValue::Object(y)) => {
                // Compare by sorted keys and values
                let mut x_keys: Vec<_> = x.keys().collect();
                let mut y_keys: Vec<_> = y.keys().collect();
                x_keys.sort();
                y_keys.sort();

                for (x_key, y_key) in x_keys.iter().zip(y_keys.iter()) {
                    let key_cmp = x_key.cmp(y_key);
                    if key_cmp != std::cmp::Ordering::Equal {
                        return key_cmp;
                    }
                    if let (Some(x_val), Some(y_val)) = (x.get(*x_key), y.get(*y_key)) {
                        let val_cmp = Self::compare_values(x_val, y_val);
                        if val_cmp != std::cmp::Ordering::Equal {
                            return val_cmp;
                        }
                    }
                }
                x.len().cmp(&y.len())
            }
            _ => std::cmp::Ordering::Equal, // Functions are considered equal for sorting
        }
    }

    fn uniq(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "uniq")?;
        let arr = args[0].as_array()?;

        let mut result: Vec<JsonnetValue> = Vec::new();
        for item in arr {
            // Check if item is already in result
            let mut found = false;
            for existing in &result {
                if existing.equals(item) {
                    found = true;
                    break;
                }
            }
            if !found {
                result.push(item.clone());
            }
        }

        Ok(JsonnetValue::array(result))
    }

    fn reverse(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "reverse")?;
        let arr = args[0].as_array()?;
        let reversed: Vec<JsonnetValue> = arr.iter().rev().cloned().collect();
        Ok(JsonnetValue::array(reversed))
    }

    // Object functions
    fn merge_patch(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "mergePatch")?;
        let target = args[0].as_object()?;
        let patch = args[1].as_object()?;

        let mut result = target.clone();

        for (key, patch_value) in patch {
            match patch_value {
                JsonnetValue::Null => {
                    // null values remove the key
                    result.remove(key);
                }
                JsonnetValue::Object(patch_obj) => {
                    // If both target and patch have objects, recursively merge
                    if let Some(JsonnetValue::Object(target_obj)) = result.get(key) {
                        let merged = Self::merge_patch(vec![
                            JsonnetValue::object(target_obj.clone()),
                            JsonnetValue::object(patch_obj.clone())
                        ])?;
                        if let JsonnetValue::Object(merged_obj) = merged {
                            result.insert(key.clone(), JsonnetValue::object(merged_obj));
                        }
                    } else {
                        // Target doesn't have an object, use patch object
                        result.insert(key.clone(), JsonnetValue::object(patch_obj.clone()));
                    }
                }
                _ => {
                    // For other values, just replace
                    result.insert(key.clone(), patch_value.clone());
                }
            }
        }

        Ok(JsonnetValue::object(result))
    }

    fn get(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 3, "get")?;
        let obj = args[0].as_object()?;
        let key = args[1].as_string()?;
        let default = &args[2];
        Ok(obj.get(key).unwrap_or(default).clone())
    }

    fn object_fields(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "objectFields")?;
        let obj = args[0].as_object()?;
        let fields: Vec<JsonnetValue> = obj.keys()
            .filter(|&k| !k.starts_with('_')) // Filter out hidden fields
            .map(|k| JsonnetValue::string(k.clone()))
            .collect();
        Ok(JsonnetValue::array(fields))
    }

    fn object_fields_all(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "objectFieldsAll")?;
        let obj = args[0].as_object()?;
        let fields: Vec<JsonnetValue> = obj.keys()
            .map(|k| JsonnetValue::string(k.clone()))
            .collect();
        Ok(JsonnetValue::array(fields))
    }

    fn object_has(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "objectHas")?;
        let obj = args[0].as_object()?;
        let key = args[1].as_string()?;
        Ok(JsonnetValue::boolean(obj.contains_key(key) && !key.starts_with('_')))
    }

    fn object_has_all(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "objectHasAll")?;
        let obj = args[0].as_object()?;
        let key = args[1].as_string()?;
        Ok(JsonnetValue::boolean(obj.contains_key(key)))
    }

    fn object_values(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "objectValues")?;
        let obj = args[0].as_object()?;
        let values: Vec<JsonnetValue> = obj.iter()
            .filter(|(k, _)| !k.starts_with('_'))
            .map(|(_, v)| v.clone())
            .collect();
        Ok(JsonnetValue::array(values))
    }

    fn object_values_all(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "objectValuesAll")?;
        let obj = args[0].as_object()?;
        let values: Vec<JsonnetValue> = obj.values().cloned().collect();
        Ok(JsonnetValue::array(values))
    }

    fn prune(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "prune")?;
        Self::prune_value(&args[0])
    }

    fn prune_value(value: &JsonnetValue) -> Result<JsonnetValue> {
        match value {
            JsonnetValue::Null => Ok(JsonnetValue::Null),
            JsonnetValue::Array(arr) => {
                let pruned: Vec<JsonnetValue> = arr.iter()
                    .map(|item| Self::prune_value(item))
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .filter(|item| !matches!(item, JsonnetValue::Null))
                    .collect();
                Ok(JsonnetValue::array(pruned))
            }
            JsonnetValue::Object(obj) => {
                let mut pruned_obj = HashMap::new();
                for (key, val) in obj {
                    let pruned_val = Self::prune_value(val)?;
                    if !matches!(pruned_val, JsonnetValue::Null) {
                        pruned_obj.insert(key.clone(), pruned_val);
                    }
                }
                Ok(JsonnetValue::object(pruned_obj))
            }
            _ => Ok(value.clone()),
        }
    }

    fn map_with_key(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "mapWithKey")?;
        let _func = &args[0];
        let obj = args[1].as_object()?;

        // For now, return a simple transformation
        // TODO: Implement proper function calling
        let mut result = HashMap::new();
        for (key, value) in obj {
            if !key.starts_with('_') {
                // Simple transformation: wrap key-value in array
                // In full implementation, this would call the function with (key, value)
                result.insert(key.clone(), JsonnetValue::array(vec![
                    JsonnetValue::string(key.clone()),
                    value.clone()
                ]));
            }
        }

        Ok(JsonnetValue::object(result))
    }

    fn object_fields_ex(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "objectFieldsEx")?;
        let obj = args[0].as_object()?;
        let include_hidden = args[1].as_boolean()?;

        let fields: Vec<JsonnetValue> = obj.keys()
            .filter(|&k| include_hidden || !k.starts_with('_'))
            .map(|k| JsonnetValue::string(k.clone()))
            .collect();

        Ok(JsonnetValue::array(fields))
    }

    fn object_values_ex(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "objectValuesEx")?;
        let obj = args[0].as_object()?;
        let include_hidden = args[1].as_boolean()?;

        let values: Vec<JsonnetValue> = obj.iter()
            .filter(|(k, _)| include_hidden || !k.starts_with('_'))
            .map(|(_, v)| v.clone())
            .collect();

        Ok(JsonnetValue::array(values))
    }

    fn to_lower(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "toLower")?;
        match &args[0] {
            JsonnetValue::String(s) => Ok(JsonnetValue::string(s.to_lowercase())),
            _ => Err(JsonnetError::runtime_error("toLower expects a string argument")),
        }
    }

    fn to_upper(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "toUpper")?;
        match &args[0] {
            JsonnetValue::String(s) => Ok(JsonnetValue::string(s.to_uppercase())),
            _ => Err(JsonnetError::runtime_error("toUpper expects a string argument")),
        }
    }

    fn trim(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "trim")?;
        match &args[0] {
            JsonnetValue::String(s) => Ok(JsonnetValue::string(s.trim().to_string())),
            _ => Err(JsonnetError::runtime_error("trim expects a string argument")),
        }
    }

    fn all(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "all")?;
        match &args[0] {
            JsonnetValue::Array(arr) => {
                let result = arr.iter().all(|item| item.is_truthy());
                Ok(JsonnetValue::boolean(result))
            }
            _ => Err(JsonnetError::runtime_error("all expects an array argument")),
        }
    }

    fn any(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "any")?;
        match &args[0] {
            JsonnetValue::Array(arr) => {
                let result = arr.iter().any(|item| item.is_truthy());
                Ok(JsonnetValue::boolean(result))
            }
            _ => Err(JsonnetError::runtime_error("any expects an array argument")),
        }
    }

    fn id(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "id")?;
        Ok(args[0].clone())
    }

    fn equals(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "equals")?;
        let a = &args[0];
        let b = &args[1];

        // First check primitive equality
        if a == b {
            return Ok(JsonnetValue::boolean(true));
        }

        // Check types
        let ta = a.type_name();
        let tb = b.type_name();
        if ta != tb {
            return Ok(JsonnetValue::boolean(false));
        }

        match (a, b) {
            (JsonnetValue::Array(arr_a), JsonnetValue::Array(arr_b)) => {
                if arr_a.len() != arr_b.len() {
                    return Ok(JsonnetValue::boolean(false));
                }
                for (i, item_a) in arr_a.iter().enumerate() {
                    let eq_args = vec![item_a.clone(), arr_b[i].clone()];
                    if let Ok(JsonnetValue::Boolean(false)) = Self::equals(eq_args) {
                        return Ok(JsonnetValue::boolean(false));
                    }
                }
                Ok(JsonnetValue::boolean(true))
            }
            (JsonnetValue::Object(obj_a), JsonnetValue::Object(obj_b)) => {
                // Get field names
                let fields_a: Vec<String> = obj_a.keys().cloned().collect();
                let fields_b: Vec<String> = obj_b.keys().cloned().collect();

                if fields_a.len() != fields_b.len() {
                    return Ok(JsonnetValue::boolean(false));
                }

                // Sort for comparison
                let mut sorted_a = fields_a.clone();
                sorted_a.sort();
                let mut sorted_b = fields_b.clone();
                sorted_b.sort();

                if sorted_a != sorted_b {
                    return Ok(JsonnetValue::boolean(false));
                }

                // Compare all field values
                for field in sorted_a {
                    let val_a = &obj_a[&field];
                    let val_b = &obj_b[&field];
                    let eq_args = vec![val_a.clone(), val_b.clone()];
                    if let Ok(JsonnetValue::Boolean(false)) = Self::equals(eq_args) {
                        return Ok(JsonnetValue::boolean(false));
                    }
                }
                Ok(JsonnetValue::boolean(true))
            }
            _ => Ok(JsonnetValue::boolean(false)),
        }
    }

    fn lines(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "lines")?;
        match &args[0] {
            JsonnetValue::Array(arr) => {
                let mut lines = Vec::new();
                for item in arr {
                    // Convert to string representation like Jsonnet does
                    match item {
                        JsonnetValue::String(s) => lines.push(s.clone()),
                        JsonnetValue::Number(n) => lines.push(n.to_string()),
                        JsonnetValue::Boolean(b) => lines.push(b.to_string()),
                        _ => lines.push(format!("{}", item)),
                    }
                }
                lines.push("".to_string()); // Add trailing newline
                Ok(JsonnetValue::string(lines.join("\n")))
            }
            _ => Err(JsonnetError::runtime_error("lines expects an array argument")),
        }
    }

    fn str_replace(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 3, "strReplace")?;

        let str_val = &args[0];
        let from_val = &args[1];
        let to_val = &args[2];

        let str = str_val.as_string()?.to_string();
        let from = from_val.as_string()?.to_string();
        let to = to_val.as_string()?.to_string();

        if from.is_empty() {
            return Err(JsonnetError::runtime_error("'from' string must not be zero length"));
        }

        // Simple implementation using Rust's string replace
        // For now, we'll use a simple approach. Full implementation would need
        // the complex recursive logic from Google Jsonnet
        let result = str.replace(&from, &to);
        Ok(JsonnetValue::string(result))
    }

    fn sha1(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "sha1")?;
        let input = args[0].as_string()?.as_bytes();
        let mut hasher = Sha1::new();
        hasher.update(input);
        let result = hasher.finalize();
        Ok(JsonnetValue::string(hex::encode(result)))
    }

    fn sha256(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "sha256")?;
        let input = args[0].as_string()?.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        Ok(JsonnetValue::string(hex::encode(result)))
    }

    fn sha3(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "sha3")?;
        let input = args[0].as_string()?.as_bytes();
        let mut hasher = Sha3_256::new();
        hasher.update(input);
        let result = hasher.finalize();
        Ok(JsonnetValue::string(hex::encode(result)))
    }

    fn sha512(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "sha512")?;
        let input = args[0].as_string()?.as_bytes();
        let mut hasher = Sha512::new();
        hasher.update(input);
        let result = hasher.finalize();
        Ok(JsonnetValue::string(hex::encode(result)))
    }

    fn ascii_lower(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "asciiLower")?;
        let input = args[0].as_string()?;
        Ok(JsonnetValue::string(input.to_ascii_lowercase()))
    }

    fn ascii_upper(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "asciiUpper")?;
        let input = args[0].as_string()?;
        Ok(JsonnetValue::string(input.to_ascii_uppercase()))
    }

    fn flat_map(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "flatMap")?;
        let _func = &args[0];
        let arr = &args[1];

        match arr {
            JsonnetValue::Array(array) => {
                let mut result = Vec::new();
                for item in array {
                    // Apply function to each item
                    // For now, we'll implement a simple version that expects the function to return an array
                    // Full implementation would need to evaluate the function
                    if let JsonnetValue::Array(sub_array) = item {
                        result.extend(sub_array.clone());
                    } else {
                        result.push(item.clone());
                    }
                }
                Ok(JsonnetValue::array(result))
            }
            JsonnetValue::String(s) => {
                // For strings, treat each character as an element
                let mut result = Vec::new();
                for ch in s.chars() {
                    // Apply function to each character - simplified implementation
                    result.push(JsonnetValue::string(ch.to_string()));
                }
                Ok(JsonnetValue::array(result))
            }
            _ => Err(JsonnetError::runtime_error("flatMap expects array or string as second argument")),
        }
    }

    fn map_with_index(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "mapWithIndex")?;
        let _func = &args[0];
        let arr = &args[1];

        match arr {
            JsonnetValue::Array(array) => {
                let mut result = Vec::new();
                for (i, item) in array.iter().enumerate() {
                    // Apply function with index - simplified implementation
                    // In full implementation, this would call the function with (index, value)
                    result.push(JsonnetValue::array(vec![JsonnetValue::number(i as f64), item.clone()]));
                }
                Ok(JsonnetValue::array(result))
            }
            JsonnetValue::String(s) => {
                let mut result = Vec::new();
                for (i, ch) in s.chars().enumerate() {
                    result.push(JsonnetValue::array(vec![
                        JsonnetValue::number(i as f64),
                        JsonnetValue::string(ch.to_string())
                    ]));
                }
                Ok(JsonnetValue::array(result))
            }
            _ => Err(JsonnetError::runtime_error("mapWithIndex expects array or string as second argument")),
        }
    }

    fn lstrip_chars(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "lstripChars")?;
        let str_val = args[0].as_string()?;
        let chars_val = args[1].as_string()?;

        let chars_set: std::collections::HashSet<char> = chars_val.chars().collect();
        let result: String = str_val.chars()
            .skip_while(|c| chars_set.contains(c))
            .collect();

        Ok(JsonnetValue::string(result))
    }

    fn rstrip_chars(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "rstripChars")?;
        let str_val = args[0].as_string()?;
        let chars_val = args[1].as_string()?;

        let chars_set: std::collections::HashSet<char> = chars_val.chars().collect();
        let result: String = str_val.chars()
            .rev()
            .skip_while(|c| chars_set.contains(c))
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect();

        Ok(JsonnetValue::string(result))
    }

    fn strip_chars(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "stripChars")?;
        let str_val = &args[0];
        let chars_val = &args[1];

        // First apply lstripChars, then rstripChars
        let lstripped_args = vec![str_val.clone(), chars_val.clone()];
        let lstripped = Self::lstrip_chars(lstripped_args)?;
        let rstripped_args = vec![lstripped, chars_val.clone()];
        Self::rstrip_chars(rstripped_args)
    }

    fn find_substr(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "findSubstr")?;
        let pat = args[0].as_string()?;
        let str = args[1].as_string()?;

        if pat.is_empty() {
            return Err(JsonnetError::runtime_error("findSubstr pattern cannot be empty"));
        }

        let mut result = Vec::new();
        let mut start = 0;

        while let Some(pos) = str[start..].find(&pat) {
            result.push(JsonnetValue::number((start + pos) as f64));
            start += pos + pat.len();
        }

        Ok(JsonnetValue::array(result))
    }

    fn repeat(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "repeat")?;
        let what = &args[0];
        let count_val = &args[1];

        let count = if let JsonnetValue::Number(n) = count_val {
            *n as usize
        } else {
            return Err(JsonnetError::runtime_error("repeat count must be a number"));
        };

        match what {
            JsonnetValue::String(s) => {
                let repeated = s.repeat(count);
                Ok(JsonnetValue::string(repeated))
            }
            JsonnetValue::Array(arr) => {
                let mut result = Vec::new();
                for _ in 0..count {
                    result.extend(arr.clone());
                }
                Ok(JsonnetValue::array(result))
            }
            _ => Err(JsonnetError::runtime_error("repeat first argument must be string or array")),
        }
    }

    fn set(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "set")?;
        match &args[0] {
            JsonnetValue::Array(arr) => {
                // Remove duplicates while preserving order
                let mut result = Vec::new();

                for item in arr {
                    // Check if item is already in result
                    let mut found = false;
                    for existing in &result {
                        if existing == item {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        result.push(item.clone());
                    }
                }

                Ok(JsonnetValue::array(result))
            }
            _ => Err(JsonnetError::runtime_error("set expects an array argument")),
        }
    }

    fn set_member(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "setMember")?;
        let value = &args[0];
        let arr = match &args[1] {
            JsonnetValue::Array(a) => a,
            _ => return Err(JsonnetError::runtime_error("setMember expects array as second argument")),
        };

        for item in arr {
            if item == value {
                return Ok(JsonnetValue::boolean(true));
            }
        }
        Ok(JsonnetValue::boolean(false))
    }

    fn set_union(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "setUnion")?;
        let arr_a = match &args[0] {
            JsonnetValue::Array(a) => a,
            _ => return Err(JsonnetError::runtime_error("setUnion expects arrays as arguments")),
        };
        let arr_b = match &args[1] {
            JsonnetValue::Array(a) => a,
            _ => return Err(JsonnetError::runtime_error("setUnion expects arrays as arguments")),
        };

        let mut result = Vec::new();

        // Add all elements from first array (preserving order)
        for item in arr_a {
            let mut found = false;
            for existing in &result {
                if existing == item {
                    found = true;
                    break;
                }
            }
            if !found {
                result.push(item.clone());
            }
        }

        // Add elements from second array that aren't already in result
        for item in arr_b {
            let mut found = false;
            for existing in &result {
                if existing == item {
                    found = true;
                    break;
                }
            }
            if !found {
                result.push(item.clone());
            }
        }

        Ok(JsonnetValue::array(result))
    }

    fn set_inter(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "setInter")?;
        let arr_a = match &args[0] {
            JsonnetValue::Array(a) => a,
            _ => return Err(JsonnetError::runtime_error("setInter expects arrays as arguments")),
        };
        let arr_b = match &args[1] {
            JsonnetValue::Array(a) => a,
            _ => return Err(JsonnetError::runtime_error("setInter expects arrays as arguments")),
        };

        let mut result = Vec::new();

        for item_a in arr_a {
            // Check if item_a exists in arr_b
            let mut found_in_b = false;
            for item_b in arr_b {
                if item_a == item_b {
                    found_in_b = true;
                    break;
                }
            }

            if found_in_b {
                // Check if item_a is already in result
                let mut already_in_result = false;
                for existing in &result {
                    if existing == item_a {
                        already_in_result = true;
                        break;
                    }
                }
                if !already_in_result {
                    result.push(item_a.clone());
                }
            }
        }

        Ok(JsonnetValue::array(result))
    }

    fn set_diff(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "setDiff")?;
        let arr_a = match &args[0] {
            JsonnetValue::Array(a) => a,
            _ => return Err(JsonnetError::runtime_error("setDiff expects arrays as arguments")),
        };
        let arr_b = match &args[1] {
            JsonnetValue::Array(a) => a,
            _ => return Err(JsonnetError::runtime_error("setDiff expects arrays as arguments")),
        };

        let mut result = Vec::new();

        for item_a in arr_a {
            // Check if item_a does NOT exist in arr_b
            let mut found_in_b = false;
            for item_b in arr_b {
                if item_a == item_b {
                    found_in_b = true;
                    break;
                }
            }

            if !found_in_b {
                // Check if item_a is already in result
                let mut already_in_result = false;
                for existing in &result {
                    if existing == item_a {
                        already_in_result = true;
                        break;
                    }
                }
                if !already_in_result {
                    result.push(item_a.clone());
                }
            }
        }

        Ok(JsonnetValue::array(result))
    }

    // Phase 4: Advanced Features

    // Manifest functions
    fn manifest_ini(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "manifestIni")?;
        // Simplified INI format - convert object to INI-like format
        match &args[0] {
            JsonnetValue::Object(obj) => {
                let mut result = String::new();
                for (key, value) in obj {
                    if !key.starts_with('_') {
                        result.push_str(&format!("[{}]\n", key));
                        if let JsonnetValue::Object(section) = value {
                            for (k, v) in section {
                                if !k.starts_with('_') {
                                    result.push_str(&format!("{}={}\n", k, v));
                                }
                            }
                        }
                        result.push('\n');
                    }
                }
                Ok(JsonnetValue::string(result.trim().to_string()))
            }
            _ => Err(JsonnetError::runtime_error("manifestIni expects an object")),
        }
    }

    fn manifest_python(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "manifestPython")?;
        // Generate Python dict representation
        let json_str = serde_json::to_string(&args[0].to_json_value())?;
        // Simple conversion - replace JSON syntax with Python dict syntax
        let python_str = json_str
            .replace("null", "None")
            .replace("true", "True")
            .replace("false", "False");
        Ok(JsonnetValue::string(python_str))
    }

    fn manifest_cpp(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "manifestCpp")?;
        // Simplified C++ code generation
        let json_str = serde_json::to_string(&args[0].to_json_value())?;
        let cpp_str = format!("// Generated C++ code\nconst char* jsonData = R\"json(\n{}\n)json\";", json_str);
        Ok(JsonnetValue::string(cpp_str))
    }

    fn manifest_xml_jsonml(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "manifestXmlJsonml")?;
        // JsonML format: [tagName, {attributes}, ...children]
        match &args[0] {
            JsonnetValue::Array(arr) if !arr.is_empty() => {
                if let JsonnetValue::String(tag) = &arr[0] {
                    let mut xml = format!("<{}", tag);

                    // Attributes (second element if it's an object)
                    let mut child_start = 1;
                    if arr.len() > 1 {
                        if let JsonnetValue::Object(attrs) = &arr[1] {
                            for (key, value) in attrs {
                                if !key.starts_with('_') {
                                    let value_str = match value {
                                        JsonnetValue::String(s) => s.clone(),
                                        _ => format!("{}", value),
                                    };
                                    xml.push_str(&format!(" {}=\"{}\"", key, value_str));
                                }
                            }
                            child_start = 2;
                        }
                    }

                    xml.push('>');

                    // Children
                    for child in &arr[child_start..] {
                        match child {
                            JsonnetValue::String(s) => xml.push_str(s),
                            JsonnetValue::Array(_) => {
                                // Recursively process child arrays
                                let child_xml = Self::manifest_xml_jsonml(vec![child.clone()])?;
                                if let JsonnetValue::String(child_str) = child_xml {
                                    xml.push_str(&child_str);
                                }
                            }
                            _ => xml.push_str(&format!("{}", child)),
                        }
                    }

                    xml.push_str(&format!("</{}>", tag));
                    Ok(JsonnetValue::string(xml))
                } else {
                    Err(JsonnetError::runtime_error("JsonML array must start with string tag name"))
                }
            }
            _ => Err(JsonnetError::runtime_error("manifestXmlJsonml expects a JsonML array")),
        }
    }

    // Advanced math functions
    fn log2(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "log2")?;
        let x = args[0].as_number()?;
        if x <= 0.0 {
            return Err(JsonnetError::runtime_error("log2 of non-positive number"));
        }
        Ok(JsonnetValue::number(x.log2()))
    }

    fn log10(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "log10")?;
        let x = args[0].as_number()?;
        if x <= 0.0 {
            return Err(JsonnetError::runtime_error("log10 of non-positive number"));
        }
        Ok(JsonnetValue::number(x.log10()))
    }

    fn log1p(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "log1p")?;
        let x = args[0].as_number()?;
        if x < -1.0 {
            return Err(JsonnetError::runtime_error("log1p of number less than -1"));
        }
        Ok(JsonnetValue::number((x + 1.0).ln()))
    }

    fn expm1(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "expm1")?;
        let x = args[0].as_number()?;
        Ok(JsonnetValue::number(x.exp() - 1.0))
    }

    // Phase 5: Remaining Core Functions

    // Array manipulation functions
    fn remove(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "remove")?;
        let arr = args[0].as_array()?;
        let value_to_remove = &args[1];

        let filtered: Vec<JsonnetValue> = arr.iter()
            .filter(|item| !item.equals(value_to_remove))
            .cloned()
            .collect();

        Ok(JsonnetValue::array(filtered))
    }

    fn remove_at(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "removeAt")?;
        let arr = args[0].as_array()?;
        let index = args[1].as_number()? as usize;

        if index >= arr.len() {
            return Err(JsonnetError::runtime_error("Index out of bounds"));
        }

        let mut result = arr.clone();
        result.remove(index);
        Ok(JsonnetValue::array(result))
    }

    fn flatten_arrays(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "flattenArrays")?;
        let arr = args[0].as_array()?;

        let mut result = Vec::new();
        Self::flatten_array_recursive(arr, &mut result);
        Ok(JsonnetValue::array(result))
    }

    fn flatten_array_recursive(arr: &[JsonnetValue], result: &mut Vec<JsonnetValue>) {
        for item in arr {
            match item {
                JsonnetValue::Array(sub_arr) => {
                    Self::flatten_array_recursive(sub_arr, result);
                }
                _ => result.push(item.clone()),
            }
        }
    }

    // Object manipulation functions
    fn object_keys_values(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "objectKeysValues")?;
        let obj = args[0].as_object()?;

        let mut result = Vec::new();
        for (key, value) in obj {
            if !key.starts_with('_') {
                result.push(JsonnetValue::object(HashMap::from([
                    ("key".to_string(), JsonnetValue::string(key.clone())),
                    ("value".to_string(), value.clone()),
                ])));
            }
        }

        Ok(JsonnetValue::array(result))
    }

    fn object_remove_key(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 2, "objectRemoveKey")?;
        let obj = args[0].as_object()?;
        let key_to_remove = args[1].as_string()?;

        let mut result = obj.clone();
        result.remove(key_to_remove);
        Ok(JsonnetValue::object(result))
    }

    // Additional type checking functions
    fn is_integer(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isInteger")?;
        match &args[0] {
            JsonnetValue::Number(n) => Ok(JsonnetValue::boolean(n.fract() == 0.0)),
            _ => Ok(JsonnetValue::boolean(false)),
        }
    }

    fn is_decimal(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isDecimal")?;
        match &args[0] {
            JsonnetValue::Number(n) => Ok(JsonnetValue::boolean(n.fract() != 0.0)),
            _ => Ok(JsonnetValue::boolean(false)),
        }
    }

    fn is_even(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isEven")?;
        match &args[0] {
            JsonnetValue::Number(n) if n.fract() == 0.0 => {
                Ok(JsonnetValue::boolean((*n as i64) % 2 == 0))
            }
            _ => Ok(JsonnetValue::boolean(false)),
        }
    }

    fn is_odd(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        Self::check_args(&args, 1, "isOdd")?;
        match &args[0] {
            JsonnetValue::Number(n) if n.fract() == 0.0 => {
                Ok(JsonnetValue::boolean((*n as i64) % 2 != 0))
            }
            _ => Ok(JsonnetValue::boolean(false)),
        }
    }

    /// Helper function to check argument count
    fn check_args(args: &[JsonnetValue], expected: usize, func_name: &str) -> Result<()> {
        if args.len() != expected {
            return Err(JsonnetError::invalid_function_call(format!(
                "{}() expects {} arguments, got {}",
                func_name, expected, args.len()
            )));
        }
        Ok(())
    }

    /// Helper function to check argument count range
    fn check_args_range(args: &[JsonnetValue], min: usize, max: usize, func_name: &str) -> Result<()> {
        if args.len() < min || args.len() > max {
            return Err(JsonnetError::invalid_function_call(format!(
                "{}() expects {} to {} arguments, got {}",
                func_name, min, max, args.len()
            )));
        }
        Ok(())
    }

    /// Call a standard library function (static method)
    pub fn call_function(name: &str, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        // Simple implementation - just return an error for now
        // This should dispatch to the appropriate std function
        Err(JsonnetError::runtime_error(format!("StdLib function '{}' not implemented", name)))
    }
}

impl JsonnetValue {
    /// Convert from serde_json::Value to JsonnetValue
    pub fn from_json_value(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => JsonnetValue::Null,
            serde_json::Value::Bool(b) => JsonnetValue::boolean(b),
            serde_json::Value::Number(n) => JsonnetValue::number(n.as_f64().unwrap_or(0.0)),
            serde_json::Value::String(s) => JsonnetValue::string(s),
            serde_json::Value::Array(arr) => {
                let jsonnet_arr: Vec<JsonnetValue> = arr.into_iter()
                    .map(JsonnetValue::from_json_value)
                    .collect();
                JsonnetValue::array(jsonnet_arr)
            }
            serde_json::Value::Object(obj) => {
                let mut jsonnet_obj = HashMap::new();
                for (k, v) in obj {
                    jsonnet_obj.insert(k, JsonnetValue::from_json_value(v));
                }
                JsonnetValue::object(jsonnet_obj)
            }
        }
    }

    // ===== NEW FUNCTIONS FOR COMPLETE COMPATIBILITY =====

    /// slice(array|string, start, [end]) - Extract slice from array or string
    pub fn slice(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "slice")?;
        let start = args[1].as_number()? as usize;

        match &args[0] {
            JsonnetValue::Array(arr) => {
                let end = if args.len() > 2 {
                    args[2].as_number()? as usize
                } else {
                    arr.len()
                };
                let start = start.min(arr.len());
                let end = end.min(arr.len());
                if start > end {
                    Ok(JsonnetValue::array(vec![]))
                } else {
                    Ok(JsonnetValue::array(arr[start..end].to_vec()))
                }
            }
            JsonnetValue::String(s) => {
                let end = if args.len() > 2 {
                    args[2].as_number()? as usize
                } else {
                    s.chars().count()
                };
                let chars: Vec<char> = s.chars().collect();
                let start = start.min(chars.len());
                let end = end.min(chars.len());
                if start > end {
                    Ok(JsonnetValue::string("".to_string()))
                } else {
                    let sliced: String = chars[start..end].iter().collect();
                    Ok(JsonnetValue::string(sliced))
                }
            }
            _ => Err(JsonnetError::invalid_function_call("slice() expects array or string as first argument".to_string())),
        }
    }

    /// zip(arrays...) - Zip multiple arrays together
    pub fn zip(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        if args.is_empty() {
            return Err(JsonnetError::invalid_function_call("zip() expects at least one argument".to_string()));
        }

        // Convert all arguments to arrays
        let arrays: Result<Vec<Vec<JsonnetValue>>> = args.into_iter()
            .map(|arg| arg.as_array().cloned())
            .collect();

        let arrays = arrays?;
        if arrays.is_empty() {
            return Ok(JsonnetValue::array(vec![]));
        }

        // Find minimum length
        let min_len = arrays.iter().map(|arr| arr.len()).min().unwrap_or(0);

        // Create zipped result
        let mut result = Vec::new();
        for i in 0..min_len {
            let mut tuple = Vec::new();
            for arr in &arrays {
                tuple.push(arr[i].clone());
            }
            result.push(JsonnetValue::array(tuple));
        }

        Ok(JsonnetValue::array(result))
    }

    /// transpose(matrix) - Transpose a matrix (array of arrays)
    pub fn transpose(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "transpose")?;
        let matrix = args[0].as_array()?;

        if matrix.is_empty() {
            return Ok(JsonnetValue::array(vec![]));
        }

        // Check if all elements are arrays and get dimensions
        let mut max_len = 0;
        for row in matrix {
            match row {
                JsonnetValue::Array(arr) => {
                    max_len = max_len.max(arr.len());
                }
                _ => return Err(JsonnetError::invalid_function_call("transpose() expects array of arrays".to_string())),
            }
        }

        if max_len == 0 {
            return Ok(JsonnetValue::array(vec![]));
        }

        // Create transposed matrix
        let mut result = Vec::new();
        for col in 0..max_len {
            let mut new_row = Vec::new();
            for row in matrix {
                if let JsonnetValue::Array(arr) = row {
                    if col < arr.len() {
                        new_row.push(arr[col].clone());
                    } else {
                        new_row.push(JsonnetValue::Null);
                    }
                }
            }
            result.push(JsonnetValue::array(new_row));
        }

        Ok(JsonnetValue::array(result))
    }

    /// flatten(array, [depth]) - Completely flatten nested arrays
    pub fn flatten(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "flatten")?;
        let depth = if args.len() > 1 {
            args[1].as_number()? as usize
        } else {
            usize::MAX
        };

        fn flatten_recursive(arr: &Vec<JsonnetValue>, current_depth: usize, max_depth: usize) -> Vec<JsonnetValue> {
            let mut result = Vec::new();
            for item in arr {
                match item {
                    JsonnetValue::Array(nested) if current_depth < max_depth => {
                        result.extend(flatten_recursive(nested, current_depth + 1, max_depth));
                    }
                    _ => result.push(item.clone()),
                }
            }
            result
        }

        let arr = args[0].as_array()?;
        let flattened = flatten_recursive(arr, 0, depth);
        Ok(JsonnetValue::array(flattened))
    }

    /// sum(array) - Sum all numbers in array
    pub fn sum(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "sum")?;
        let arr = args[0].as_array()?;

        let mut total = 0.0;
        for item in arr {
            total += item.as_number()?;
        }

        Ok(JsonnetValue::number(total))
    }

    /// product(array) - Product of all numbers in array
    pub fn product(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "product")?;
        let arr = args[0].as_array()?;

        let mut result = 1.0;
        for item in arr {
            result *= item.as_number()?;
        }

        Ok(JsonnetValue::number(result))
    }

    /// all(array) - Check if all elements are truthy
    pub fn all(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "all")?;
        let arr = args[0].as_array()?;

        for item in arr {
            if !item.is_truthy() {
                return Ok(JsonnetValue::boolean(false));
            }
        }

        Ok(JsonnetValue::boolean(true))
    }

    /// any(array) - Check if any element is truthy
    pub fn any(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "any")?;
        let arr = args[0].as_array()?;

        for item in arr {
            if item.is_truthy() {
                return Ok(JsonnetValue::boolean(true));
            }
        }

        Ok(JsonnetValue::boolean(false))
    }

    /// sortBy(array, keyFunc) - Sort array by key function
    pub fn sort_by(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "sortBy")?;
        let _arr = args[0].as_array()?.clone();
        let _key_func = &args[1];

        // For now, implement a simple version that assumes the key function returns numbers
        // Full implementation would require function calling callback
        Err(JsonnetError::runtime_error("sortBy() requires function calling mechanism - placeholder implementation".to_string()))
    }

    /// groupBy(array, keyFunc) - Group array elements by key function
    pub fn group_by(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "groupBy")?;
        // Placeholder implementation
        Err(JsonnetError::runtime_error("groupBy() requires function calling mechanism - placeholder implementation".to_string()))
    }

    /// partition(array, predFunc) - Partition array by predicate function
    pub fn partition(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "partition")?;
        // Placeholder implementation
        Err(JsonnetError::runtime_error("partition() requires function calling mechanism - placeholder implementation".to_string()))
    }

    /// chunk(array, size) - Split array into chunks of given size
    pub fn chunk(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "chunk")?;
        let arr = args[0].as_array()?;
        let size = args[1].as_number()? as usize;

        if size == 0 {
            return Err(JsonnetError::invalid_function_call("chunk() size must be positive".to_string()));
        }

        let mut result = Vec::new();
        for chunk in arr.chunks(size) {
            result.push(JsonnetValue::array(chunk.to_vec()));
        }

        Ok(JsonnetValue::array(result))
    }

    /// unique(array) - Remove duplicates from array (alternative to uniq)
    pub fn unique(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "unique")?;
        let arr = args[0].as_array()?;

        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();

        for item in arr {
            // Simple equality check - in real Jsonnet this uses deep equality
            if !seen.contains(&format!("{:?}", item)) {
                seen.insert(format!("{:?}", item));
                result.push(item.clone());
            }
        }

        Ok(JsonnetValue::array(result))
    }

    /// difference(arrays...) - Set difference of arrays
    pub fn difference(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        if args.is_empty() {
            return Ok(JsonnetValue::array(vec![]));
        }

        let first = args[0].as_array()?;
        let mut result = first.clone();

        for arg in &args[1..] {
            let other = arg.as_array()?;
            let other_set: std::collections::HashSet<String> = other.iter()
                .map(|v| format!("{:?}", v))
                .collect();

            result.retain(|item| !other_set.contains(&format!("{:?}", item)));
        }

        Ok(JsonnetValue::array(result))
    }

    /// intersection(arrays...) - Set intersection of arrays
    pub fn intersection(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        if args.is_empty() {
            return Ok(JsonnetValue::array(vec![]));
        }

        let first = args[0].as_array()?;
        let mut result = first.clone();

        for arg in &args[1..] {
            let other = arg.as_array()?;
            let other_set: std::collections::HashSet<String> = other.iter()
                .map(|v| format!("{:?}", v))
                .collect();

            result.retain(|item| other_set.contains(&format!("{:?}", item)));
        }

        Ok(JsonnetValue::array(result))
    }

    /// symmetricDifference(a, b) - Symmetric difference of two arrays
    pub fn symmetric_difference(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "symmetricDifference")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let a_set: std::collections::HashSet<String> = a.iter()
            .map(|v| format!("{:?}", v))
            .collect();
        let b_set: std::collections::HashSet<String> = b.iter()
            .map(|v| format!("{:?}", v))
            .collect();

        let symmetric_diff: std::collections::HashSet<_> = a_set.symmetric_difference(&b_set).cloned().collect();

        let mut result: Vec<JsonnetValue> = a.iter()
            .filter(|item| symmetric_diff.contains(&format!("{:?}", item)))
            .chain(b.iter().filter(|item| symmetric_diff.contains(&format!("{:?}", item))))
            .cloned()
            .collect();

        Ok(JsonnetValue::array(result))
    }

    /// isSubset(a, b) - Check if a is subset of b
    pub fn is_subset(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "isSubset")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let b_set: std::collections::HashSet<String> = b.iter()
            .map(|v| format!("{:?}", v))
            .collect();

        let is_subset = a.iter().all(|item| b_set.contains(&format!("{:?}", item)));

        Ok(JsonnetValue::boolean(is_subset))
    }

    /// isSuperset(a, b) - Check if a is superset of b
    pub fn is_superset(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "isSuperset")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let a_set: std::collections::HashSet<String> = a.iter()
            .map(|v| format!("{:?}", v))
            .collect();

        let is_superset = b.iter().all(|item| a_set.contains(&format!("{:?}", item)));

        Ok(JsonnetValue::boolean(is_superset))
    }

    /// isDisjoint(a, b) - Check if a and b are disjoint
    pub fn is_disjoint(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "isDisjoint")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let a_set: std::collections::HashSet<String> = a.iter()
            .map(|v| format!("{:?}", v))
            .collect();
        let b_set: std::collections::HashSet<String> = b.iter()
            .map(|v| format!("{:?}", v))
            .collect();

        let is_disjoint = a_set.intersection(&b_set).count() == 0;

        Ok(JsonnetValue::boolean(is_disjoint))
    }

    /// cartesian(arrays) - Cartesian product of arrays
    pub fn cartesian(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "cartesian")?;
        let arrays = args[0].as_array()?;

        if arrays.is_empty() {
            return Ok(JsonnetValue::array(vec![JsonnetValue::array(vec![])]));
        }

        // Convert to vectors
        let mut vec_arrays = Vec::new();
        for arr in arrays {
            vec_arrays.push(arr.as_array()?.clone());
        }

        fn cartesian_product(arrays: &[Vec<JsonnetValue>]) -> Vec<Vec<JsonnetValue>> {
            if arrays.is_empty() {
                return vec![vec![]];
            }

            let mut result = Vec::new();
            let first = &arrays[0];
            let rest = &arrays[1..];

            for item in first {
                for mut combo in cartesian_product(rest) {
                    combo.insert(0, item.clone());
                    result.push(combo);
                }
            }

            result
        }

        let products = cartesian_product(&vec_arrays);
        let result: Vec<JsonnetValue> = products.into_iter()
            .map(|combo| JsonnetValue::array(combo))
            .collect();

        Ok(JsonnetValue::array(result))
    }

    /// cross(a, b) - Cross product of two arrays
    pub fn cross(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "cross")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        let mut result = Vec::new();
        for item_a in a {
            for item_b in b {
                result.push(JsonnetValue::array(vec![item_a.clone(), item_b.clone()]));
            }
        }

        Ok(JsonnetValue::array(result))
    }

    /// dot(a, b) - Dot product of two numeric arrays
    pub fn dot(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "dot")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        if a.len() != b.len() {
            return Err(JsonnetError::invalid_function_call("dot() arrays must have same length".to_string()));
        }

        let mut sum = 0.0;
        for (x, y) in a.iter().zip(b.iter()) {
            sum += x.as_number()? * y.as_number()?;
        }

        Ok(JsonnetValue::number(sum))
    }

    /// norm(array) - Euclidean norm of numeric array
    pub fn norm(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "norm")?;
        let arr = args[0].as_array()?;

        let mut sum_squares = 0.0;
        for item in arr {
            let val = item.as_number()?;
            sum_squares += val * val;
        }

        Ok(JsonnetValue::number(sum_squares.sqrt()))
    }

    /// normalize(array) - Normalize numeric array
    pub fn normalize(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "normalize")?;
        let arr = args[0].as_array()?;

        // Calculate norm directly to avoid recursion
        let mut sum_squares = 0.0;
        for item in arr {
            let val = item.as_number()?;
            sum_squares += val * val;
        }
        let norm_val = sum_squares.sqrt();
        if norm_val == 0.0 {
            return Ok(args[0].clone());
        }

        let mut result = Vec::new();
        for item in arr {
            let val = item.as_number()?;
            result.push(JsonnetValue::number(val / norm_val));
        }

        Ok(JsonnetValue::array(result))
    }

    /// distance(a, b) - Euclidean distance between two points
    pub fn distance(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "distance")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        if a.len() != b.len() {
            return Err(JsonnetError::invalid_function_call("distance() arrays must have same length".to_string()));
        }

        let mut sum_squares = 0.0;
        for (x, y) in a.iter().zip(b.iter()) {
            let diff = x.as_number()? - y.as_number()?;
            sum_squares += diff * diff;
        }

        Ok(JsonnetValue::number(sum_squares.sqrt()))
    }

    /// angle(a, b) - Angle between two vectors
    pub fn angle(&mut self, args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "angle")?;
        let a = args[0].as_array()?;
        let b = args[1].as_array()?;

        if a.len() != b.len() {
            return Err(JsonnetError::invalid_function_call("angle() arrays must have same length".to_string()));
        }

        // Calculate dot product directly
        let mut dot_product = 0.0;
        for (x, y) in a.iter().zip(b.iter()) {
            dot_product += x.as_number()? * y.as_number()?;
        }

        // Calculate norms directly
        let mut norm_a_sq = 0.0;
        for item in a {
            let val = item.as_number()?;
            norm_a_sq += val * val;
        }
        let norm_a = norm_a_sq.sqrt();

        let mut norm_b_sq = 0.0;
        for item in b {
            let val = item.as_number()?;
            norm_b_sq += val * val;
        }
        let norm_b = norm_b_sq.sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return Ok(JsonnetValue::number(0.0));
        }

        let cos_theta = dot_product / (norm_a * norm_b);
        let cos_theta = cos_theta.max(-1.0).min(1.0); // Clamp to avoid floating point errors

        Ok(JsonnetValue::number(cos_theta.acos()))
    }

    /// rotate(point, angle, [center]) - Rotate 2D point
    pub fn rotate(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "rotate")?;
        let point = args[0].as_array()?;
        let angle = args[1].as_number()?;

        if point.len() != 2 {
            return Err(JsonnetError::invalid_function_call("rotate() point must be 2D".to_string()));
        }

        let center = if args.len() > 2 {
            args[2].as_array()?.to_vec()
        } else {
            vec![JsonnetValue::number(0.0), JsonnetValue::number(0.0)]
        };

        if center.len() != 2 {
            return Err(JsonnetError::invalid_function_call("rotate() center must be 2D".to_string()));
        }

        let x = point[0].as_number()? - center[0].as_number()?;
        let y = point[1].as_number()? - center[1].as_number()?;

        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let new_x = x * cos_a - y * sin_a + center[0].as_number()?;
        let new_y = x * sin_a + y * cos_a + center[1].as_number()?;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    /// scale(point, factor, [center]) - Scale 2D point
    pub fn scale(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "scale")?;
        let point = args[0].as_array()?;
        let factor = args[1].as_number()?;

        if point.len() != 2 {
            return Err(JsonnetError::invalid_function_call("scale() point must be 2D".to_string()));
        }

        let center = if args.len() > 2 {
            args[2].as_array()?.to_vec()
        } else {
            vec![JsonnetValue::number(0.0), JsonnetValue::number(0.0)]
        };

        if center.len() != 2 {
            return Err(JsonnetError::invalid_function_call("scale() center must be 2D".to_string()));
        }

        let x = point[0].as_number()? - center[0].as_number()?;
        let y = point[1].as_number()? - center[1].as_number()?;

        let new_x = x * factor + center[0].as_number()?;
        let new_y = y * factor + center[1].as_number()?;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    /// translate(point, offset) - Translate 2D point
    pub fn translate(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "translate")?;
        let point = args[0].as_array()?;
        let offset = args[1].as_array()?;

        if point.len() != 2 || offset.len() != 2 {
            return Err(JsonnetError::invalid_function_call("translate() requires 2D point and offset".to_string()));
        }

        let new_x = point[0].as_number()? + offset[0].as_number()?;
        let new_y = point[1].as_number()? + offset[1].as_number()?;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    /// reflect(point, axis) - Reflect 2D point over axis
    pub fn reflect(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "reflect")?;
        let point = args[0].as_array()?;
        let axis = args[1].as_number()?; // angle of reflection axis in radians

        if point.len() != 2 {
            return Err(JsonnetError::invalid_function_call("reflect() point must be 2D".to_string()));
        }

        let x = point[0].as_number()?;
        let y = point[1].as_number()?;

        let cos_2a = (2.0 * axis).cos();
        let sin_2a = (2.0 * axis).sin();

        let new_x = x * cos_2a + y * sin_2a;
        let new_y = x * sin_2a - y * cos_2a;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    /// affine(point, matrix) - Apply affine transformation
    pub fn affine(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "affine")?;
        let point = args[0].as_array()?;
        let matrix = args[1].as_array()?;

        if point.len() != 2 {
            return Err(JsonnetError::invalid_function_call("affine() point must be 2D".to_string()));
        }

        if matrix.len() != 6 {
            return Err(JsonnetError::invalid_function_call("affine() matrix must be 6 elements [a,b,c,d,e,f]".to_string()));
        }

        let x = point[0].as_number()?;
        let y = point[1].as_number()?;

        let a = matrix[0].as_number()?;
        let b = matrix[1].as_number()?;
        let c = matrix[2].as_number()?;
        let d = matrix[3].as_number()?;
        let e = matrix[4].as_number()?;
        let f = matrix[5].as_number()?;

        let new_x = a * x + b * y + e;
        let new_y = c * x + d * y + f;

        Ok(JsonnetValue::array(vec![JsonnetValue::number(new_x), JsonnetValue::number(new_y)]))
    }

    /// splitLimit(string, sep, limit) - Split string with limit
    pub fn split_limit(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 3, "splitLimit")?;
        let s = args[0].as_string()?;
        let sep = args[1].as_string()?;
        let limit = args[2].as_number()? as usize;

        if sep.is_empty() {
            // Split into characters
            let chars: Vec<String> = s.chars().take(limit).map(|c| c.to_string()).collect();
            let result: Vec<JsonnetValue> = chars.into_iter().map(JsonnetValue::string).collect();
            return Ok(JsonnetValue::array(result));
        }

        let mut parts: Vec<&str> = s.splitn(limit + 1, &sep).collect();
        if parts.len() > limit {
            // Join the remaining parts
            let remaining = parts.split_off(limit);
            parts.push(&s[(s.len() - remaining.join(&sep).len())..]);
        }

        let result: Vec<JsonnetValue> = parts.into_iter().map(|s| JsonnetValue::string(s.to_string())).collect();
        Ok(JsonnetValue::array(result))
    }

    /// join(sep, arrays...) - Join arrays with separator (variadic version)
    pub fn join_variadic(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        if args.is_empty() {
            return Err(JsonnetError::invalid_function_call("join() expects at least one argument".to_string()));
        }

        let sep = args[0].as_string()?;
        let arrays: Result<Vec<Vec<JsonnetValue>>> = args[1..].iter()
            .map(|arg| arg.as_array().cloned())
            .collect();

        let arrays = arrays?;
        let mut result = Vec::new();

        for (i, arr) in arrays.iter().enumerate() {
            if i > 0 && !sep.is_empty() {
                result.push(JsonnetValue::string(sep.clone()));
            }
            result.extend(arr.iter().cloned());
        }

        Ok(JsonnetValue::array(result))
    }

    /// replace(string, old, new) - Replace all occurrences
    pub fn replace(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 3, "replace")?;
        let s = args[0].as_string()?;
        let old = args[1].as_string()?;
        let new = args[2].as_string()?;

        let result = s.replace(&old, &new);
        Ok(JsonnetValue::string(result))
    }

    /// contains(container, element) - Check if container contains element
    pub fn contains_variadic(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "contains")?;

        match &args[0] {
            JsonnetValue::Array(arr) => {
                // Simple linear search with string comparison
                let target = format!("{:?}", &args[1]);
                for item in arr {
                    if format!("{:?}", item) == target {
                        return Ok(JsonnetValue::boolean(true));
                    }
                }
                Ok(JsonnetValue::boolean(false))
            }
            JsonnetValue::String(s) => {
                let substr = args[1].as_string()?;
                Ok(JsonnetValue::boolean(s.contains(&substr)))
            }
            JsonnetValue::Object(obj) => {
                let key = args[1].as_string()?;
                Ok(JsonnetValue::boolean(obj.contains_key(&*key)))
            }
            _ => Err(JsonnetError::invalid_function_call("contains() expects array, string, or object".to_string())),
        }
    }

    // ==========================================
    // AI Agent Functions (Manimani)
    // ==========================================

    /// ai.httpGet(url, headers={}) - Make HTTP GET request
    pub fn ai_http_get(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args_range(&args, 1, 2, "ai.httpGet")?;
        let url = args[0].as_string()?;
        let headers = if args.len() > 1 {
            args[1].as_object()?.clone()
        } else {
            HashMap::new()
        };

        // This would be implemented as an external function call
        // For now, return a placeholder
        let result = json!({
            "url": url,
            "method": "GET",
            "headers": headers,
            "status": "pending",
            "body": "HTTP request will be executed by runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// ai.httpPost(url, body, headers={}) - Make HTTP POST request
    pub fn ai_http_post(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args_range(&args, 2, 3, "ai.httpPost")?;
        let url = args[0].as_string()?;
        let body = args[1].clone();
        let headers = if args.len() > 2 {
            args[2].as_object()?.clone()
        } else {
            HashMap::new()
        };

        // This would be implemented as an external function call
        let result = json!({
            "url": url,
            "method": "POST",
            "body": body,
            "headers": headers,
            "status": "pending"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// ai.callModel(model, messages, options={}) - Call AI model
    pub fn ai_call_model(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args_range(&args, 2, 3, "ai.callModel")?;
        let model = args[0].as_string()?;
        let messages = args[1].as_array()?.clone();
        let options = if args.len() > 2 {
            args[2].as_object()?.clone()
        } else {
            HashMap::new()
        };

        // This would call the AI model API
        let result = json!({
            "model": model,
            "messages": messages,
            "options": options,
            "status": "pending",
            "response": "AI model response will be generated by runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// tool.execute(command, args=[], env={}) - Execute external command
    pub fn tool_execute(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args_range(&args, 1, 3, "tool.execute")?;
        let command = args[0].as_string()?;
        let cmd_args = if args.len() > 1 {
            args[1].as_array()?.clone()
        } else {
            Vec::new()
        };
        let env = if args.len() > 2 {
            args[2].as_object()?.clone()
        } else {
            HashMap::new()
        };

        // This would execute the external command
        let result = json!({
            "command": command,
            "args": cmd_args,
            "env": env,
            "status": "pending",
            "output": "Command will be executed by runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// memory.get(key) - Get value from memory
    pub fn memory_get(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "memory.get")?;
        let key = args[0].as_string()?;

        // This would retrieve from memory store
        let result = json!({
            "key": key,
            "operation": "get",
            "status": "pending",
            "value": null
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// memory.set(key, value) - Set value in memory
    pub fn memory_set(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "memory.set")?;
        let key = args[0].as_string()?;
        let value = args[1].clone();

        // This would store in memory store
        let result = json!({
            "key": key,
            "value": value,
            "operation": "set",
            "status": "pending"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// agent.create(type, config) - Create an AI agent
    pub fn agent_create(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "agent.create")?;
        let agent_type = args[0].as_string()?;
        let config = args[1].as_object()?.clone();

        // This would create an agent instance
        let result = json!({
            "type": agent_type,
            "config": config,
            "id": "agent_id_placeholder",
            "status": "created"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// agent.execute(agent, input) - Execute agent with input
    pub fn agent_execute(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "agent.execute")?;
        let agent = args[0].clone();
        let input = args[1].as_string()?;

        // This would execute the agent
        let result = json!({
            "agent": agent,
            "input": input,
            "status": "pending",
            "output": "Agent execution will be handled by runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// chain.create(steps) - Create a processing chain
    pub fn chain_create(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 1, "chain.create")?;
        let steps = args[0].as_array()?.clone();

        // This would create a processing chain
        let result = json!({
            "steps": steps,
            "id": "chain_id_placeholder",
            "status": "created"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

    /// chain.execute(chain, input) - Execute a processing chain
    pub fn chain_execute(args: Vec<JsonnetValue>) -> Result<JsonnetValue> {
        StdLib::check_args(&args, 2, "chain.execute")?;
        let chain = args[0].clone();
        let input = args[1].clone();

        // This would execute the chain
        let result = json!({
            "chain": chain,
            "input": input,
            "status": "pending",
            "output": "Chain execution will be handled by runtime"
        });
        Ok(JsonnetValue::from_json_value(result))
    }

}
