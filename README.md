# repohelix

> **Read a repository like a strand of DNA. Chart it like a coastline.**
>
> `repohelix` transcribes your Git history into two living maps — a **helix** of
> files threaded through time, and a **co-change atlas** of the invisible bonds
> between them. It is a Rust standard-library CLI (zero crates) paired with a
> dependency-free TypeScript viewer. It reads real history, imports deterministic
> fixture logs for demos, and emits JSON or text you can pipe anywhere.

<p align="center">
  <img src="docs/assets/repo-helix.svg" alt="repohelix helix strand of the nebula fixture" width="640" />
</p>

<p align="center">
  <img src="docs/assets/cochange-atlas.svg" alt="repohelix co-change atlas of the nebula fixture" width="640" />
</p>

<p align="center">
  <em>Both images above are local, animated SVGs rendered from the bundled
  <code>nebula</code> fixture. No remote media, no tracking, no build step to see them.</em>
</p>

---

## The metaphor, and why it matters

Most history tools hand you a flat list of commits. `repohelix` treats history as
what it actually is: a **genome**. Every commit is a transcription event. Every
file is a locus that mutates at its own tempo. Some loci mutate together, bound
by an invisible regulatory logic. Some are edited by a single hand for years;
others are touched by the whole lab.

Two complementary readings fall out of that lens:

- **DNA (the helix).** Files threaded along a double strand in the order they
  first appeared. Radius encodes *churn* — how much text has flowed through the
  locus. Colour encodes *ownership concentration* — teal for shared stewardship,
  amber for single-owner (bus-factor) risk. Busy files pulse faster.
- **Cartography (the atlas).** The same files placed on a ring, with chords
  drawn between those that mutate together. A heavy, straight chord is a tight
  **temporal coupling** — two files that change in lockstep even if they live in
  different directories. This is the map that reveals hidden dependencies and
  missing abstractions.

The two views answer different questions. The helix asks *"where does effort and
risk accumulate over time?"* The atlas asks *"what is secretly wired to what?"*

> **A promise up front:** every number `repohelix` produces describes *code and
> change activity* — never people. Ownership concentration is a knowledge-risk
> signal (the classic *bus factor*), not a performance score. See
> [Metrics are not about people](#metrics-are-not-about-people) and
> [`docs/MODEL.md`](docs/MODEL.md).

---

## Table of contents

- [Feature tour](#feature-tour)
- [Install and build](#install-and-build)
- [Quick start (30 seconds)](#quick-start-30-seconds)
- [The command line](#the-command-line)
- [What it computes](#what-it-computes)
- [Output formats](#output-formats)
- [The interactive viewer](#the-interactive-viewer)
- [The fixture format (deterministic demos)](#the-fixture-format-deterministic-demos)
- [Architecture](#architecture)
- [Metrics are not about people](#metrics-are-not-about-people)
- [Testing](#testing)
- [Design constraints](#design-constraints)
- [FAQ](#faq)
- [License](#license)

---

## Feature tour

| Capability                    | What you get                                                                 |
| ----------------------------- | ---------------------------------------------------------------------------- |
| **Live repository analysis**  | Runs the local `git` binary non-interactively (no pager, no prompts, no color). |
| **Fixture-log import**        | Feed a captured/synthetic log for reproducible demos and CI — no repo needed. |
| **Churn**                     | Per-file added / removed / total, with binary edits handled sanely.          |
| **Hotspots**                  | Files high on *both* churn and frequency, scored by geometric mean.          |
| **Temporal coupling**         | Co-change pairs with Jaccard *strength* and association *confidence*.         |
| **Temporal clusters**         | Commit bursts grouped into development sessions by a tunable time gap.        |
| **Ownership concentration**   | Normalized Herfindahl index per file — a bus-factor signal, anonymized.      |
| **JSON + text reports**       | Machine-readable (`schema: repohelix/analysis/v1`) or a clean terminal table. |
| **Viewer payload**            | Compact JSON (`repohelix/viewer/v1`) for the browser helix/atlas.            |
| **Deterministic output**      | Same input bytes → same output bytes. Ideal for snapshot tests.              |
| **Zero dependencies**         | Rust CLI uses only `std`; the viewer uses only the DOM. TypeScript is a dev tool. |

---

## Install and build

You need a recent Rust toolchain (1.70+; developed on 1.98) and a local `git`.

```sh
# Build the release binary.
cargo build --release
# Binary lands at target/release/repohelix (.exe on Windows).

# Or run straight from source during development.
cargo run -- help
```

There is nothing else to install. The manifest has an intentionally empty
`[dependencies]` table — `repohelix` is a pure standard-library program.

---

## Quick start (30 seconds)

The repository ships with a synthetic fixture, `fixtures/nebula.gitlog`, modeling
a small fictional web service. It exists so every demo and test is reproducible
without a live repository.

```sh
# 1. Human-readable report of the fixture history.
cargo run -- analyze --log fixtures/nebula.gitlog --format text

# 2. Same analysis as pretty JSON.
cargo run -- analyze --log fixtures/nebula.gitlog --format json

# 3. Analyze THIS repository (or any Git repo).
cargo run -- analyze --repo . --format text

# 4. Emit the compact payload the browser viewer consumes.
cargo run -- viewer-data --log fixtures/nebula.gitlog --out viewer/data.json
```

The text report for the fixture opens like this (abridged):

```
repohelix — Git history analysis
================================

Summary
  commits ......... 24
  files ........... 18
  authors ......... 4
  total churn ..... 2744
  span (days) ..... 20

Top hotspots (churn x frequency)
  path                                       churn commits   score
  src/handlers.rs                              684      10   1.000
  src/router.rs                                410       9   0.734
