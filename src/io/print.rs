use crate::algorithms::transform::{contour, intervals};
use crate::algorithms::similarity::{AlignmentStep, SimilarityResult};
use crate::core::{Motif, Note};

pub fn format_analysis(motif: &Motif) -> String {
    let range = motif
        .pitch_range()
        .map(|(low, high)| format!("{low} to {high}"))
        .unwrap_or_else(|| "none".to_string());

    format!(
        "Notes: {}\nDuration: {} beats\nPitch range: {}\nIntervals: {}\nContour: {}",
        motif.note_count(),
        motif.total_duration(),
        range,
        format_intervals(&intervals(motif)),
        contour(motif)
            .iter()
            .map(|step| step.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn format_intervals(intervals: &[i32]) -> String {
    intervals
        .iter()
        .map(|interval| {
            if *interval > 0 {
                format!("+{interval}")
            } else {
                interval.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn format_motif(motif: &Motif) -> String {
    motif
        .notes
        .iter()
        .map(format_note)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn format_similarity(result: &SimilarityResult) -> String {
    format!(
        "Distance: {:.3}\nSimilarity: {:.2}\n\nAlignment:\n{}\n{}\n\nOperations:\n{}",
        result.distance,
        result.similarity,
        format_alignment_row(&result.alignment, |step| step.left.as_ref()),
        format_alignment_row(&result.alignment, |step| step.right.as_ref()),
        result
            .alignment
            .iter()
            .map(|step| step.operation.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn format_alignment_row<'a>(
    alignment: &'a [AlignmentStep],
    note: impl Fn(&'a AlignmentStep) -> Option<&'a Note>,
) -> String {
    alignment
        .iter()
        .map(|step| note(step).map(format_note).unwrap_or_else(|| "--".to_string()))
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_note(note: &Note) -> String {
    format!("{}:{}", note.pitch, note.duration)
}
