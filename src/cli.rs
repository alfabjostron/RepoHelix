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
    pub pretty: bool,
    pub max_commits: Option<usize>,
    pub out: Option<PathBuf>,
    pub params: Params,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ViewerOpts {
    pub source: Source,
    pub out: Option<PathBuf>,
    pub max_commits: Option<usize>,
    pub params: Params,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Json,
    Text,
}

// Params derives PartialEq is not available; implement structural equality via
// fields we care about in tests through the option wrappers above. To keep
// Command comparable in tests, provide PartialEq for Params here.
impl PartialEq for Params {
    fn eq(&self, other: &Self) -> bool {
        self.min_cochange == other.min_cochange
            && self.top_cochange == other.top_cochange
            && self.top_hotspots == other.top_hotspots
            && self.cluster_gap_secs == other.cluster_gap_secs
            && self.max_files_for_cochange == other.max_files_for_cochange
    }
}
