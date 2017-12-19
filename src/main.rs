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
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let command = cli::parse(args)?;
    match command {
        Command::Help => {
            print!("{}", cli::help_text());
            Ok(())
        }
        Command::Version => {
            println!("repohelix {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Command::Analyze(opts) => {
            let history =
                load_history(&opts.source, opts.max_commits).map_err(|e| e.to_string())?;
            let analysis = analyze(&history, &opts.params);
            let rendered = match opts.format {
                Format::Json => report::to_json(&analysis, opts.pretty),
                Format::Text => report::to_text(&analysis),
            };
            emit(opts.out.as_deref(), &rendered).map_err(|e| e.to_string())
        }
        Command::ViewerData(opts) => {
            let history =
                load_history(&opts.source, opts.max_commits).map_err(|e| e.to_string())?;
            let analysis = analyze(&history, &opts.params);
            // 64 nodes keeps the payload compact and the helix legible.
            let payload = viewer::build(&analysis, 64);
            emit(opts.out.as_deref(), &payload.to_compact()).map_err(|e| e.to_string())
        }
    }
}

fn load_history(
    source: &Source,
    max_commits: Option<usize>,
) -> Result<repohelix::model::History, Error> {
    match source {
        Source::Repo(p) => load_from_repo(p, max_commits),
        Source::Log(p) => load_from_log(p, max_commits),
    }
}
