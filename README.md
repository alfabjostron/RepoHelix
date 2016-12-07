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
