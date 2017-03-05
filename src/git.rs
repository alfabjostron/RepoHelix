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
                let code = code
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "signal".into());
                write!(f, "git exited with status {code}: {}", stderr.trim())
            }
            GitError::Encoding => write!(f, "git produced non-UTF-8 output"),
        }
    }
}

impl std::error::Error for GitError {}

/// The custom pretty format we request from `git log`. Fields, in order:
/// hash, author name, author email, author unix time, subject.
fn pretty_format() -> String {
    // %x1e is the record separator; %x1f delimits fields on the header line.
    // The trailing separator lets us split cleanly before numstat lines.
    format!(
        "{sep}%H{fs}%an{fs}%ae{fs}%at{fs}%s",
        sep = COMMIT_SEP,
        fs = "\x1f"
    )
}

/// Run `git log` in the given repository and return raw stdout.
///
/// `max_commits` optionally caps history depth. The command is fully
/// non-interactive: no pager, no color, no prompts.
pub fn run_log(repo: &Path, max_commits: Option<usize>) -> Result<String, GitError> {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(repo)
        // Disable any pager unconditionally.
        .arg("--no-pager")
        .arg("log")
        .arg("--no-color")
        .arg("--no-renames")
        .arg("--numstat")
        .arg(format!("--pretty=format:{}", pretty_format()));

    if let Some(n) = max_commits {
        cmd.arg(format!("-n{}", n));
    }

    // Force batch behavior regardless of user git config.
    cmd.env("GIT_PAGER", "cat");
    cmd.env("GIT_TERMINAL_PROMPT", "0");
    cmd.env("GIT_OPTIONAL_LOCKS", "0");
    cmd.env("GIT_CONFIG_NOSYSTEM", "1");
    cmd.env("LC_ALL", "C");

    let output = cmd.output().map_err(GitError::Spawn)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(GitError::Failed {
            code: output.status.code(),
            stderr,
