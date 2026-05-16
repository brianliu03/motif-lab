use super::{Beats, Pitch, PitchSpelling};

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub pitch: Pitch,
    pub spelling: Option<PitchSpelling>,
    pub start: Beats,
    pub duration: Beats,
    pub velocity: u8,
}

impl Note {
    pub fn new(pitch: Pitch, start: Beats, duration: Beats) -> Self {
        Self {
            pitch,
            spelling: None,
            start,
            duration,
            velocity: 100,
        }
    }

    pub fn with_spelling(mut self, spelling: PitchSpelling) -> Self {
        self.spelling = Some(spelling);
        self
    }
}
