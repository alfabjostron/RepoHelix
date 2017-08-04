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
    pub max_files_for_cochange: usize,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            min_cochange: 2,
            top_cochange: 40,
            top_hotspots: 20,
            // Six hours: commits farther apart than this start a new session.
            cluster_gap_secs: 6 * 3600,
            max_files_for_cochange: 40,
        }
    }
}

/// Run the full analysis pipeline over a history.
pub fn analyze(history: &History, params: &Params) -> Analysis {
    let file_stats = compute_file_stats(history);
    let co_changes = compute_cochange(history, params);
    let hotspots = compute_hotspots(&file_stats, params);
    let ownership = compute_ownership(history);
    let clusters = compute_clusters(history, params);
    let summary = compute_summary(history, &file_stats);
    Analysis {
        file_stats,
        co_changes,
        hotspots,
        ownership,
        clusters,
        summary,
    }
}

fn compute_file_stats(history: &History) -> Vec<FileStat> {
    struct Acc {
        commits: u64,
        added: u64,
        removed: u64,
        churn: u64,
        authors: BTreeSet<String>,
        first_seen: i64,
        last_seen: i64,
    }
    let mut map: BTreeMap<String, Acc> = BTreeMap::new();

    for c in &history.commits {
        for f in &c.files {
            let acc = map.entry(f.path.clone()).or_insert_with(|| Acc {
                commits: 0,
                added: 0,
                removed: 0,
                churn: 0,
                authors: BTreeSet::new(),
                first_seen: c.timestamp,
                last_seen: c.timestamp,
            });
            acc.commits += 1;
            acc.added += f.added.unwrap_or(0);
            acc.removed += f.removed.unwrap_or(0);
            acc.churn += f.churn();
            acc.authors.insert(c.identity().to_string());
            acc.first_seen = acc.first_seen.min(c.timestamp);
            acc.last_seen = acc.last_seen.max(c.timestamp);
        }
    }

    let mut stats: Vec<FileStat> = map
        .into_iter()
        .map(|(path, a)| FileStat {
            path,
            commits: a.commits,
            added: a.added,
            removed: a.removed,
            churn: a.churn,
            authors: a.authors.len() as u64,
            first_seen: a.first_seen,
            last_seen: a.last_seen,
        })
        .collect();

    // Sort by churn desc, then commits desc, then path for determinism.
    stats.sort_by(|x, y| {
        y.churn
            .cmp(&x.churn)
            .then(y.commits.cmp(&x.commits))
            .then(x.path.cmp(&y.path))
    });
    stats
}

fn compute_cochange(history: &History, params: &Params) -> Vec<CoChange> {
    // Count individual file appearances and unordered-pair co-appearances.
    let mut file_count: HashMap<&str, u64> = HashMap::new();
    let mut pair_count: HashMap<(&str, &str), u64> = HashMap::new();

    for c in &history.commits {
        // Distinct, sorted set of paths in this commit.
        let mut paths: Vec<&str> = c.files.iter().map(|f| f.path.as_str()).collect();
        paths.sort_unstable();
        paths.dedup();

        if paths.is_empty() {
            continue;
        }
        if params.max_files_for_cochange > 0 && paths.len() > params.max_files_for_cochange {
            // Still count singletons so confidence denominators stay correct.
            for p in &paths {
                *file_count.entry(*p).or_insert(0) += 1;
            }
            continue;
        }

        for p in &paths {
            *file_count.entry(*p).or_insert(0) += 1;
        }
        for i in 0..paths.len() {
            for j in (i + 1)..paths.len() {
                // paths already sorted, so (i, j) is canonical ordering.
                *pair_count.entry((paths[i], paths[j])).or_insert(0) += 1;
            }
        }
    }

    let mut pairs: Vec<CoChange> = pair_count
        .into_iter()
        .filter(|(_, together)| *together >= params.min_cochange)
        .map(|((a, b), together)| {
            let ca = *file_count.get(a).unwrap_or(&together);
            let cb = *file_count.get(b).unwrap_or(&together);
            let union = (ca + cb).saturating_sub(together);
            let strength = if union == 0 {
                0.0
            } else {
                together as f64 / union as f64
            };
            // Use the more frequently changed file as antecedent.
            let antecedent = ca.max(cb);
            let confidence = if antecedent == 0 {
                0.0
            } else {
                together as f64 / antecedent as f64
            };
            CoChange {
                a: a.to_string(),
                b: b.to_string(),
                together,
                count_a: ca,
                count_b: cb,
                strength,
                confidence,
            }
        })
        .collect();

    pairs.sort_by(|x, y| {
        y.strength
            .partial_cmp(&x.strength)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(y.together.cmp(&x.together))
            .then(x.a.cmp(&y.a))
            .then(x.b.cmp(&y.b))
    });
    pairs.truncate(params.top_cochange);
    pairs
}

fn compute_hotspots(file_stats: &[FileStat], params: &Params) -> Vec<Hotspot> {
    let max_churn = file_stats.iter().map(|f| f.churn).max().unwrap_or(0) as f64;
    let max_commits = file_stats.iter().map(|f| f.commits).max().unwrap_or(0) as f64;

    let mut spots: Vec<Hotspot> = file_stats
        .iter()
        .map(|f| {
            let nc = if max_churn > 0.0 {
                f.churn as f64 / max_churn
            } else {
                0.0
            };
            let nf = if max_commits > 0.0 {
                f.commits as f64 / max_commits
            } else {
                0.0
            };
            // Geometric mean rewards files that are high on *both* axes.
            let score = (nc * nf).sqrt();
            Hotspot {
                path: f.path.clone(),
                churn: f.churn,
                commits: f.commits,
                score,
            }
        })
        .collect();

    spots.sort_by(|x, y| {
        y.score
            .partial_cmp(&x.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(y.churn.cmp(&x.churn))
            .then(x.path.cmp(&y.path))
    });
    spots.truncate(params.top_hotspots);
    spots
}

fn compute_ownership(history: &History) -> Vec<Ownership> {
    // For each file: map identity -> commit count.
    let mut per_file: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    for c in &history.commits {
        for f in &c.files {
            *per_file
                .entry(f.path.clone())
                .or_default()
                .entry(c.identity().to_string())
                .or_insert(0) += 1;
        }
    }

    // Stable anonymized labels: rank global identities by total activity.
    let mut global: BTreeMap<String, u64> = BTreeMap::new();
    for counts in per_file.values() {
        for (id, n) in counts {
            *global.entry(id.clone()).or_insert(0) += *n;
        }
    }
    let mut ranked: Vec<(String, u64)> = global.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let mut label_of: HashMap<String, String> = HashMap::new();
    for (i, (id, _)) in ranked.iter().enumerate() {
        label_of.insert(id.clone(), format!("author#{}", i + 1));
    }

    let mut out: Vec<Ownership> = per_file
        .into_iter()
        .map(|(path, counts)| {
            let total: u64 = counts.values().sum();
            let mut top_id = String::new();
            let mut top_n = 0u64;
            let mut hhi = 0.0f64;
            for (id, n) in &counts {
                let share = *n as f64 / total as f64;
                hhi += share * share;
                if *n > top_n || (*n == top_n && id < &top_id) {
                    if *n > top_n {
                        top_n = *n;
                        top_id = id.clone();
                    } else if top_id.is_empty() {
                        top_id = id.clone();
                    }
                }
            }
            let authors = counts.len() as u64;
            // Normalize HHI so a single author -> 1.0 and even split -> ~0.
            let concentration = if authors <= 1 {
                1.0
            } else {
                let n = authors as f64;
                (hhi - 1.0 / n) / (1.0 - 1.0 / n)
            };
            let top_share = if total == 0 {
                0.0
            } else {
