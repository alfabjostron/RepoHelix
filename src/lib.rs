//! `repohelix` — a Git history architecture and temporal-coupling explorer.
//!
//! # What it does
//!
//! `repohelix` reads Git commit history (either from a live repository via the
//! local `git` binary, or from a captured *fixture log* file) and derives a set
//! of descriptive metrics about how the codebase evolves:
//!
//! - **Churn** — how much each file changes (lines added + removed).
//! - **Co-change / temporal coupling** — pairs of files that tend to change in
