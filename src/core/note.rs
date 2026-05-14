use super::{Beats, Pitch};

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub pitch: Pitch,
    pub start: Beats,
    pub duration: Beats,
    pub velocity: u8,
}

impl Note {
    pub fn new(pitch: Pitch, start: Beats, duration: Beats) -> Self {
        Self {
            pitch,
            start,
            duration,
            velocity: 100,
        }
    }
}

