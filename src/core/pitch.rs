use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pitch(pub i32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PitchParseError {
    Empty,
    InvalidNoteName(String),
    MissingOctave,
    InvalidOctave(String),
}

impl fmt::Display for PitchParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PitchParseError::Empty => write!(f, "empty pitch"),
            PitchParseError::InvalidNoteName(name) => write!(f, "invalid note name '{name}'"),
            PitchParseError::MissingOctave => write!(f, "missing octave"),
            PitchParseError::InvalidOctave(octave) => write!(f, "invalid octave '{octave}'"),
        }
    }
}

impl std::error::Error for PitchParseError {}

impl Pitch {
    pub fn interval_to(self, other: Pitch) -> i32 {
        other.0 - self.0
    }
}

impl FromStr for Pitch {
    type Err = PitchParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(PitchParseError::Empty);
        }

        let mut chars = input.char_indices();
        let (_, letter) = chars.next().ok_or(PitchParseError::Empty)?;

        let base = match letter {
            'C' => 0,
            'D' => 2,
            'E' => 4,
            'F' => 5,
            'G' => 7,
            'A' => 9,
            'B' => 11,
            _ => return Err(PitchParseError::InvalidNoteName(input.to_string())),
        };

        let mut accidental = 0;
        let octave_start = match chars.next() {
            Some((idx, '#')) => {
                accidental = 1;
                idx + '#'.len_utf8()
            }
            Some((idx, 'b')) => {
                accidental = -1;
                idx + 'b'.len_utf8()
            }
            Some((idx, _)) => idx,
            None => return Err(PitchParseError::MissingOctave),
        };

        let octave_text = &input[octave_start..];
        if octave_text.is_empty() {
            return Err(PitchParseError::MissingOctave);
        }

        let octave = octave_text
            .parse::<i32>()
            .map_err(|_| PitchParseError::InvalidOctave(octave_text.to_string()))?;

        Ok(Pitch((octave + 1) * 12 + base + accidental))
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pitch_class = self.0.rem_euclid(12);
        let octave = self.0.div_euclid(12) - 1;
        let name = match pitch_class {
            0 => "C",
            1 => "C#",
            2 => "D",
            3 => "Eb",
            4 => "E",
            5 => "F",
            6 => "F#",
            7 => "G",
            8 => "Ab",
            9 => "A",
            10 => "Bb",
            11 => "B",
            _ => unreachable!("pitch class is normalized with rem_euclid"),
        };

        write!(f, "{name}{octave}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pitch_names_into_midi_numbers() {
        assert_eq!("C4".parse::<Pitch>().unwrap(), Pitch(60));
        assert_eq!("G4".parse::<Pitch>().unwrap(), Pitch(67));
        assert_eq!("C#4".parse::<Pitch>().unwrap(), Pitch(61));
        assert_eq!("Bb3".parse::<Pitch>().unwrap(), Pitch(58));
    }
}

