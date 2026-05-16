use crate::algorithms::compression::{CompressionCandidate, Pattern};
use crate::algorithms::graph::TransitionGraph;
use crate::algorithms::similarity::{AlignmentStep, SimilarityResult};
use crate::algorithms::transform::{contour, intervals};
use crate::core::{Beats, Motif, Note, Pitch};

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

pub fn format_compression_candidates(candidates: &[CompressionCandidate]) -> String {
    if candidates.is_empty() {
        return "No repeated patterns found.".to_string();
    }

    let mut lines = vec!["Repeated patterns:".to_string()];

    for (index, candidate) in candidates.iter().take(8).enumerate() {
        lines.push(format!(
            "{}. {} | length {} | occurrences {} | starts {:?} | savings {}",
            index + 1,
            format_pattern(&candidate.pattern),
            candidate.length,
            candidate.occurrence_count,
            candidate.start_indices,
            candidate.savings_score
        ));
    }

    lines.join("\n")
}

pub fn format_transition_graph(graph: &TransitionGraph) -> String {
    if graph.edges.is_empty() {
        return "No transitions found.".to_string();
    }

    let mut lines = vec!["Transitions:".to_string()];
    for (from, targets) in &graph.edges {
        let transitions = targets
            .iter()
            .map(|(to, count)| format!("{to} ({count})"))
            .collect::<Vec<_>>()
            .join(", ");
        lines.push(format!("{from} -> {transitions}"));
    }

    lines.join("\n")
}

pub fn format_pitch_walk(walk: &[Pitch]) -> String {
    if walk.is_empty() {
        return "Walk:".to_string();
    }

    format!(
        "Walk:\n{}",
        walk.iter()
            .map(ToString::to_string)
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
    let pitch = note
        .spelling
        .map(|spelling| spelling.format_pitch(note.pitch))
        .unwrap_or_else(|| note.pitch.to_string());

    format!("{}:{}", pitch, note.duration)
}

fn format_pattern(pattern: &Pattern) -> String {
    match pattern {
        Pattern::Notes(notes) => notes
            .iter()
            .map(|note| format!("{}:{}", note.pitch, format_beats(note.duration)))
            .collect::<Vec<_>>()
            .join(" "),
        Pattern::Intervals(intervals) => format!("intervals [{}]", format_intervals(intervals)),
    }
}

fn format_beats(beats: Beats) -> String {
    beats.to_string()
}
