# The repohelix model

This document defines every metric `repohelix` computes, the exact formula, the
assumptions behind it, and — importantly — what each metric *does not* mean. If
you only read one section, read [Metrics are not personnel
judgments](#metrics-are-not-personnel-judgments).

`repohelix` treats a repository like a strand of DNA: a long, ordered record of
mutations. Each commit is a transcription event; each file is a locus that
mutates at its own rate. The tool reads that record and draws two maps — a
**helix** (files threaded through time) and a **co-change atlas** (files bound
by how often they mutate together). Everything below is descriptive
cartography, not evaluation.

## Input model

The unit of input is a **commit**, parsed from `git log --numstat` (or from a
captured *fixture log* with identical structure). Each commit carries:

| Field          | Source              | Notes                                        |
| -------------- | ------------------- | -------------------------------------------- |
| `hash`         | `%H`                | Commit identity.                             |
| `author_name`  | `%an`               | Display name.                                |
| `author_email` | `%ae`               | Preferred identity key (more stable).        |
| `timestamp`    | `%at`               | Author time, Unix epoch seconds.             |
| `subject`      | `%s`                | First line of the message.                   |
| `files[]`      | `--numstat` rows    | Per-file `added`, `removed`, `path`.         |

Renames are **not** followed (`--no-renames`). A renamed file therefore appears
as a deletion of the old path and creation of the new one. This keeps the model
simple and predictable; following renames is a possible future extension noted
in the CHANGELOG.

Binary files report `-`/`-` for line counts. `repohelix` treats a binary edit as
**one unit of churn** so the file still registers activity without inventing
line numbers.

## Churn

For a file `f`:

```
added(f)   = Σ added over all commits touching f
removed(f) = Σ removed over all commits touching f
churn(f)   = added(f) + removed(f)          (binary edits contribute 1 each)
commits(f) = number of commits touching f
```

Churn is the crudest possible activity signal: how much text moved through a
file. High churn is neither good nor bad. A parser generator, a lockfile, or a
central dispatch module can all be legitimately high-churn.

## Hotspots

A **hotspot** is a file that is high on *both* churn and change frequency —
the intersection where maintenance effort and change risk tend to concentrate.

Let `maxChurn` and `maxCommits` be the maxima across all files. For file `f`:

```
nc(f) = churn(f)   / maxChurn      (0 if maxChurn == 0)
nf(f) = commits(f) / maxCommits    (0 if maxCommits == 0)
score(f) = sqrt( nc(f) * nf(f) )   (geometric mean, range 0..1)
```

The geometric mean is deliberate: a file that is huge but changed once
(a vendored blob) scores low, and so does a file changed constantly by one-line
tweaks. Only files that are *both* large and frequently edited rise to the top.
This mirrors the "hotspot" idea popularized by Adam Tornhill's behavioural code
analysis, adapted to the standard-library-only constraints of this tool.

## Co-change / temporal coupling

Two files are **temporally coupled** if they tend to change in the same commit.
For an unordered pair `{a, b}`:

```
together(a,b) = number of commits that changed both a and b
count(a)      = number of commits that changed a
count(b)      = number of commits that changed b

strength(a,b)   = together / (count(a) + count(b) - together)   (Jaccard index)
confidence(a,b) = together / max(count(a), count(b))            (association rule)
```

- **strength** is symmetric and lives in `[0, 1]`. `1.0` means the two files
  *always* change together and never apart.
- **confidence** answers "given the more frequently changed of the pair
  changed, how often did the other change too?" It is the association-rule
  confidence using the busier file as antecedent, chosen for stability.

Two guards keep coupling honest:

1. `--min-cochange` (default 2): a pair must co-occur at least this many times
   to be reported, filtering one-off coincidences.
2. `--max-files-cochange` (default 40): commits touching more files than this
   (bulk imports, vendor drops, formatting sweeps) are excluded from pair
   counting, because they would manufacture spurious coupling between unrelated
   files. Their singleton counts are still recorded so confidence denominators
   stay correct.

Temporal coupling is the tool's most actionable signal: strongly coupled files
that live far apart in the directory tree often indicate a hidden dependency or
a missing abstraction.

## Temporal clusters (development sessions)

Commits are sorted ascending by time. A new **cluster** begins whenever the gap
to the previous commit exceeds `--cluster-gap` seconds (default `21600`, i.e.
six hours). Each cluster reports its commit count, distinct files touched, and
total churn.

Clusters approximate *sessions* — bursts of related work such as a feature
push or a release-hardening pass. They are a lens on *tempo*, not on who was
working or how hard.

## Ownership concentration

For each file we count commits per author identity (email preferred, name as
fallback). Let file `f` have identities with commit shares `p_1 … p_k` summing
