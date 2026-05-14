use crate::core::Motif;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::parse::parse_motif;

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
}

