use super::{Beats, Note, Pitch};

#[derive(Debug, Clone, PartialEq)]
pub struct Motif {
    pub notes: Vec<Note>,
}

impl Motif {
    pub fn new(notes: Vec<Note>) -> Self {
        Self { notes }
    }

    pub fn note_count(&self) -> usize {
        self.notes.len()
    }

    pub fn total_duration(&self) -> Beats {
        self.notes
            .iter()
            .map(|note| note.start.0 + note.duration.0)
            .fold(0.0, f32::max)
            .into()
    }

    pub fn pitch_range(&self) -> Option<(Pitch, Pitch)> {
        let min = self.notes.iter().map(|note| note.pitch).min()?;
        let max = self.notes.iter().map(|note| note.pitch).max()?;
        Some((min, max))
    }
}

impl From<f32> for Beats {
    fn from(value: f32) -> Self {
        Beats(value)
    }
}

