# AGENTS.md

## Project Overview

This repository is an exploratory Rust project for building a modular algorithmic composition engine, tentatively called **Motif Lab**.

The goal is not to build a full DAW, web app, or production audio system at the start. The goal is to create a clean, testable Rust codebase for representing musical ideas as data structures and applying algorithms from music theory, dynamic programming, compression, graph theory, grammar systems, and search.

The motivating idea:

> Build a portable compositional workbench for exploring music through algorithms, especially for a musician traveling without regular access to a physical instrument.

This should feel like a serious systems/algorithmic Rust project with musical meaning, not a toy generator.

---

## Core Design Philosophy

Prefer a small, modular, well-tested engine over a large app.

Work in this order:

1. Core musical data structures
2. Parsing and printing simple human-readable motif files
3. Pure transformation functions
4. Analysis algorithms
5. Dynamic programming similarity
6. Compression / repeated pattern discovery
7. Graph representation and generation
8. Grammar / DSL experiments
9. MIDI export
10. Optional UI, WASM, or interactive layer later

Do **not** prematurely build:

- A web UI
- A full DAW
- Real-time audio synthesis
- MusicXML support
- A plugin system
- Cloud/database features
- AI generation
- Complex notation rendering

The project should remain CLI-first and library-first until the core engine is useful.

---

## Expected Repository Shape

Use this structure unless the existing repository already has a better equivalent:

```text
motif-lab/
  Cargo.toml
  README.md
  AGENTS.md

  src/
    main.rs
    lib.rs

    core/
      mod.rs
      pitch.rs
      note.rs
      motif.rs
      rhythm.rs

    io/
      mod.rs
      parse.rs
      print.rs
      midi.rs

    algorithms/
      mod.rs
      transform.rs
      similarity.rs
      compression.rs
      graph.rs
      grammar.rs
      search.rs

    cli/
      mod.rs
      commands.rs

  examples/
    simple.motif
    repeated.motif
    passing_tone_a.motif
    passing_tone_b.motif
    travel_sketch_01.motif

  tests/
    transform_tests.rs
    similarity_tests.rs
    compression_tests.rs

  notes/
    001-representation.md
    002-transformations.md
    003-edit-distance.md
    004-compression.md
    005-graphs.md
    006-grammar.md
```

If working inside an existing structure, preserve the existing architecture where reasonable, but keep the separation between `core`, `io`, `algorithms`, and `cli`.

---

## Rust Coding Standards

Use idiomatic, readable Rust.

Priorities:

1. Correctness
2. Simplicity
3. Testability
4. Clear domain modeling
5. Extensibility
6. Performance only when needed

Avoid clever abstractions early. Prefer direct, understandable code.

Use strong types for musical concepts where helpful:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pitch(pub i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Beats(pub f32);

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub pitch: Pitch,
    pub start: Beats,
    pub duration: Beats,
    pub velocity: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Motif {
    pub notes: Vec<Note>,
}
```

Use `Result<T, E>` for fallible parsing and IO. Avoid `unwrap()` and `expect()` outside tests, examples, or truly impossible internal invariants.

Use `thiserror` or a simple custom error enum if error handling grows.

Use `clap` for CLI argument parsing.

Use `serde` only if/when structured serialization is useful. Do not introduce it before it is needed.

---

## Functional Style Preference

Favor pure functions for transformations and algorithms.

Prefer:

```rust
pub fn transpose(motif: &Motif, semitones: i32) -> Motif
```

Over mutation-heavy APIs like:

```rust
motif.transpose_in_place(semitones);
```

In-place versions can be added later if performance requires them.

Transformation functions should be easy to compose and easy to test.

Examples:

```rust
pub fn retrograde(motif: &Motif) -> Motif
pub fn invert(motif: &Motif, axis: Pitch) -> Motif
pub fn augment(motif: &Motif, factor: f32) -> Motif
pub fn diminish(motif: &Motif, factor: f32) -> Motif
pub fn intervals(motif: &Motif) -> Vec<i32>
pub fn contour(motif: &Motif) -> Vec<ContourStep>
```

---

## Initial Motif File Format

Start with a very simple text format that is easy to write by hand while traveling.

Initial format:

```text
C4:1 D4:1 E4:1 G4:2 E4:1
```

Where:

- `C4` is pitch
- `:1` is duration in beats
- Notes are separated by whitespace
- Start times are inferred sequentially

Support sharps/flats eventually:

```text
C#4:1 Bb3:0.5 F4:2
```

Do not build a complex DSL before the simple format works well.

Later DSL ideas can include:

```text
motif A = C4:1 D4:1 E4:2
motif B = invert A around C4
phrase P = repeat A 3 then B
```

But this should come after core modules are stable.

---

## CLI Milestones

The first useful CLI should support these commands:

```bash
motif-lab analyze examples/simple.motif
motif-lab transform examples/simple.motif --transpose 5
motif-lab transform examples/simple.motif --retrograde
motif-lab transform examples/simple.motif --invert C4
motif-lab compare examples/passing_tone_a.motif examples/passing_tone_b.motif
motif-lab compress examples/repeated.motif
```

Expected style of output:

```text
Notes: 5
Duration: 6 beats
Pitch range: C4 to G4
Intervals: +2 +2 +3 -3
Contour: up up up down
```

For transformations, output the transformed motif in the same simple text format:

```text
G4:1 A4:1 B4:1 D5:2 B4:1
```

For comparisons:

```text
Similarity: 0.82

Alignment:
C4  D4  E4  G4  E4
C4  --  E4  G4  E4

Interpretation:
B can be heard as A with one passing tone removed.
```

The interpretation line can be heuristic and should not overclaim.

---

## Algorithm Modules

### `transform.rs`

Implement basic musical transformations first:

- `transpose`
- `retrograde`
- `invert`
- `augment`
- `diminish`
- `normalize_start`
- `intervals`
- `contour`

Keep these deterministic and well-tested.

### `similarity.rs`

Implement dynamic-programming motif comparison.

Start simple:

- Insert cost: `1.0`
- Delete cost: `1.0`
- Substitute cost: based on pitch distance and duration difference

Then refine toward musical cost functions:

- Same pitch: `0.0`
- Same pitch class: low cost
- Stepwise substitution: lower cost
- Large leap substitution: higher cost
- Rhythmic difference: additive cost

The output should eventually include:

- Raw edit distance
- Normalized similarity score
- Alignment path
- Operation list: keep, insert, delete, substitute

Do not make the first version too complex.

### `compression.rs`

Start with repeated n-gram detection over note or interval sequences.

Initial goals:

- Find repeated subsequences of length 2-8
- Count occurrences
- Rank by simple compression savings
- Print candidate compressed motifs

Possible simple scoring:

```text
savings = (pattern_length * occurrence_count) - pattern_length - occurrence_count
```

This does not need to be theoretically perfect. It is an exploratory musical compression prototype.

Later extensions:

- LZ-style dictionary compression
- Grammar compression
- Minimum description length scoring
- Compression over intervals instead of absolute pitches
- Compression over contour instead of exact notes

### `graph.rs`

Represent motifs as transition graphs.

Possible node types:

- Exact pitches
- Pitch classes
- Intervals
- Contour states
- Harmonic/tension states later

Initial capabilities:

- Build transition graph from motif
- Count transitions
- Print adjacency list
- Generate random walk continuation

Later:

- Weighted random walks
- Shortest path between musical states
- Dijkstra/A* experiments
- Motif centrality / repeated nodes

### `grammar.rs`

Do not implement this too early.

Eventually, support a tiny musical grammar or DSL:

```text
motif A = C4:1 D4:1 E4:1
motif B = transpose A +5
phrase P = A A retrograde(B)
```

Implementation should include:

- Lexer
- Parser
- AST
- Interpreter to `Motif`

Keep syntax minimal and documented.

### `search.rs`

Use this for constraint-based or heuristic generation.

Possible algorithms:

- Backtracking
- Beam search
- Random search
- Simulated annealing later
- Genetic algorithms later

Do not start here. This module depends on a stable representation and scoring functions.

---

## Testing Expectations

Every core transformation and algorithm should have tests.

Use unit tests for small pure functions and integration tests for CLI-level behavior.

Examples:

```rust
#[test]
fn transpose_moves_all_pitches_by_interval() {
    // ...
}

#[test]
fn retrograde_preserves_total_duration() {
    // ...
}

#[test]
fn repeated_ngram_detects_simple_repetition() {
    // ...
}
```

Tests should verify musical invariants, not just exact values.

Useful invariants:

- Transposition preserves durations
- Retrograde preserves total duration
- Inversion around an axis maps pitch distances symmetrically
- Augmentation multiplies total duration
- Interval extraction for `C4 D4 E4` returns `[2, 2]`
- Compression finds repeated motifs in obvious examples
- Similarity of identical motifs is maximal

Run before committing:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

If `clippy -D warnings` is too strict during early exploration, document why and keep warnings minimal.

---

## Documentation Expectations

Keep the README practical and portfolio-ready.

README should eventually include:

1. What Motif Lab is
2. Why it exists
3. Example commands
4. Example musical outputs
5. Algorithmic modules
6. Roadmap
7. Notes on musical/computational philosophy

Maintain a `notes/` folder as a lab notebook.

Each note should answer:

- What is the musical question?
- What is the computational idea?
- What did this experiment implement?
- What worked?
- What felt wrong or limited?
- What is the next experiment?

This project should read as a creative engineering research project, not just a utility.

---

## Dependency Policy

Keep dependencies minimal.

Good early dependencies:

- `clap` for CLI parsing
- `thiserror` for clean errors, if needed
- `pretty_assertions` for tests, optional

Potential later dependencies:

- MIDI crate for export
- `petgraph` for graph algorithms
- parser combinator crate such as `nom` or `pest`, but only once the DSL needs it
- `serde` for serialization

Avoid adding a large dependency before the project has a concrete need.

If implementing a learning algorithm, first implement the simple version manually before reaching for a crate. The goal is to practice algorithms, not only wire libraries together.

---

## Senior Dev Guidance for Codex

When adding code:

1. Make the smallest meaningful change.
2. Preserve or improve module boundaries.
3. Add tests with each algorithm.
4. Keep public APIs narrow.
5. Prefer clear names over clever abstractions.
6. Do not introduce async unless IO requires it.
7. Do not add a database, server, UI, or web framework.
8. Do not rewrite working code without a clear reason.
9. Do not overfit the model to Western classical theory only; keep representations general where reasonable.
10. Keep musical interpretation humble. Algorithms suggest structure; they do not determine meaning.

When uncertain, choose the simpler implementation and leave a clear TODO.

Use TODOs like:

```rust
// TODO: Support enharmonic spelling instead of normalizing everything to MIDI pitch.
```

Avoid vague TODOs like:

```rust
// TODO: make better
```

---

## Musical Modeling Principles

Do not assume one representation of music is final.

The same motif may be represented as:

- Absolute pitches
- Intervals
- Pitch classes
- Contour
- Durations
- Rhythmic cells
- Graph transitions
- Grammar expansions
- Compressed patterns

Design APIs so these representations can coexist.

Example:

```rust
pub fn intervals(motif: &Motif) -> Vec<i32>
pub fn pitch_classes(motif: &Motif) -> Vec<u8>
pub fn rhythmic_pattern(motif: &Motif) -> Vec<Beats>
pub fn contour(motif: &Motif) -> Vec<ContourStep>
```

Avoid putting too much intelligence into `Motif` itself. Keep analysis functions separate.

---

## MVP Definition

The first true MVP is:

> A Rust CLI that reads a simple motif file, analyzes interval/contour structure, applies basic transformations, compares two motifs using dynamic programming, and detects repeated fragments for compression.

MVP commands:

```bash
motif-lab analyze simple.motif
motif-lab transform simple.motif --retrograde
motif-lab transform simple.motif --invert C4
motif-lab compare a.motif b.motif
motif-lab compress repeated.motif
```

Do not expand beyond this MVP until these commands work reliably and have tests.

---

## Suggested First 10 Commits

1. Initialize Rust CLI project
2. Add `Pitch`, `Beats`, `Note`, and `Motif`
3. Add simple `.motif` parser
4. Add motif analyzer: duration, range, intervals, contour
5. Add transpose transformation
6. Add retrograde transformation
7. Add inversion around axis pitch
8. Add tests for core transformations
9. Add compare command with simple edit distance
10. Add repeated n-gram compression prototype

---

## Long-Term Roadmap

After the MVP, consider these directions:

### MIDI Export

Export transformed or generated motifs to `.mid` so results can be heard in any DAW or notation tool.

### Graph Composition

Use transition graphs and random walks to create continuations from existing motifs.

### Musical DSL

Create a small language for describing motifs, transformations, and phrase structures.

### Constraint-Based Generation

Generate motifs under constraints such as:

- Stay within one octave
- End on tonic
- Use only five notes
- Increase tension over time
- Include exactly two repeated cells

### Travel Sketch Mode

Allow motif files to include metadata:

```text
@title Valparaiso Sketch
@place Valparaiso, Chile
@date 2026-05-14
@mood salt, height, color, decay

C4:1 Eb4:1 G4:2 F4:1
```

This can later connect the project to a Watson conference narrative.

### WASM or UI Layer

Only after the Rust engine is useful, consider compiling the core to WASM and building a lightweight interface around it.

The UI should remain secondary to the engine.

---

## Aesthetic Direction

This project should feel like a cross between:

- A musician's sketchbook
- An algorithms textbook
- A symbolic composition engine
- A travel notebook
- A small compiler project

Avoid generic AI-music framing.

Preferred framing:

> This project does not try to automate composition. It builds computational objects that help a musician think compositionally while away from an instrument.

---

## Definition of Good Work

A good contribution to this repository should usually do at least one of the following:

- Make musical data easier to represent
- Make transformations more reliable
- Add a meaningful algorithmic lens
- Improve test coverage
- Improve CLI usability
- Clarify documentation
- Preserve simplicity while increasing expressive power

A poor contribution would:

- Add large architecture before it is needed
- Add UI before the engine is meaningful
- Introduce dependencies without justification
- Hide musical logic behind unclear abstractions
- Generate music without explaining the representation or algorithm
- Make the code harder to test

---

## Immediate Next Step

Start by implementing the smallest end-to-end loop:

```bash
motif-lab analyze exa