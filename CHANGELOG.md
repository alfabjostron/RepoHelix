# Changelog

All notable changes to `repohelix` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2024-01-21

The first release. `repohelix` reads Git history and draws two maps of it — a
helix and a co-change atlas — with a standard-library-only Rust core and a
dependency-free TypeScript viewer.

### Added

- **CLI** with `analyze`, `viewer-data`, `help`, and `version` commands.
- **Live repository analysis** via non-interactive `git log --numstat`
  (`--no-pager`, `--no-color`, `--no-renames`, and batch-forcing environment
  variables).
- **Fixture-log import** (`--log`) that reuses the live parser for byte-for-byte
  reproducible demos and CI.
- **Metrics**: per-file churn, hotspots (geometric mean of normalized churn and
  frequency), temporal coupling (Jaccard strength + association confidence),
  temporal clusters (session detection by time gap), and ownership concentration
  (normalized Herfindahl index, anonymized identities).
- **Reports**: pretty/compact JSON (`schema: repohelix/analysis/v1`) and a text
  table report, both carrying the "not a personnel judgment" reminder.
- **Viewer payload** builder (`schema: repohelix/viewer/v1`), capped at 64 nodes.
- **Minimal std-only JSON writer** with correct escaping, stable key order, and
  finite-number safety.
- **TypeScript viewer**: DOM/SVG-only helix and co-change atlas with hover
  tooltips, split into a pure `core` module and an SVG-rendering module.
- **Synthetic fixture** `fixtures/nebula.gitlog` modeling four development
  sessions, plus generator scripts for the fixture and the viewer dataset.
- **Documentation**: a 15–22 KB README with the DNA/cartography identity, two
  local animated SVGs, and `docs/MODEL.md` defining every formula and the ethics
  of interpretation.
- **Tests**: 41 Rust unit + integration tests and a zero-framework TypeScript
  core test suite.
- **Tooling**: `Makefile`, GitHub Actions CI, MIT `LICENSE`.

### Known limitations

- Renames are not followed; a rename appears as a delete + add.
- The viewer renders the 64 busiest files; the full data is in the JSON report.
