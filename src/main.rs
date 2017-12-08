//! Command-line entry point for `repohelix`.

use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use repohelix::cli::{self, Command, Format, Source};
use repohelix::metrics::analyze;
