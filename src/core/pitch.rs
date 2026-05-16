use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pitch(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PitchSpelling {
    pub letter: PitchLetter,
    pub accidental: Accidental,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PitchLetter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Accidental {
    Natural,
    Sharp,
    Flat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellingPolicy {
    PreserveContext,
    Flats,
    Sharps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpellingStyle {
    Default,
    Flats,
    Sharps,
}

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

    pub fn spelling(
        self,
        policy: SpellingPolicy,
        context: &[Option<PitchSpelling>],
    ) -> PitchSpelling {
        let style = match policy {
            SpellingPolicy::PreserveContext => infer_spelling_style(context),
            SpellingPolicy::Flats => SpellingStyle::Flats,
            SpellingPolicy::Sharps => SpellingStyle::Sharps,
        };

        PitchSpelling::for_pitch_class(self.0.rem_euclid(12), style)
    }
}

impl FromStr for Pitch {
    type Err = PitchParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_pitch_with_spelling(input).map(|(pitch, _)| pitch)
    }
}

pub fn parse_pitch_with_spelling(
    input: &str,
) -> Result<(Pitch, PitchSpelling), PitchParseError> {
    if input.is_empty() {
        return Err(PitchParseError::Empty);
    }

    let mut chars = input.char_indices();
    let (_, letter) = chars.next().ok_or(PitchParseError::Empty)?;

    let letter = match letter {
        'C' => PitchLetter::C,
        'D' => PitchLetter::D,
        'E' => PitchLetter::E,
        'F' => PitchLetter::F,
        'G' => PitchLetter::G,
        'A' => PitchLetter::A,
        'B' => PitchLetter::B,
        _ => return Err(PitchParseError::InvalidNoteName(input.to_string())),
    };

    let mut accidental = 0;
    let mut spelling_accidental = Accidental::Natural;
    let octave_start = match chars.next() {
        Some((idx, '#')) => {
            accidental = 1;
            spelling_accidental = Accidental::Sharp;
            idx + '#'.len_utf8()
        }
        Some((idx, 'b')) => {
            accidental = -1;
            spelling_accidental = Accidental::Flat;
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

    Ok((
        Pitch((octave + 1) * 12 + letter.semitone_offset() + accidental),
        PitchSpelling {
            letter,
            accidental: spelling_accidental,
        },
    ))
}

impl PitchSpelling {
    fn for_pitch_class(pitch_class: i32, style: SpellingStyle) -> Self {
        match style {
            SpellingStyle::Default => default_spelling(pitch_class),
            SpellingStyle::Flats => flat_spelling(pitch_class),
            SpellingStyle::Sharps => sharp_spelling(pitch_class),
        }
    }

    pub fn format_pitch(self, pitch: Pitch) -> String {
        let octave = (pitch.0 - self.semitone_offset()).div_euclid(12) - 1;
        format!("{}{}", self, octave)
    }

    fn semitone_offset(self) -> i32 {
        self.letter.semitone_offset()
            + match self.accidental {
                Accidental::Natural => 0,
                Accidental::Sharp => 1,
                Accidental::Flat => -1,
            }
    }
}

impl PitchLetter {
    fn semitone_offset(self) -> i32 {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        }
    }
}

impl fmt::Display for PitchSpelling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter = match self.letter {
            PitchLetter::C => "C",
            PitchLetter::D => "D",
            PitchLetter::E => "E",
            PitchLetter::F => "F",
            PitchLetter::G => "G",
            PitchLetter::A => "A",
            PitchLetter::B => "B",
        };
        let accidental = match self.accidental {
            Accidental::Natural => "",
            Accidental::Sharp => "#",
            Accidental::Flat => "b",
        };

        write!(f, "{letter}{accidental}")
    }
}

fn infer_spelling_style(context: &[Option<PitchSpelling>]) -> SpellingStyle {
    let (flats, sharps) = context.iter().flatten().fold((0, 0), |(flats, sharps), spelling| {
        match spelling.accidental {
            Accidental::Flat => (flats + 1, sharps),
            Accidental::Sharp => (flats, sharps + 1),
            Accidental::Natural => (flats, sharps),
        }
    });

    match flats.cmp(&sharps) {
        std::cmp::Ordering::Greater => SpellingStyle::Flats,
        std::cmp::Ordering::Less => SpellingStyle::Sharps,
        std::cmp::Ordering::Equal => SpellingStyle::Default,
    }
}

fn default_spelling(pitch_class: i32) -> PitchSpelling {
    match pitch_class {
        0 => natural(PitchLetter::C),
        1 => sharp(PitchLetter::C),
        2 => natural(PitchLetter::D),
        3 => flat(PitchLetter::E),
        4 => natural(PitchLetter::E),
        5 => natural(PitchLetter::F),
        6 => sharp(PitchLetter::F),
        7 => natural(PitchLetter::G),
        8 => flat(PitchLetter::A),
        9 => natural(PitchLetter::A),
        10 => flat(PitchLetter::B),
        11 => natural(PitchLetter::B),
        _ => unreachable!("pitch class is normalized with rem_euclid"),
    }
}

fn flat_spelling(pitch_class: i32) -> PitchSpelling {
    match pitch_class {
        0 => natural(PitchLetter::C),
        1 => flat(PitchLetter::D),
        2 => natural(PitchLetter::D),
        3 => flat(PitchLetter::E),
        4 => natural(PitchLetter::E),
        5 => natural(PitchLetter::F),
        6 => flat(PitchLetter::G),
        7 => natural(PitchLetter::G),
        8 => flat(PitchLetter::A),
        9 => natural(PitchLetter::A),
        10 => flat(PitchLetter::B),
        11 => natural(PitchLetter::B),
        _ => unreachable!("pitch class is normalized with rem_euclid"),
    }
}

fn sharp_spelling(pitch_class: i32) -> PitchSpelling {
    match pitch_class {
        0 => natural(PitchLetter::C),
        1 => sharp(PitchLetter::C),
        2 => natural(PitchLetter::D),
        3 => sharp(PitchLetter::D),
        4 => natural(PitchLetter::E),
        5 => natural(PitchLetter::F),
        6 => sharp(PitchLetter::F),
        7 => natural(PitchLetter::G),
        8 => sharp(PitchLetter::G),
        9 => natural(PitchLetter::A),
        10 => sharp(PitchLetter::A),
        11 => natural(PitchLetter::B),
        _ => unreachable!("pitch class is normalized with rem_euclid"),
    }
}

fn natural(letter: PitchLetter) -> PitchSpelling {
    PitchSpelling {
        letter,
        accidental: Accidental::Natural,
    }
}

fn sharp(letter: PitchLetter) -> PitchSpelling {
    PitchSpelling {
        letter,
        accidental: Accidental::Sharp,
    }
}

fn flat(letter: PitchLetter) -> PitchSpelling {
    PitchSpelling {
        letter,
        accidental: Accidental::Flat,
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            default_spelling(self.0.rem_euclid(12)).format_pitch(*self)
        )
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

    #[test]
    fn parses_pitch_spelling_metadata() {
        let (pitch, spelling) = parse_pitch_with_spelling("Db5").unwrap();

        assert_eq!(pitch, Pitch(73));
        assert_eq!(
            spelling,
            PitchSpelling {
                letter: PitchLetter::D,
                accidental: Accidental::Flat,
            }
        );
    }

    #[test]
    fn formats_preserved_spelling_with_written_octave() {
        let (pitch, spelling) = parse_pitch_with_spelling("Cb4").unwrap();

        assert_eq!(pitch, Pitch(59));
        assert_eq!(spelling.format_pitch(pitch), "Cb4");
    }
}
