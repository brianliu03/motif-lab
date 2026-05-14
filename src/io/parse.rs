use crate::core::{Beats, Motif, Note, Pitch};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    EmptyToken,
    MissingDuration(String),
    InvalidPitch(String),
    InvalidDuration(String),
    NonPositiveDuration(f32),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyToken => write!(f, "empty token"),
            ParseError::MissingDuration(token) => write!(f, "missing duration in '{token}'"),
            ParseError::InvalidPitch(pitch) => write!(f, "invalid pitch '{pitch}'"),
            ParseError::InvalidDuration(duration) => write!(f, "invalid duration '{duration}'"),
            ParseError::NonPositiveDuration(duration) => {
                write!(f, "duration must be positive, got {duration}")
            }
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_motif(input: &str) -> Result<Motif, ParseError> {
    let mut notes = Vec::new();
    let mut start = Beats::ZERO;

    for token in input.split_whitespace() {
        let note = parse_note_token(token, start)?;
        start = start + note.duration;
        notes.push(note);
    }

    Ok(Motif::new(notes))
}

fn parse_note_token(token: &str, start: Beats) -> Result<Note, ParseError> {
    if token.is_empty() {
        return Err(ParseError::EmptyToken);
    }

    let (pitch_text, duration_text) = token
        .split_once(':')
        .ok_or_else(|| ParseError::MissingDuration(token.to_string()))?;

    let pitch =
        Pitch::from_str(pitch_text).map_err(|_| ParseError::InvalidPitch(pitch_text.to_string()))?;

    let duration = duration_text
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidDuration(duration_text.to_string()))?;

    if duration <= 0.0 {
        return Err(ParseError::NonPositiveDuration(duration));
    }

    Ok(Note::new(pitch, start, Beats(duration)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_motif_and_infers_sequential_start_times() {
        let motif = parse_motif("C4:1 D4:0.5 E4:2").unwrap();

        assert_eq!(motif.notes.len(), 3);
        assert_eq!(motif.notes[0].start, Beats(0.0));
        assert_eq!(motif.notes[1].start, Beats(1.0));
        assert_eq!(motif.notes[2].start, Beats(1.5));
        assert_eq!(motif.total_duration(), Beats(3.5));
    }
}

