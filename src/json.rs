//! Minimal JSON value builder and serializer.
//!
//! `repohelix` avoids external crates, so this module provides just enough
//! JSON support to emit machine-readable reports. It handles objects, arrays,
//! strings, numbers, booleans and null with correct escaping and stable,
//! insertion-ordered object keys (important for deterministic output).

use std::fmt::Write as _;

/// A JSON value. Object key order is preserved as inserted so that report
/// output is byte-for-byte stable across runs on the same input.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    /// Integers are stored separately to avoid float formatting artifacts.
    Int(i64),
    Float(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    /// Convenience constructor for a string value.
    pub fn str(s: impl Into<String>) -> Json {
        Json::Str(s.into())
    }

    /// Build an object from an ordered list of key/value pairs.
    pub fn obj(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }

    /// Serialize to a compact JSON string (no insignificant whitespace).
    pub fn to_compact(&self) -> String {
        let mut out = String::new();
        self.write_compact(&mut out);
        out
    }

    /// Serialize to a pretty-printed JSON string using two-space indentation.
    pub fn to_pretty(&self) -> String {
        let mut out = String::new();
        self.write_pretty(&mut out, 0);
        out
    }

    fn write_compact(&self, out: &mut String) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Int(n) => {
                let _ = write!(out, "{}", n);
            }
            Json::Float(f) => out.push_str(&format_float(*f)),
            Json::Str(s) => write_json_string(out, s),
            Json::Array(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    item.write_compact(out);
                }
                out.push(']');
            }
            Json::Object(pairs) => {
                out.push('{');
