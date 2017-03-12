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
