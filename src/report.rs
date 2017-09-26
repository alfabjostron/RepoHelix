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
