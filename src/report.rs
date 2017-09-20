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
