//! Command-line entry point for `repohelix`.

use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use repohelix::cli::{self, Command, Format, Source};
use repohelix::metrics::analyze;
use repohelix::{load_from_log, load_from_repo, report, viewer, Error};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            let _ = writeln!(std::io::stderr(), "repohelix: {msg}");
            ExitCode::FAILURE
