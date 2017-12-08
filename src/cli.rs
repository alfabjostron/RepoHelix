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

/// Parse arguments (excluding the program name).
pub fn parse(args: &[String]) -> Result<Command, String> {
    let mut iter = args.iter();
    let sub = match iter.next() {
        None => return Ok(Command::Help),
        Some(s) => s.as_str(),
    };

    match sub {
        "help" | "--help" | "-h" => Ok(Command::Help),
        "version" | "--version" | "-V" => Ok(Command::Version),
        "analyze" => parse_analyze(args[1..].to_vec()),
        "viewer-data" => parse_viewer(args[1..].to_vec()),
        other => Err(format!("unknown command '{other}'. Try 'repohelix help'.")),
    }
}

struct Cursor {
    args: Vec<String>,
    idx: usize,
}

impl Cursor {
    fn new(args: Vec<String>) -> Self {
        Cursor { args, idx: 0 }
    }
    fn next(&mut self) -> Option<String> {
        let v = self.args.get(self.idx).cloned();
        if v.is_some() {
            self.idx += 1;
        }
        v
    }
    fn value(&mut self, flag: &str) -> Result<String, String> {
        self.next()
            .ok_or_else(|| format!("flag '{flag}' requires a value"))
    }
}

fn parse_source(repo: Option<PathBuf>, log: Option<PathBuf>) -> Result<Source, String> {
    match (repo, log) {
        (Some(_), Some(_)) => Err("choose only one of --repo or --log".to_string()),
        (Some(r), None) => Ok(Source::Repo(r)),
        (None, Some(l)) => Ok(Source::Log(l)),
        // Default: analyze the current directory as a repository.
        (None, None) => Ok(Source::Repo(PathBuf::from("."))),
    }
}

fn parse_analyze(args: Vec<String>) -> Result<Command, String> {
    let mut c = Cursor::new(args);
    let mut repo = None;
    let mut log = None;
    let mut format = Format::Text;
    let mut pretty = true;
    let mut max_commits = None;
    let mut out = None;
    let mut params = Params::default();

    while let Some(a) = c.next() {
        match a.as_str() {
            "--repo" => repo = Some(PathBuf::from(c.value("--repo")?)),
            "--log" => log = Some(PathBuf::from(c.value("--log")?)),
            "--format" => {
                format = match c.value("--format")?.as_str() {
                    "json" => Format::Json,
                    "text" => Format::Text,
                    other => return Err(format!("invalid --format '{other}' (json|text)")),
                }
            }
            "--compact" => pretty = false,
            "--pretty" => pretty = true,
            "--max-commits" => {
                max_commits = Some(parse_usize(&c.value("--max-commits")?, "--max-commits")?)
            }
            "--out" => out = Some(PathBuf::from(c.value("--out")?)),
            "--min-cochange" => {
                params.min_cochange = parse_u64(&c.value("--min-cochange")?, "--min-cochange")?
            }
            "--top-cochange" => {
                params.top_cochange = parse_usize(&c.value("--top-cochange")?, "--top-cochange")?
            }
            "--top-hotspots" => {
                params.top_hotspots = parse_usize(&c.value("--top-hotspots")?, "--top-hotspots")?
            }
            "--cluster-gap" => {
                params.cluster_gap_secs = parse_i64(&c.value("--cluster-gap")?, "--cluster-gap")?
            }
            "--max-files-cochange" => {
                params.max_files_for_cochange =
                    parse_usize(&c.value("--max-files-cochange")?, "--max-files-cochange")?
            }
            other => return Err(format!("unknown flag '{other}' for analyze")),
        }
    }

    let source = parse_source(repo, log)?;
    Ok(Command::Analyze(AnalyzeOpts {
        source,
        format,
        pretty,
        max_commits,
        out,
        params,
    }))
}

fn parse_viewer(args: Vec<String>) -> Result<Command, String> {
    let mut c = Cursor::new(args);
    let mut repo = None;
    let mut log = None;
    let mut out = None;
    let mut max_commits = None;
    let mut params = Params::default();

    while let Some(a) = c.next() {
        match a.as_str() {
            "--repo" => repo = Some(PathBuf::from(c.value("--repo")?)),
            "--log" => log = Some(PathBuf::from(c.value("--log")?)),
            "--out" => out = Some(PathBuf::from(c.value("--out")?)),
            "--max-commits" => {
                max_commits = Some(parse_usize(&c.value("--max-commits")?, "--max-commits")?)
            }
            "--top-cochange" => {
                params.top_cochange = parse_usize(&c.value("--top-cochange")?, "--top-cochange")?
            }
            "--min-cochange" => {
                params.min_cochange = parse_u64(&c.value("--min-cochange")?, "--min-cochange")?
            }
            other => return Err(format!("unknown flag '{other}' for viewer-data")),
        }
    }

    let source = parse_source(repo, log)?;
    Ok(Command::ViewerData(ViewerOpts {
        source,
        out,
        max_commits,
        params,
    }))
}

fn parse_usize(s: &str, flag: &str) -> Result<usize, String> {
    s.parse::<usize>()
        .map_err(|_| format!("flag '{flag}' expects a non-negative integer, got '{s}'"))
}

fn parse_u64(s: &str, flag: &str) -> Result<u64, String> {
    s.parse::<u64>()
        .map_err(|_| format!("flag '{flag}' expects a non-negative integer, got '{s}'"))
}

fn parse_i64(s: &str, flag: &str) -> Result<i64, String> {
    s.parse::<i64>()
        .map_err(|_| format!("flag '{flag}' expects an integer, got '{s}'"))
}

/// The help text shown for `repohelix help`.
pub fn help_text() -> &'static str {
    "repohelix — Git history architecture & temporal-coupling explorer\n\
     \n\
     USAGE:\n\
     \x20 repohelix <command> [options]\n\
     \n\
     COMMANDS:\n\
     \x20 analyze        Analyze history and print a JSON or text report\n\
     \x20 viewer-data    Emit compact JSON for the TypeScript helix viewer\n\
     \x20 help           Show this help\n\
     \x20 version        Show version\n\
     \n\
     SOURCE (pick one; defaults to --repo .):\n\
     \x20 --repo <dir>            Analyze a live Git repository\n\
     \x20 --log <file>           Import a captured/fixture git log file\n\
     \n\
     ANALYZE OPTIONS:\n\
     \x20 --format <json|text>    Output format (default: text)\n\
     \x20 --compact               Compact JSON (default: pretty)\n\
     \x20 --max-commits <n>       Limit history depth\n\
     \x20 --out <file>            Write to file instead of stdout\n\
     \x20 --min-cochange <n>      Minimum co-change count (default: 2)\n\
     \x20 --top-cochange <n>      Max co-change pairs (default: 40)\n\
     \x20 --top-hotspots <n>      Max hotspots (default: 20)\n\
     \x20 --cluster-gap <secs>    Session gap threshold (default: 21600)\n\
     \x20 --max-files-cochange <n> Skip huge commits for coupling (default: 40)\n\
     \n\
     Metrics describe code and change activity only. They are not measures of\n\
     individual productivity and must not be used as personnel judgments.\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn defaults_to_help_when_empty() {
        assert_eq!(parse(&[]).unwrap(), Command::Help);
    }

    #[test]
    fn parses_analyze_with_log() {
        let cmd = parse(&s(&["analyze", "--log", "fixture.log", "--format", "json"])).unwrap();
        match cmd {
            Command::Analyze(o) => {
                assert_eq!(o.source, Source::Log(PathBuf::from("fixture.log")));
                assert_eq!(o.format, Format::Json);
                assert!(o.pretty);
            }
            _ => panic!("expected analyze"),
        }
    }

    #[test]
    fn compact_flag_disables_pretty() {
        let cmd = parse(&s(&["analyze", "--compact"])).unwrap();
        match cmd {
            Command::Analyze(o) => assert!(!o.pretty),
            _ => panic!(),
        }
    }

    #[test]
