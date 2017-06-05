//! `repohelix` — a Git history architecture and temporal-coupling explorer.
//!
//! # What it does
//!
//! `repohelix` reads Git commit history (either from a live repository via the
//! local `git` binary, or from a captured *fixture log* file) and derives a set
//! of descriptive metrics about how the codebase evolves:
//!
//! - **Churn** — how much each file changes (lines added + removed).
//! - **Co-change / temporal coupling** — pairs of files that tend to change in
//!   the same commit, scored with a Jaccard-style strength and a directional
//!   confidence.
//! - **Hotspots** — files that are simultaneously high-churn and
//!   high-frequency, where maintenance effort concentrates.
//! - **Ownership concentration** — how change activity for a file is spread
//!   across author identities, as a bus-factor / knowledge-distribution signal.
//! - **Temporal clusters** — bursts of commits that approximate development
//!   sessions or release pushes.
//!
//! # A note on interpretation
//!
//! These metrics describe *files and change activity*, not people. Ownership
//! concentration is a risk signal about where knowledge is thin, not a measure
//! of anyone's contribution or worth. See `docs/MODEL.md`.
//!
//! # Determinism
//!
//! All outputs are sorted by stable keys, so the same input always yields the
//! same bytes. This makes fixture-log demos reproducible and testable.

pub mod cli;
pub mod git;
pub mod json;
pub mod metrics;
pub mod model;
pub mod parse;
pub mod report;
pub mod viewer;

use std::fs;
use std::path::Path;

/// High-level error type for the pipeline.
#[derive(Debug)]
pub enum Error {
    Git(git::GitError),
    Io(std::io::Error),
    NotARepo(String),
    Usage(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Git(e) => write!(f, "{e}"),
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::NotARepo(p) => write!(f, "'{p}' is not a Git repository"),
            Error::Usage(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<git::GitError> for Error {
    fn from(e: git::GitError) -> Self {
        Error::Git(e)
    }
}
impl From<std::io::Error> for Error {
