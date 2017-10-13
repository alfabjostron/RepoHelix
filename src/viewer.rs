//! Export a compact graph payload for the TypeScript helix/map viewer.
//!
//! The viewer needs a small, self-contained JSON document: file nodes with
//! churn/commit weight, co-change links with strength, and temporal clusters
//! for the helix strand animation. We cap node count so the payload stays tiny
//! and the browser render stays smooth without any dependencies.

use crate::json::Json;
use crate::metrics::Analysis;
use std::collections::BTreeMap;

/// Build the viewer payload from an analysis, limiting to the most active
/// `max_nodes` files so the visualization stays legible.
pub fn build(analysis: &Analysis, max_nodes: usize) -> Json {
    // Rank files by churn (file_stats is already churn-sorted).
    let selected: Vec<&str> = analysis
        .file_stats
        .iter()
        .take(max_nodes)
        .map(|f| f.path.as_str())
        .collect();

    // Assign stable indices to selected files.
    let mut index_of: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, p) in selected.iter().enumerate() {
        index_of.insert(*p, i);
    }

    let nodes = Json::Array(
        analysis
            .file_stats
            .iter()
            .take(max_nodes)
            .map(|f| {
                let own = analysis
                    .ownership
                    .iter()
                    .find(|o| o.path == f.path)
                    .map(|o| o.concentration)
                    .unwrap_or(0.0);
                Json::obj(vec![
                    ("path", Json::str(&f.path)),
                    ("churn", Json::Int(f.churn as i64)),
                    ("commits", Json::Int(f.commits as i64)),
                    ("authors", Json::Int(f.authors as i64)),
                    ("concentration", Json::Float(own)),
                    ("first_seen", Json::Int(f.first_seen)),
                    ("last_seen", Json::Int(f.last_seen)),
                ])
            })
            .collect(),
    );

    // Only keep links where both endpoints are among the selected nodes.
    let links = Json::Array(
        analysis
            .co_changes
            .iter()
            .filter_map(|c| {
                let a = *index_of.get(c.a.as_str())?;
                let b = *index_of.get(c.b.as_str())?;
                Some(Json::obj(vec![
                    ("source", Json::Int(a as i64)),
                    ("target", Json::Int(b as i64)),
                    ("strength", Json::Float(c.strength)),
                    ("together", Json::Int(c.together as i64)),
                ]))
            })
            .collect(),
    );

    let clusters = Json::Array(
        analysis
            .clusters
            .iter()
            .map(|c| {
                Json::obj(vec![
                    ("index", Json::Int(c.index as i64)),
                    ("start", Json::Int(c.start)),
                    ("end", Json::Int(c.end)),
                    ("commits", Json::Int(c.commits as i64)),
                    ("churn", Json::Int(c.churn as i64)),
                ])
            })
            .collect(),
    );

    Json::obj(vec![
        ("schema", Json::str("repohelix/viewer/v1")),
        ("commits", Json::Int(analysis.summary.commits as i64)),
        ("span_days", Json::Int(analysis.summary.span_days)),
        ("nodes", nodes),
        ("links", links),
