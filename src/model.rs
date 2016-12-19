//! Core data model for `repohelix`.
//!
//! These structures represent the parsed Git history and the derived
//! analytical views. Everything here is plain data; computation lives in
//! [`crate::metrics`].

/// A single file's change within one commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
