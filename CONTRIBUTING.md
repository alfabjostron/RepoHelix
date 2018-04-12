# Contributing to RepoHelix

RepoHelix is deliberately local-first: parse git logs, build co-change
atlases and churn windows, emit deterministic reports. No network, no
dependencies, no hidden state. Contributions that respect that scope are
welcome.

## Development setup

```bash
git clone https://github.com/alfabjostron/RepoHelix.git
cd RepoHelix
cargo build
cargo test --all
```

The viewer lives in `viewer/` (`npm install && npm run build && npm test`).
CI regenerates the fixture demo and fails on output drift.

## Ground rules

- **Deterministic reports.** The same gitlog and flags must always
  produce the same report bytes. No wall-clock in output, no
  map-iteration order.
- **Std-only core.** The parser, metrics, and JSON codec stay on the Rust
  standard library; a new dependency needs a very good reason.
- **Preserve file identity.** Rename tracking is the backbone of every
  metric; changes there need fixtures covering both rename cases and
  edge-pruning behavior.
- **Tests on behaviour changes.** `tests/integration.rs` is the
  contract; threshold and pruning changes need boundary coverage.

## Commit style

Short imperative subjects (`feat: ...`, `fix: ...`, `docs: ...`). Body only
when the "why" is not obvious from the diff.

## Reporting issues

Attach a sanitized gitlog (or describe the repository shape), the exact
flags used, and the report section that looks wrong.