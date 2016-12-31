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

