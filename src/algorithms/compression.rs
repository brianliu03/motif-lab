use crate::algorithms::transform::intervals;
use crate::core::{Beats, Motif, Pitch};
use std::collections::HashMap;

const MIN_PATTERN_LENGTH: usize = 2;
const MAX_PATTERN_LENGTH: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub struct CompressionCandidate {
    pub pattern: Pattern,
    pub length: usize,
    pub occurrence_count: usize,
    pub start_indices: Vec<usize>,
    pub savings_score: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Notes(Vec<NotePatternItem>),
    Intervals(Vec<i32>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NotePatternItem {
    pub pitch: Pitch,
    pub duration: Beats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NoteKey {
    pitch: i32,
    duration_bits: u32,
}

impl NoteKey {
    fn from_item(item: NotePatternItem) -> Self {
        Self {
            pitch: item.pitch.0,
            duration_bits: item.duration.0.to_bits(),
        }
    }

    fn to_item(self) -> NotePatternItem {
        NotePatternItem {
            pitch: Pitch(self.pitch),
            duration: Beats(f32::from_bits(self.duration_bits)),
        }
    }
}

pub fn repeated_patterns(motif: &Motif) -> Vec<CompressionCandidate> {
    let mut candidates = Vec::new();

    let note_items = motif
        .notes
        .iter()
        .map(|note| NotePatternItem {
            pitch: note.pitch,
            duration: note.duration,
        })
        .collect::<Vec<_>>();
    let note_keys = note_items
        .iter()
        .copied()
        .map(NoteKey::from_item)
        .collect::<Vec<_>>();

    candidates.extend(detect_note_patterns(&note_keys));
    candidates.extend(detect_interval_patterns(&intervals(motif)));

    candidates.sort_by(|a, b| {
        b.savings_score
            .cmp(&a.savings_score)
            .then_with(|| b.occurrence_count.cmp(&a.occurrence_count))
            .then_with(|| b.length.cmp(&a.length))
    });

    candidates
}

fn detect_note_patterns(notes: &[NoteKey]) -> Vec<CompressionCandidate> {
    let mut candidates = Vec::new();

    for length in MIN_PATTERN_LENGTH..=MAX_PATTERN_LENGTH.min(notes.len()) {
        let mut occurrences: HashMap<Vec<NoteKey>, Vec<usize>> = HashMap::new();
        for start in 0..=notes.len() - length {
            occurrences
                .entry(notes[start..start + length].to_vec())
                .or_default()
                .push(start);
        }

        for (pattern, starts) in occurrences {
            let start_indices = non_overlapping_starts(starts, length);
            if start_indices.len() < 2 {
                continue;
            }

            let savings_score = savings_score(length, start_indices.len());
            candidates.push(CompressionCandidate {
                pattern: Pattern::Notes(pattern.into_iter().map(NoteKey::to_item).collect()),
                length,
                occurrence_count: start_indices.len(),
                start_indices,
                savings_score,
            });
        }
    }

    candidates
}

fn detect_interval_patterns(intervals: &[i32]) -> Vec<CompressionCandidate> {
    let mut candidates = Vec::new();

    for length in MIN_PATTERN_LENGTH..=MAX_PATTERN_LENGTH.min(intervals.len()) {
        let mut occurrences: HashMap<Vec<i32>, Vec<usize>> = HashMap::new();
        for start in 0..=intervals.len() - length {
            occurrences
                .entry(intervals[start..start + length].to_vec())
                .or_default()
                .push(start);
        }

        for (pattern, starts) in occurrences {
            let start_indices = non_overlapping_starts(starts, length);
            if start_indices.len() < 2 {
                continue;
            }

            let savings_score = savings_score(length, start_indices.len());
            candidates.push(CompressionCandidate {
                pattern: Pattern::Intervals(pattern),
                length,
                occurrence_count: start_indices.len(),
                start_indices,
                savings_score,
            });
        }
    }

    candidates
}

fn non_overlapping_starts(mut starts: Vec<usize>, length: usize) -> Vec<usize> {
    starts.sort_unstable();

    let mut selected = Vec::new();
    let mut next_allowed_start = 0;

    for start in starts {
        if selected.is_empty() || start >= next_allowed_start {
            selected.push(start);
            next_allowed_start = start + length;
        }
    }

    selected
}

fn savings_score(length: usize, occurrence_count: usize) -> i32 {
    (length * occurrence_count) as i32 - length as i32 - occurrence_count as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::parse::parse_motif;

    #[test]
    fn detects_obvious_repeated_note_pattern() {
        let motif =
            parse_motif("C4:1 D4:1 E4:1 C4:1 D4:1 E4:1 C4:1 D4:1 E4:1 G4:2")
                .unwrap();
        let candidates = repeated_patterns(&motif);

        let repeated_cde = candidates.iter().find(|candidate| {
            matches!(
                &candidate.pattern,
                Pattern::Notes(pattern)
                    if pattern
                        == &vec![
                            NotePatternItem {
                                pitch: Pitch(60),
                                duration: Beats(1.0),
                            },
                            NotePatternItem {
                                pitch: Pitch(62),
                                duration: Beats(1.0),
                            },
                            NotePatternItem {
                                pitch: Pitch(64),
                                duration: Beats(1.0),
                            },
                        ]
            )
        });

        let candidate = repeated_cde.unwrap();
        assert_eq!(candidate.length, 3);
        assert_eq!(candidate.occurrence_count, 3);
        assert_eq!(candidate.start_indices, vec![0, 3, 6]);
        assert_eq!(candidate.savings_score, 3);
    }

    #[test]
    fn detects_repeated_interval_patterns() {
        let motif =
            parse_motif("C4:1 D4:1 E4:1 C4:1 D4:1 E4:1 C4:1 D4:1 E4:1 G4:2")
                .unwrap();
        let candidates = repeated_patterns(&motif);

        assert!(candidates.iter().any(|candidate| {
            candidate.pattern == Pattern::Intervals(vec![2, 2])
                && candidate.start_indices == vec![0, 3, 6]
        }));
    }
}
