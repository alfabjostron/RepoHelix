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
