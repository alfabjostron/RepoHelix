//! Analytical metrics derived from a [`History`].
//!
//! Every metric here is a descriptive summary of *file* and *change* activity.
//! None of these numbers is a judgment about people. Ownership concentration,
//! for example, measures how change activity for a file is distributed across
//! author identities — it describes knowledge-distribution risk (bus factor),
//! not individual performance. See `docs/MODEL.md` for the full rationale.

use crate::model::{Commit, History};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Per-file churn and touch statistics.
#[derive(Debug, Clone, PartialEq)]
pub struct FileStat {
    pub path: String,
    /// Number of commits that touched this file.
    pub commits: u64,
    /// Total lines added across history.
    pub added: u64,
    /// Total lines removed across history.
    pub removed: u64,
    /// Total churn (added + removed, binaries count as 1 each touch).
    pub churn: u64,
    /// Distinct author identities that touched this file.
    pub authors: u64,
    /// First (oldest) timestamp the file was touched.
    pub first_seen: i64,
    /// Last (newest) timestamp the file was touched.
    pub last_seen: i64,
}

/// A co-change relationship between two files.
#[derive(Debug, Clone, PartialEq)]
pub struct CoChange {
    pub a: String,
    pub b: String,
    /// Commits in which both files changed together.
    pub together: u64,
    /// Commits touching `a`.
    pub count_a: u64,
    /// Commits touching `b`.
    pub count_b: u64,
    /// Jaccard-like coupling strength in [0, 1]:
    /// together / (count_a + count_b - together).
    pub strength: f64,
    /// Confidence: P(b changes | a changes) = together / count_a, using the
    /// more frequently changed file as the antecedent for stability.
    pub confidence: f64,
}

/// A hotspot combines high churn with high change frequency. Files that are
/// both large-churn and frequently touched tend to concentrate maintenance
/// effort and risk.
#[derive(Debug, Clone, PartialEq)]
pub struct Hotspot {
    pub path: String,
    pub churn: u64,
    pub commits: u64,
    /// Normalized score in [0, 1]: geometric blend of normalized churn and
    /// normalized commit frequency.
    pub score: f64,
}

/// Ownership distribution for a single file.
#[derive(Debug, Clone, PartialEq)]
pub struct Ownership {
    pub path: String,
    /// Distinct author identities.
    pub authors: u64,
    /// Fraction of commits from the single most active identity (0..1).
    pub top_share: f64,
    /// Normalized Herfindahl–Hirschman Index of commit shares in [0, 1].
    /// 1.0 means a single identity made every change (maximal concentration,
    /// i.e. a bus-factor of one). Lower values mean shared stewardship.
    pub concentration: f64,
    /// Anonymized label for the most active identity (e.g. "author#1").
    pub top_identity_label: String,
}

/// A temporal cluster is a burst of commits separated from neighbors by a gap
/// larger than the chosen threshold. Clusters approximate development sessions
/// or release pushes.
#[derive(Debug, Clone, PartialEq)]
pub struct TemporalCluster {
