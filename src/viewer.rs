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
