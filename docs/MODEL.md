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
