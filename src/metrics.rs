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
    pub index: usize,
    pub start: i64,
    pub end: i64,
    pub commits: u64,
    /// Distinct files touched during the cluster.
    pub files_touched: u64,
    /// Total churn during the cluster.
    pub churn: u64,
}

/// The complete analysis bundle.
#[derive(Debug, Clone)]
pub struct Analysis {
    pub file_stats: Vec<FileStat>,
    pub co_changes: Vec<CoChange>,
    pub hotspots: Vec<Hotspot>,
    pub ownership: Vec<Ownership>,
    pub clusters: Vec<TemporalCluster>,
    pub summary: Summary,
}

/// Repository-wide totals.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub commits: u64,
    pub files: u64,
    pub authors: u64,
    pub total_churn: u64,
    pub first_commit: i64,
    pub last_commit: i64,
    /// Span in whole days between first and last commit.
    pub span_days: i64,
}

/// Tunable parameters for the analysis.
#[derive(Debug, Clone)]
pub struct Params {
    /// Minimum co-occurrence count for a co-change pair to be reported.
    pub min_cochange: u64,
    /// Maximum number of co-change pairs to report.
    pub top_cochange: usize,
    /// Maximum number of hotspots to report.
    pub top_hotspots: usize,
    /// Gap (in seconds) between commits that starts a new temporal cluster.
    pub cluster_gap_secs: i64,
    /// Ignore commits that touch more than this many files (likely bulk
    /// imports or vendored drops) when computing co-change, to avoid spurious
    /// coupling. Zero disables the filter.
