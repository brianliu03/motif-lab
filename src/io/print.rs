use crate::algorithms::transform::{contour, intervals};
use crate::core::Motif;

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
        .map(|note| format!("{}:{}", note.pitch, note.duration))
        .collect::<Vec<_>>()
        .join(" ")
}
