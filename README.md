# Motif Lab

Motif Lab is a Rust-based algorithmic composition engine for musical thinking away from the instrument. It treats short musical ideas as structured data, then applies small, inspectable algorithms for analysis, transformation, comparison, compression, and graph-based continuation.

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

The engine is organized as follows:

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

## Data Model

The core model is deliberately small:

```text
Motif
  notes: Vec<Note>

Note
  pitch: Pitch
  spelling: Option<PitchSpelling>
  start: Beats
  duration: Beats
  velocity: u8

Pitch
  i32 MIDI-like semitone number

PitchSpelling
  letter name plus optional sharp or flat for display

Beats
  f32 beat value
```

`Pitch` stores numeric pitch identity, while `PitchSpelling` preserves the written note name when available. For example, `Db4` and `C#4` both parse to the same sounding pitch number, but the original spelling can still be used when printing motifs back to text.

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

Transformations that do not change pitch class, such as `retrograde`, `augment`, and `diminish`, preserve the original spelling metadata. Pitch-changing transformations, such as `transpose` and `invert`, keep sounding pitch correctness and choose output spelling with a policy:

- `preserve-context`: infer flats or sharps from the source motif
- `flats`: prefer flat spellings
- `sharps`: prefer sharp spellings

The default policy is `preserve-context`. The transformations return new motifs instead of mutating the input in place.

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
cargo run -- transform examples/simple.motif --transpose 5 --spelling-policy flats
cargo run -- transform examples/simple.motif --retrograde
cargo run -- transform examples/simple.motif --invert C4
cargo run -- compare examples/passing_tone_a.motif examples/passing_tone_b.motif
cargo run -- compress examples/repeated.motif
cargo run -- graph examples/simple.motif
cargo run -- walk examples/simple.motif --steps 8 --seed 42
```

