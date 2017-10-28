//! Hand-rolled command-line parsing (no external argument crate).
//!
//! Usage patterns supported:
//!   repohelix analyze [--repo <dir>] [--log <file>] [options]
//!   repohelix viewer-data [--repo <dir>] [--log <file>] [--out <file>]
//!   repohelix help | --help | -h
//!   repohelix version | --version | -V

use crate::metrics::Params;
use std::path::PathBuf;

/// The chosen subcommand and its parsed options.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Emit a JSON or text analysis report.
    Analyze(AnalyzeOpts),
    /// Emit the compact JSON consumed by the TypeScript viewer.
    ViewerData(ViewerOpts),
    Help,
    Version,
}

/// Where history comes from: a live repository or a captured fixture log.
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    Repo(PathBuf),
    Log(PathBuf),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyzeOpts {
    pub source: Source,
    pub format: Format,
