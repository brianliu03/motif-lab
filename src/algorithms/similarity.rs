use crate::core::{Motif, Note};

const INSERT_COST: f32 = 1.0;
const DELETE_COST: f32 = 1.0;

#[derive(Debug, Clone, PartialEq)]
pub struct SimilarityResult {
    pub distance: f32,
    pub similarity: f32,
    pub alignment: Vec<AlignmentStep>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlignmentStep {
    pub operation: EditOperation,
    pub left: Option<Note>,
    pub right: Option<Note>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditOperation {
    Keep,
    Insert,
    Delete,
    Substitute,
}

impl EditOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            EditOperation::Keep => "keep",
            EditOperation::Insert => "insert",
            EditOperation::Delete => "delete",
            EditOperation::Substitute => "substitute",
        }
    }
}

pub fn compare_motifs(left: &Motif, right: &Motif) -> SimilarityResult {
    let left_len = left.notes.len();
    let right_len = right.notes.len();

    if left_len == 0 && right_len == 0 {
        return SimilarityResult {
            distance: 0.0,
            similarity: 1.0,
            alignment: Vec::new(),
        };
    }

    let mut distances = vec![vec![0.0; right_len + 1]; left_len + 1];
    let mut operations = vec![vec![EditOperation::Keep; right_len + 1]; left_len + 1];

    for i in 1..=left_len {
        distances[i][0] = distances[i - 1][0] + DELETE_COST;
        operations[i][0] = EditOperation::Delete;
    }

    for j in 1..=right_len {
        distances[0][j] = distances[0][j - 1] + INSERT_COST;
        operations[0][j] = EditOperation::Insert;
    }

    for i in 1..=left_len {
        for j in 1..=right_len {
            let substitution_cost = substitution_cost(&left.notes[i - 1], &right.notes[j - 1]);
            let diagonal_operation = if substitution_cost == 0.0 {
                EditOperation::Keep
            } else {
                EditOperation::Substitute
            };

            let candidates = [
                (
                    distances[i - 1][j - 1] + substitution_cost,
                    diagonal_operation,
                ),
                (distances[i - 1][j] + DELETE_COST, EditOperation::Delete),
                (distances[i][j - 1] + INSERT_COST, EditOperation::Insert),
            ];

            let mut best = candidates[0];
            for candidate in candidates.into_iter().skip(1) {
                if candidate.0 < best.0 {
                    best = candidate;
                }
            }

            distances[i][j] = best.0;
            operations[i][j] = best.1;
        }
    }

    let distance = distances[left_len][right_len];
    let max_notes = left_len.max(right_len) as f32;
    let similarity = (1.0 - (distance / max_notes)).clamp(0.0, 1.0);

    SimilarityResult {
        distance,
        similarity,
        alignment: backtrace_alignment(left, right, &operations),
    }
}

fn substitution_cost(left: &Note, right: &Note) -> f32 {
    let pitch_distance = (left.pitch.0 - right.pitch.0).abs() as f32;
    let normalized_pitch_distance = (pitch_distance / 12.0).min(1.0);

    let max_duration = left.duration.0.max(right.duration.0);
    let normalized_duration_difference = if max_duration == 0.0 {
        0.0
    } else {
        ((left.duration.0 - right.duration.0).abs() / max_duration).min(1.0)
    };

    normalized_pitch_distance + normalized_duration_difference
}

fn backtrace_alignment(
    left: &Motif,
    right: &Motif,
    operations: &[Vec<EditOperation>],
) -> Vec<AlignmentStep> {
    let mut alignment = Vec::new();
    let mut i = left.notes.len();
    let mut j = right.notes.len();

    while i > 0 || j > 0 {
        let operation = operations[i][j];
        match operation {
            EditOperation::Keep | EditOperation::Substitute => {
                alignment.push(AlignmentStep {
                    operation,
                    left: Some(left.notes[i - 1].clone()),
                    right: Some(right.notes[j - 1].clone()),
                });
                i -= 1;
                j -= 1;
            }
            EditOperation::Delete => {
                alignment.push(AlignmentStep {
                    operation,
                    left: Some(left.notes[i - 1].clone()),
                    right: None,
                });
                i -= 1;
            }
            EditOperation::Insert => {
                alignment.push(AlignmentStep {
                    operation,
                    left: None,
                    right: Some(right.notes[j - 1].clone()),
                });
                j -= 1;
            }
        }
    }

    alignment.reverse();
    alignment
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::parse::parse_motif;

    #[test]
    fn identical_motifs_have_similarity_one() {
        let motif = parse_motif("C4:1 D4:1 E4:2").unwrap();
        let result = compare_motifs(&motif, &motif);

        assert_eq!(result.distance, 0.0);
        assert_eq!(result.similarity, 1.0);
        assert!(result
            .alignment
            .iter()
            .all(|step| step.operation == EditOperation::Keep));
    }

    #[test]
    fn passing_tone_is_more_similar_than_unrelated_pitches() {
        let base = parse_motif("C4:1 E4:1 G4:1").unwrap();
        let passing_tone = parse_motif("C4:1 D4:0.5 E4:1 G4:1").unwrap();
        let unrelated = parse_motif("F#5:1 C#3:1 Bb5:1").unwrap();

        let passing_result = compare_motifs(&base, &passing_tone);
        let unrelated_result = compare_motifs(&base, &unrelated);

        assert!(passing_result.similarity > unrelated_result.similarity);
        assert!(passing_result
            .alignment
            .iter()
            .any(|step| step.operation == EditOperation::Insert));
    }

    #[test]
    fn empty_motifs_are_handled_gracefully() {
        let empty = parse_motif("").unwrap();
        let motif = parse_motif("C4:1 D4:1").unwrap();

        let empty_to_empty = compare_motifs(&empty, &empty);
        let empty_to_motif = compare_motifs(&empty, &motif);

        assert_eq!(empty_to_empty.distance, 0.0);
        assert_eq!(empty_to_empty.similarity, 1.0);
        assert!(empty_to_empty.alignment.is_empty());

        assert_eq!(empty_to_motif.distance, 2.0);
        assert_eq!(empty_to_motif.similarity, 0.0);
        assert!(empty_to_motif
            .alignment
            .iter()
            .all(|step| step.operation == EditOperation::Insert));
    }
}
