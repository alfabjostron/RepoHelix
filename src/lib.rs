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
//!   the same commit, scored with a Jaccard-style strength and a directional
//!   confidence.
//! - **Hotspots** — files that are simultaneously high-churn and
//!   high-frequency, where maintenance effort concentrates.
//! - **Ownership concentration** — how change activity for a file is spread
//!   across author identities, as a bus-factor / knowledge-distribution signal.
//! - **Temporal clusters** — bursts of commits that approximate development
//!   sessions or release pushes.
//!
//! # A note on interpretation
//!
//! These metrics describe *files and change activity*, not people. Ownership
//! concentration is a risk signal about where knowledge is thin, not a measure
//! of anyone's contribution or worth. See `docs/MODEL.md`.
//!
//! # Determinism
//!
