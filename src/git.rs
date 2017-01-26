//! Non-interactive invocation of the local `git` binary.
//!
//! `repohelix` never opens an editor, pager, or prompt. All git calls set
//! environment variables and flags that guarantee batch behavior, so the tool
//! is safe to run inside scripts and CI.

use std::io;
use std::path::Path;
use std::process::Command;

/// The record separator injected between commits. It is an ASCII unit
/// separator (0x1f) followed by a sentinel token, chosen so it cannot appear
/// in normal commit metadata.
pub const COMMIT_SEP: &str = "\x1eREPOHELIX_COMMIT\x1e";

/// Field separator within a commit header line (ASCII unit separator).
pub const FIELD_SEP: char = '\x1f';

/// Errors that can occur while talking to git.
#[derive(Debug)]
pub enum GitError {
    /// The git process could not be spawned (e.g. git not installed).
    Spawn(io::Error),
    /// git ran but exited with a non-zero status.
    Failed { code: Option<i32>, stderr: String },
    /// Output was not valid UTF-8.
    Encoding,
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::Spawn(e) => write!(f, "could not run git: {e}"),
            GitError::Failed { code, stderr } => {
