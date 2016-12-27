//! Parse the raw `git log --numstat` output produced by [`crate::git`].
//!
//! The format is a stream of records, each beginning with `COMMIT_SEP`,
//! followed by a header line of unit-separator-delimited fields, then zero or
//! more numstat lines of the form `<added>\t<removed>\t<path>`.
//!
//! The same parser handles the deterministic fixture-log format, which is
//! simply captured `git log` output stored in a file. This lets demos run
//! without needing a live repository.

use crate::git::{COMMIT_SEP, FIELD_SEP};
use crate::model::{Commit, FileChange, History};

/// Parse a raw log string into a [`History`].
///
/// Malformed records are skipped defensively rather than aborting the whole
