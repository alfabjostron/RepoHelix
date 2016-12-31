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
            commits.push(commit);
        }
    }

    History::new(commits)
}

fn parse_record(record: &str) -> Option<Commit> {
    let mut lines = record.lines();
    let header = lines.next()?;

    let mut fields = header.split(FIELD_SEP);
    let hash = fields.next()?.trim().to_string();
    let author_name = fields.next().unwrap_or("").trim().to_string();
    let author_email = fields.next().unwrap_or("").trim().to_string();
    let timestamp = fields
        .next()
        .and_then(|s| s.trim().parse::<i64>().ok())
        .unwrap_or(0);
    // Subject may itself contain the field separator only if authored oddly;
    // rejoin the remainder to be safe.
    let subject_parts: Vec<&str> = fields.collect();
    let subject = subject_parts.join(&FIELD_SEP.to_string());

    if hash.is_empty() {
        return None;
    }

    let mut files = Vec::new();
    for line in lines {
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() {
            continue;
        }
        if let Some(fc) = parse_numstat_line(line) {
            files.push(fc);
        }
    }

    Some(Commit {
        hash,
        author_name,
        author_email,
        timestamp,
        subject: subject.trim().to_string(),
        files,
    })
}

/// Parse a single numstat line: `<added>\t<removed>\t<path>`.
/// Binary files use `-` for the counts.
fn parse_numstat_line(line: &str) -> Option<FileChange> {
    let mut parts = line.splitn(3, '\t');
    let added_s = parts.next()?.trim();
    let removed_s = parts.next()?.trim();
    let path = parts.next()?.trim();

    if path.is_empty() {
        return None;
    }

    let added = if added_s == "-" {
        None
    } else {
        added_s.parse::<u64>().ok()
    };
    let removed = if removed_s == "-" {
        None
    } else {
        removed_s.parse::<u64>().ok()
    };

    Some(FileChange {
        path: path.to_string(),
        added,
        removed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_log() -> String {
        // Two commits: newest first, as git emits.
        format!(
            "{sep}abc123{fs}Ada Lovelace{fs}ada@example.org{fs}1700000200{fs}Refactor engine\n\
             10\t2\tsrc/engine.rs\n\
             3\t0\tsrc/lib.rs\n\
             {sep}def456{fs}Grace Hopper{fs}grace@example.org{fs}1700000100{fs}Initial commit\n\
             40\t0\tsrc/engine.rs\n\
             -\t-\tassets/logo.png\n",
            sep = COMMIT_SEP,
            fs = "\x1f"
        )
    }

    #[test]
    fn parses_two_commits() {
        let h = parse_log(&sample_log());
        assert_eq!(h.commit_count(), 2);
        let first = &h.commits[0];
        assert_eq!(first.hash, "abc123");
        assert_eq!(first.author_name, "Ada Lovelace");
        assert_eq!(first.author_email, "ada@example.org");
        assert_eq!(first.timestamp, 1700000200);
        assert_eq!(first.subject, "Refactor engine");
        assert_eq!(first.files.len(), 2);
        assert_eq!(first.files[0].added, Some(10));
        assert_eq!(first.files[0].removed, Some(2));
    }

    #[test]
    fn handles_binary_files() {
