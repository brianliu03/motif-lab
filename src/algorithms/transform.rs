use crate::core::{Beats, Motif, Pitch, PitchSpelling, SpellingPolicy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContourStep {
    Up,
    Down,
    Same,
}

impl ContourStep {
    pub fn as_str(self) -> &'static str {
        match self {
            ContourStep::Up => "up",
            ContourStep::Down => "down",
            ContourStep::Same => "same",
        }
    }
}

pub fn intervals(motif: &Motif) -> Vec<i32> {
    motif
        .notes
        .windows(2)
        .map(|pair| pair[0].pitch.interval_to(pair[1].pitch))
        .collect()
}

pub fn contour(motif: &Motif) -> Vec<ContourStep> {
    intervals(motif)
        .into_iter()
        .map(|interval| match interval.cmp(&0) {
            std::cmp::Ordering::Greater => ContourStep::Up,
            std::cmp::Ordering::Less => ContourStep::Down,
            std::cmp::Ordering::Equal => ContourStep::Same,
        })
        .collect()
}

pub fn transpose(motif: &Motif, semitones: i32) -> Motif {
    transpose_with_spelling(motif, semitones, SpellingPolicy::PreserveContext)
}

pub fn transpose_with_spelling(
    motif: &Motif,
    semitones: i32,
    spelling_policy: SpellingPolicy,
) -> Motif {
    let context = spelling_context(motif);

    Motif::new(
        motif
            .notes
            .iter()
            .map(|note| {
                let mut transformed = note.clone();
                transformed.pitch.0 += semitones;
                transformed.spelling =
                    Some(transformed.pitch.spelling(spelling_policy, &context));
                transformed
            })
            .collect(),
    )
}

pub fn retrograde(motif: &Motif) -> Motif {
    let total_duration = motif.total_duration().0;
    let mut notes = motif
        .notes
        .iter()
        .map(|note| {
            let mut transformed = note.clone();
            transformed.start = Beats(total_duration - (note.start.0 + note.duration.0));
            transformed
        })
        .collect::<Vec<_>>();

    notes.sort_by(|a, b| a.start.0.total_cmp(&b.start.0));
    Motif::new(notes)
}

pub fn invert(motif: &Motif, axis_pitch: Pitch) -> Motif {
    invert_with_spelling(motif, axis_pitch, SpellingPolicy::PreserveContext)
}

pub fn invert_with_spelling(
    motif: &Motif,
    axis_pitch: Pitch,
    spelling_policy: SpellingPolicy,
) -> Motif {
    let context = spelling_context(motif);

    Motif::new(
        motif
            .notes
            .iter()
            .map(|note| {
                let mut transformed = note.clone();
                transformed.pitch.0 = (2 * axis_pitch.0) - note.pitch.0;
                transformed.spelling =
                    Some(transformed.pitch.spelling(spelling_policy, &context));
                transformed
            })
            .collect(),
    )
}

pub fn augment(motif: &Motif, factor: f32) -> Motif {
    scale_time(motif, factor)
}

pub fn diminish(motif: &Motif, factor: f32) -> Motif {
    scale_time(motif, 1.0 / factor)
}

fn scale_time(motif: &Motif, factor: f32) -> Motif {
    Motif::new(
        motif
            .notes
            .iter()
            .map(|note| {
                let mut transformed = note.clone();
                transformed.start = Beats(note.start.0 * factor);
                transformed.duration = Beats(note.duration.0 * factor);
                transformed
            })
            .collect(),
    )
}

fn spelling_context(motif: &Motif) -> Vec<Option<PitchSpelling>> {
    motif.notes.iter().map(|note| note.spelling).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::parse::parse_motif;
    use crate::io::print::format_motif;

    #[test]
    fn extracts_intervals_in_semitones() {
        let motif = parse_motif("C4:1 D4:1 E4:1 G4:2 E4:1").unwrap();

        assert_eq!(intervals(&motif), vec![2, 2, 3, -3]);
    }

    #[test]
    fn extracts_contour_steps() {
        let motif = parse_motif("C4:1 D4:1 D4:1 B3:1").unwrap();

        assert_eq!(
            contour(&motif),
            vec![ContourStep::Up, ContourStep::Same, ContourStep::Down]
        );
    }

    #[test]
    fn transpose_preserves_durations() {
        let motif = parse_motif("C4:1 D4:0.5 G4:2").unwrap();
        let transformed = transpose(&motif, 5);

        assert_eq!(
            transformed
                .notes
                .iter()
                .map(|note| note.duration)
                .collect::<Vec<_>>(),
            motif
                .notes
                .iter()
                .map(|note| note.duration)
                .collect::<Vec<_>>()
        );
        assert_eq!(transformed.notes[0].pitch.0, motif.notes[0].pitch.0 + 5);
    }

    #[test]
    fn transpose_preserves_flat_context_by_default() {
        let motif = parse_motif("Ab4:4 F4:2 Eb4:3 Db4:2").unwrap();
        let transformed = transpose(&motif, 5);

        assert_eq!(format_motif(&transformed), "Db5:4 Bb4:2 Ab4:3 Gb4:2");
    }

    #[test]
    fn transpose_can_prefer_sharp_spellings() {
        let motif = parse_motif("Ab4:4 F4:2 Eb4:3 Db4:2").unwrap();
        let transformed = transpose_with_spelling(&motif, 5, SpellingPolicy::Sharps);

        assert_eq!(format_motif(&transformed), "C#5:4 A#4:2 G#4:3 F#4:2");
    }

    #[test]
    fn retrograde_preserves_total_duration() {
        let motif = parse_motif("C4:1 D4:1 G4:2").unwrap();
        let transformed = retrograde(&motif);

        assert_eq!(transformed.total_duration(), motif.total_duration());
        assert_eq!(transformed.notes[0].pitch, motif.notes[2].pitch);
    }

    #[test]
    fn retrograde_preserves_original_pitch_spelling() {
        let motif = parse_motif("Ab4:4 F4:2 Eb4:3 Db4:2").unwrap();
        let transformed = retrograde(&motif);

        assert_eq!(format_motif(&transformed), "Db4:2 Eb4:3 F4:2 Ab4:4");
    }

    #[test]
    fn inversion_maps_pitch_distances_symmetrically_around_axis() {
        let motif = parse_motif("C4:1 D4:1 G4:1").unwrap();
        let axis = "C4".parse().unwrap();
        let transformed = invert(&motif, axis);

        for (original, inverted) in motif.notes.iter().zip(transformed.notes.iter()) {
            assert_eq!(axis.0 - original.pitch.0, inverted.pitch.0 - axis.0);
        }
    }

    #[test]
    fn inversion_uses_requested_spelling_policy() {
        let motif = parse_motif("Ab4:1 C4:1").unwrap();
        let axis = "F4".parse().unwrap();
        let transformed = invert_with_spelling(&motif, axis, SpellingPolicy::Flats);

        assert_eq!(format_motif(&transformed), "D4:1 Bb4:1");
    }

    #[test]
    fn augmentation_multiplies_durations_and_total_length() {
        let motif = parse_motif("C4:1 D4:0.5 G4:2").unwrap();
        let transformed = augment(&motif, 2.0);

        assert_eq!(transformed.notes[0].duration, Beats(2.0));
        assert_eq!(transformed.notes[1].duration, Beats(1.0));
        assert_eq!(transformed.notes[2].duration, Beats(4.0));
        assert_eq!(transformed.total_duration(), Beats(motif.total_duration().0 * 2.0));
    }
}
