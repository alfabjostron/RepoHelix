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
/// parse, so a single corrupt line cannot break an otherwise valid import.
pub fn parse_log(raw: &str) -> History {
    let mut commits = Vec::new();

    // Records are separated by COMMIT_SEP. The first split segment before the
    // first separator is empty (or leading whitespace) and is ignored.
    for record in raw.split(COMMIT_SEP) {
        let record = record.trim_matches(['\n', '\r']);
        if record.is_empty() {
            continue;
        }
        if let Some(commit) = parse_record(record) {
