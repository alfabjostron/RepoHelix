//! Rendering of an [`Analysis`] into JSON and human-readable text reports.

use crate::json::Json;
use crate::metrics::Analysis;
use std::fmt::Write as _;

/// Render the analysis as a JSON value tree, then to a pretty string.
pub fn to_json(analysis: &Analysis, pretty: bool) -> String {
    let root = build_json(analysis);
    if pretty {
        root.to_pretty()
    } else {
        root.to_compact()
    }
}

fn build_json(a: &Analysis) -> Json {
    let summary = Json::obj(vec![
        ("commits", Json::Int(a.summary.commits as i64)),
        ("files", Json::Int(a.summary.files as i64)),
        ("authors", Json::Int(a.summary.authors as i64)),
        ("total_churn", Json::Int(a.summary.total_churn as i64)),
        ("first_commit", Json::Int(a.summary.first_commit)),
        ("last_commit", Json::Int(a.summary.last_commit)),
        ("span_days", Json::Int(a.summary.span_days)),
    ]);

    let files = Json::Array(
        a.file_stats
            .iter()
            .map(|f| {
                Json::obj(vec![
                    ("path", Json::str(&f.path)),
                    ("commits", Json::Int(f.commits as i64)),
                    ("added", Json::Int(f.added as i64)),
                    ("removed", Json::Int(f.removed as i64)),
                    ("churn", Json::Int(f.churn as i64)),
                    ("authors", Json::Int(f.authors as i64)),
                    ("first_seen", Json::Int(f.first_seen)),
                    ("last_seen", Json::Int(f.last_seen)),
                ])
            })
            .collect(),
    );

    let cochanges = Json::Array(
        a.co_changes
            .iter()
            .map(|c| {
                Json::obj(vec![
                    ("a", Json::str(&c.a)),
                    ("b", Json::str(&c.b)),
                    ("together", Json::Int(c.together as i64)),
                    ("count_a", Json::Int(c.count_a as i64)),
                    ("count_b", Json::Int(c.count_b as i64)),
                    ("strength", Json::Float(c.strength)),
                    ("confidence", Json::Float(c.confidence)),
                ])
            })
            .collect(),
    );

    let hotspots = Json::Array(
        a.hotspots
            .iter()
            .map(|h| {
                Json::obj(vec![
                    ("path", Json::str(&h.path)),
                    ("churn", Json::Int(h.churn as i64)),
                    ("commits", Json::Int(h.commits as i64)),
                    ("score", Json::Float(h.score)),
                ])
            })
            .collect(),
    );

    let ownership = Json::Array(
        a.ownership
            .iter()
            .map(|o| {
                Json::obj(vec![
                    ("path", Json::str(&o.path)),
                    ("authors", Json::Int(o.authors as i64)),
                    ("top_share", Json::Float(o.top_share)),
                    ("concentration", Json::Float(o.concentration)),
                    ("top_identity_label", Json::str(&o.top_identity_label)),
                ])
            })
            .collect(),
    );

    let clusters = Json::Array(
        a.clusters
            .iter()
            .map(|c| {
                Json::obj(vec![
                    ("index", Json::Int(c.index as i64)),
                    ("start", Json::Int(c.start)),
                    ("end", Json::Int(c.end)),
                    ("commits", Json::Int(c.commits as i64)),
                    ("files_touched", Json::Int(c.files_touched as i64)),
                    ("churn", Json::Int(c.churn as i64)),
                ])
            })
            .collect(),
    );

    Json::obj(vec![
        ("schema", Json::str("repohelix/analysis/v1")),
        (
            "disclaimer",
            Json::str(
                "Metrics describe file and change activity only. They are not \
                 measures of individual productivity or personnel judgments.",
            ),
        ),
        ("summary", summary),
        ("files", files),
        ("cochanges", cochanges),
        ("hotspots", hotspots),
        ("ownership", ownership),
        ("clusters", clusters),
    ])
}

/// Render a compact, human-readable text report.
pub fn to_text(analysis: &Analysis) -> String {
    let mut out = String::new();
    let s = &analysis.summary;

    let _ = writeln!(out, "repohelix — Git history analysis");
    let _ = writeln!(out, "================================");
    let _ = writeln!(out);
    let _ = writeln!(out, "Summary");
    let _ = writeln!(out, "  commits ......... {}", s.commits);
    let _ = writeln!(out, "  files ........... {}", s.files);
    let _ = writeln!(out, "  authors ......... {}", s.authors);
    let _ = writeln!(out, "  total churn ..... {}", s.total_churn);
    let _ = writeln!(out, "  span (days) ..... {}", s.span_days);
    let _ = writeln!(out);

    let _ = writeln!(out, "Top hotspots (churn x frequency)");
    let _ = writeln!(
        out,
        "  {:<40} {:>7} {:>7} {:>7}",
        "path", "churn", "commits", "score"
    );
    for h in analysis.hotspots.iter().take(15) {
        let _ = writeln!(
            out,
            "  {:<40} {:>7} {:>7} {:>7.3}",
            truncate(&h.path, 40),
            h.churn,
            h.commits,
            h.score
        );
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "Strongest temporal coupling (co-change)");
    let _ = writeln!(
        out,
        "  {:>5} {:>5} {:<26} {:<26}",
        "str", "conf", "file A", "file B"
    );
    for c in analysis.co_changes.iter().take(15) {
        let _ = writeln!(
            out,
            "  {:>5.2} {:>5.2} {:<26} {:<26}",
            c.strength,
            c.confidence,
            truncate(&c.a, 26),
            truncate(&c.b, 26)
        );
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "Ownership concentration (bus-factor risk)");
    let _ = writeln!(
        out,
        "  {:<40} {:>7} {:>7} {:>10}",
        "path", "authors", "top%", "concentr."
    );
    for o in analysis.ownership.iter().take(15) {
        let _ = writeln!(
            out,
            "  {:<40} {:>7} {:>6.0}% {:>10.3}",
            truncate(&o.path, 40),
            o.authors,
            o.top_share * 100.0,
            o.concentration
        );
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "Temporal clusters (development sessions)");
    let _ = writeln!(
        out,
        "  {:>5} {:>8} {:>7} {:>7}",
        "idx", "commits", "files", "churn"
    );
    for c in analysis.clusters.iter().take(15) {
        let _ = writeln!(
            out,
            "  {:>5} {:>8} {:>7} {:>7}",
            c.index, c.commits, c.files_touched, c.churn
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "Note: these metrics describe code and change activity, not people."
    );

    out
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let keep = max.saturating_sub(1);
        let head: String = s
            .chars()
            .rev()
            .take(keep)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        format!("…{head}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{analyze, Params};
    use crate::model::{Commit, FileChange, History};

    fn hist() -> History {
        History::new(vec![Commit {
            hash: "h1".into(),
            author_name: "Ada".into(),
            author_email: "ada@x".into(),
            timestamp: 1000,
            subject: "s".into(),
            files: vec![FileChange {
                path: "a.rs".into(),
                added: Some(3),
                removed: Some(1),
            }],
        }])
    }

    #[test]
    fn json_contains_schema_and_disclaimer() {
        let a = analyze(&hist(), &Params::default());
        let j = to_json(&a, false);
        assert!(j.contains("repohelix/analysis/v1"));
        assert!(j.contains("not"));
        assert!(j.contains("\"summary\""));
    }

    #[test]
    fn json_is_valid_roundtrippable_structure() {
        let a = analyze(&hist(), &Params::default());
        let j = to_json(&a, true);
        // Pretty output should be balanced and contain nested arrays.
        assert_eq!(j.matches('{').count(), j.matches('}').count());
        assert_eq!(j.matches('[').count(), j.matches(']').count());
    }

    #[test]
    fn text_report_has_sections() {
        let a = analyze(&hist(), &Params::default());
        let t = to_text(&a);
        assert!(t.contains("Summary"));
        assert!(t.contains("Top hotspots"));
        assert!(t.contains("Ownership concentration"));
        assert!(t.contains("not people"));
    }

    #[test]
    fn truncate_short_and_long() {
        assert_eq!(truncate("abc", 5), "abc");
        assert_eq!(truncate("abcdefgh", 4), "…fgh");
    }
}
