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
        }
    }

    /// Whether this change is binary (no textual line counts).
    pub fn is_binary(&self) -> bool {
        self.added.is_none() && self.removed.is_none()
    }
}

/// One commit with its metadata and the files it touched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// Abbreviated or full commit hash.
    pub hash: String,
    /// Author display name.
    pub author_name: String,
    /// Author email (used as the stable identity key for ownership).
    pub author_email: String,
    /// Author timestamp as Unix epoch seconds.
    pub timestamp: i64,
    /// First line of the commit message (the subject).
    pub subject: String,
    /// Files changed by this commit.
    pub files: Vec<FileChange>,
}

impl Commit {
    /// The identity used for ownership aggregation. Email is preferred because
    /// it is more stable than display names, which vary across machines.
    pub fn identity(&self) -> &str {
        if self.author_email.is_empty() {
            &self.author_name
        } else {
            &self.author_email
        }
    }
}

/// The full parsed history: an ordered list of commits (newest first, as Git
/// emits them by default).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct History {
    pub commits: Vec<Commit>,
}

impl History {
