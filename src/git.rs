//! Non-interactive invocation of the local `git` binary.
//!
//! `repohelix` never opens an editor, pager, or prompt. All git calls set
//! environment variables and flags that guarantee batch behavior, so the tool
//! is safe to run inside scripts and CI.

use std::io;
use std::path::Path;
use std::process::Command;

