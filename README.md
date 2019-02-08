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
  src/storage.rs                               343       7   0.592
  ...

Strongest temporal coupling (co-change)
    str  conf file A                     file B
   0.58  0.70 src/handlers.rs            src/router.rs
   0.38  0.43 Cargo.toml                 src/lib.rs
   ...

Note: these metrics describe code and change activity, not people.
```

`handlers.rs` is the clear hotspot, and `handlers.rs ↔ router.rs` is the tightest
coupling — exactly what the fixture's story (a router wired to handlers, then
repeatedly refactored together) is designed to reveal.

---

## The command line

```
repohelix <command> [options]

COMMANDS
  analyze        Analyze history and print a JSON or text report
  viewer-data    Emit compact JSON for the TypeScript helix viewer
  help           Show help
  version        Show version

SOURCE (pick one; defaults to --repo .)
  --repo <dir>              Analyze a live Git repository
  --log <file>              Import a captured/fixture git log file

ANALYZE OPTIONS
  --format <json|text>      Output format (default: text)
  --compact                 Compact JSON instead of pretty
  --max-commits <n>         Limit history depth (newest n commits)
  --out <file>              Write to a file instead of stdout
  --min-cochange <n>        Minimum co-change count to report (default: 2)
  --top-cochange <n>        Max co-change pairs (default: 40)
  --top-hotspots <n>        Max hotspots (default: 20)
  --cluster-gap <secs>      Session gap threshold (default: 21600 = 6h)
  --max-files-cochange <n>  Skip huge commits for coupling (default: 40)
```

Every git invocation sets `GIT_PAGER=cat`, `GIT_TERMINAL_PROMPT=0`,
`GIT_OPTIONAL_LOCKS=0`, `GIT_CONFIG_NOSYSTEM=1`, and `LC_ALL=C`, plus the
`--no-pager --no-color --no-renames` flags. It will never open an editor, block
on a prompt, or emit ANSI colour into your pipeline.

---

## What it computes

A one-paragraph summary of each metric; the precise formulas live in
[`docs/MODEL.md`](docs/MODEL.md).

- **Churn** — total lines added plus removed per file. The rawest activity
  signal. Binary edits count as one unit so they still register.
- **Hotspots** — `sqrt(normalizedChurn · normalizedFrequency)`. Only files that
  are *both* large and frequently edited rise to the top; a huge one-shot import
  or a trivial repeated tweak both score low.
- **Temporal coupling** — for a file pair, `strength = together / (a + b −
  together)` (Jaccard) and `confidence = together / max(a, b)` (association
  rule). Guarded by a minimum co-occurrence and a max-files-per-commit filter so
  bulk commits don't manufacture fake coupling.
- **Temporal clusters** — commits sorted by time and split wherever the gap
  exceeds the threshold. Each cluster is a development *session* with its own
  commit count, file span, and churn.
- **Ownership concentration** — a normalized Herfindahl index of per-file commit
  shares in `[0, 1]`. `1.0` = one identity made every change (bus factor of
  one); low = shared stewardship. Identities are reported as anonymized labels.

---

## Output formats

### Text

A compact terminal report with a summary block and four ranked tables
(hotspots, coupling, ownership, clusters). Every text report ends with the
reminder that the metrics describe code, not people.

### JSON (`schema: repohelix/analysis/v1`)

A stable, sorted document with top-level keys `summary`, `files`, `cochanges`,
`hotspots`, `ownership`, and `clusters`, plus a `disclaimer` string embedded in
the payload itself. Pretty by default; `--compact` for pipelines.

```jsonc
{
  "schema": "repohelix/analysis/v1",
  "disclaimer": "Metrics describe file and change activity only. ...",
  "summary": { "commits": 24, "files": 18, "authors": 4, "total_churn": 2744, "span_days": 20, ... },
  "files":     [ { "path": "src/handlers.rs", "churn": 684, "commits": 10, "authors": 4, ... }, ... ],
  "cochanges": [ { "a": "src/handlers.rs", "b": "src/router.rs", "strength": 0.583333, ... }, ... ],
  "hotspots":  [ { "path": "src/handlers.rs", "score": 1.0, ... }, ... ],
  "ownership": [ { "path": "src/migrate.rs", "concentration": 1.0, "top_identity_label": "author#4" }, ... ],
  "clusters":  [ { "index": 0, "commits": 4, "files_touched": 7, "churn": 426, ... }, ... ]
}
```

The bundled JSON writer (`src/json.rs`) escapes correctly, preserves key order,
trims float noise, and emits `null` for non-finite numbers so the output is
always valid JSON.

### Viewer payload (`schema: repohelix/viewer/v1`)

A minimal graph: `nodes` (files with churn/commits/authors/concentration/time),
`links` (co-change pairs referencing node indices), and `clusters`. Capped at 64
nodes so the browser render stays smooth and the file stays tiny.

---

## The interactive viewer

The `viewer/` directory holds a dependency-free TypeScript app that renders the
payload as the same helix + atlas you see at the top of this README — but
interactive, with hover tooltips.

```sh
cd viewer
npm run build       # tsc → dist/viewer.js (+ dist/core.js)

# Regenerate the embedded demo dataset from the fixture (Windows PowerShell):
pwsh -File ../scripts/gen_viewer_data.ps1   # writes viewer/data.js

# Then open viewer/index.html in any browser. It loads data.js locally.
```

The viewer is split into two modules:

- **`src/core.ts`** — pure, DOM-free math and data shaping (colour ramps, churn
  scaling, ring layout, ordering, validation, summary text). This is what the
  unit tests exercise under plain Node.
- **`src/viewer.ts`** — the SVG rendering built on top of `core`, plus the
  auto-mount glue.

No framework, no bundler, no CDN, no network. The only dev dependency is
TypeScript itself, and even that is optional if you write the JS by hand.

---

## The fixture format (deterministic demos)

A **fixture log** is simply captured `git log --numstat` output using the custom
record/field separators `repohelix` requests. Because the parser is the same one
used for live repositories, a fixture behaves identically to a real repo — but is
byte-for-byte reproducible.

Regenerate the bundled `nebula` fixture with:

```sh
pwsh -File scripts/gen_fixture.ps1     # writes fixtures/nebula.gitlog
```

The generator encodes a deliberate story across four development sessions
(clusters): a bootstrap, a data layer, a feature push, and a hardening pass. That
story is what produces the hotspot on `handlers.rs`, the strong
`handlers ↔ router` coupling, and the single-owner files like `migrate.rs`. The
integration tests assert on exactly these outcomes, so the fixture doubles as a
golden dataset.

To capture your *own* fixture from a real repo, run `repohelix analyze --repo .`
inside it — or record git's raw output — and store it under `fixtures/`.

---

## Architecture

```
repohelix/
├── Cargo.toml, Cargo.lock      # zero-dependency Rust manifest
├── src/
│   ├── main.rs                 # CLI entry point, output emission
│   ├── lib.rs                  # public API, pipeline (load_from_repo/log)
│   ├── cli.rs                  # hand-rolled argument parsing
│   ├── git.rs                  # non-interactive git invocation
│   ├── parse.rs                # git-log → History parser (also fixtures)
│   ├── model.rs                # Commit / FileChange / History types
│   ├── metrics.rs              # churn, co-change, clusters, hotspots, ownership
│   ├── report.rs               # JSON + text rendering
│   ├── json.rs                 # minimal std-only JSON writer
│   └── viewer.rs               # compact viewer-payload builder
├── tests/integration.rs        # end-to-end tests against the fixture
├── fixtures/nebula.gitlog      # synthetic, deterministic history
├── viewer/                     # dependency-free TypeScript viewer
│   ├── src/core.ts             # pure logic (tested under Node)
