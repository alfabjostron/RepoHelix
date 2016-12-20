//! Core data model for `repohelix`.
//!
//! These structures represent the parsed Git history and the derived
//! analytical views. Everything here is plain data; computation lives in
//! [`crate::metrics`].

/// A single file's change within one commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
    /// Repository-relative path of the file after the change.
    pub path: String,
    /// Lines added (from `git log --numstat`). `None` for binary files.
    pub added: Option<u64>,
    /// Lines removed. `None` for binary files.
    pub removed: Option<u64>,
}

impl FileChange {
    /// Total churn (added + removed) for this change, treating binary edits as
    /// a single unit of churn so they still register as activity.
    pub fn churn(&self) -> u64 {
        match (self.added, self.removed) {
            (Some(a), Some(r)) => a + r,
            _ => 1,
