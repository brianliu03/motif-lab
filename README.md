# Motif Lab

Motif Lab is a Rust-based algorithmic composition engine for musical thinking away from the instrument. It treats short musical ideas as structured data, then applies small, inspectable algorithms for analysis, transformation, comparison, compression, and graph-based continuation.

The project is intentionally CLI-first and library-first. It is not a DAW, audio engine, notation system, or web app. The first goal is a clean Rust core that makes musical structure easy to represent, test, and transform.

## Current Motif Format

Motifs are plain text files made of whitespace-separated notes:

```text
C4:1 D4:1 E4:1 G4:2 E4:1
```

Each token has:

- a pitch name, such as `C4`, `F#4`, or `Bb3`
- a duration in beats after `:`
- an inferred start time based on the notes before it

For example, `C4:1 D4:0.5 E4:2` becomes three notes starting at beats `0`, `1`, and `1.5`.

## System Design

The engine is organized as a simple pipeline:

```text
.motif text
   |
   v
io::parse
   |
   v
core data model: Motif -> Note -> Pitch / Beats
   |
   +--> algorithms::transform
   +--> algorithms::similarity
   +--> algorithms::compression
   +--> algorithms::graph
   |
   v
io::print
   |
   v
CLI output
```

The CLI is intentionally thin. It reads files, calls the parser, dispatches to one algorithm module, and prints a human-readable result. Most musical behavior lives in pure functions that accept `&Motif` and return a new value or analysis result.

## Data Model

The core model is deliberately small:

```text
Motif
  notes: Vec<Note>

Note
  pitch: Pitch
  start: Beats
  duration: Beats
  velocity: u8

Pitch
  i32 MIDI-like semitone number

Beats
  f32 beat value
```

`Pitch` stores numeric pitch identity, not notation spelling. For example, `Db4` and `C#4` both parse to the same pitch number. This makes interval math, comparison, graph nodes, and transformations straightforward, but it means the printer currently emits a canonical spelling rather than preserving every enharmonic spelling from the source file.

## Component Layout

```text
src/
  core/
    pitch.rs      Pitch parsing, display, and interval distance
    rhythm.rs     Beats wrapper
    note.rs       Note data structure
    motif.rs      Motif container and basic aggregate queries

  io/
    parse.rs      .motif text -> Motif
    print.rs      Motif and algorithm results -> CLI text

  algorithms/
    transform.rs  transpose, retrograde, invert, augment, diminish, intervals, contour
    similarity.rs dynamic-programming motif comparison
    compression.rs repeated note and interval pattern discovery
    graph.rs      pitch transition graph and deterministic weighted walks

  cli/
    commands.rs   clap command definitions and command dispatch
```

## Algorithm Flow

### Analysis

`analyze` parses a motif and computes summary features:

- note count
- total duration
- pitch range
- interval sequence in semitones
- melodic contour as `up`, `down`, or `same`

The interval and contour logic lives in `algorithms::transform` because those features are also useful building blocks for later transformations and analyses.

### Transformation

`transform` applies one or more pure transformations to a parsed motif:

```text
Motif -> transform function -> Motif -> format_motif
```

Current transformations include:

- `transpose`: add semitones to every pitch
- `retrograde`: mirror note start times around the total duration, then sort by new start time
- `invert`: reflect each pitch around an axis pitch
- `augment`: multiply start times and durations
- `diminish`: divide start times and durations

The transformations return new motifs instead of mutating the input in place.

### Similarity

`compare` uses dynamic programming over two note sequences:

```text
left Motif + right Motif
   |
   v
edit-distance matrix
   |
   v
distance + normalized similarity + alignment path
```

Insert and delete cost `1.0`. Substitution cost combines normalized pitch distance and normalized duration difference. The result includes both a score and an alignment of keep, insert, delete, and substitute operations.

### Compression

`compress` searches for repeated fragments in two representations:

- exact note patterns, using pitch and duration
- interval patterns, using semitone movement

It checks repeated n-grams of length `2` through `8`, removes overlapping occurrences, and ranks candidates with a simple savings score:

```text
savings = pattern_length * occurrence_count - pattern_length - occurrence_count
```

This is an exploratory compression lens, not a final music-theory claim.

### Graphs And Walks

`graph` turns a motif into weighted pitch transitions:

```text
C4 D4 E4 D4 E4

C4 -> D4 (1)
D4 -> E4 (2)
E4 -> D4 (1)
```

`walk` uses those transition counts as weights and a deterministic seed to generate a reproducible pitch continuation.

## Commands

```bash
cargo run -- analyze examples/simple.motif
cargo run -- transform examples/simple.motif --transpose 5
cargo run -- transform examples/simple.motif --retrograde
cargo run -- transform examples/simple.motif --invert C4
cargo run -- compare examples/passing_tone_a.motif examples/passing_tone_b.motif
cargo run -- compress examples/repeated.motif
cargo run -- graph examples/simple.motif
cargo run -- walk examples/simple.motif --steps 8 --seed 42
```

## Design Principles

- Keep musical data structures explicit and testable.
- Prefer pure functions for algorithms and transformations.
- Keep the CLI as a small orchestration layer.
- Add musical interpretation cautiously; algorithms suggest structure, they do not determine meaning.
- Avoid large dependencies until a concrete algorithmic need appears.

## Roadmap

The near-term MVP is a Rust CLI that can read a simple motif file, analyze interval and contour structure, apply basic transformations, compare motifs using dynamic programming, and detect repeated fragments for compression.

Later directions may include MIDI export, richer graph composition, a small motif DSL, constraint-based search, and travel sketch metadata. Those layers should build on the core engine rather than replacing it.
