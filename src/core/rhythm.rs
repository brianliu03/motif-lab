use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Beats(pub f32);

impl Beats {
    pub const ZERO: Beats = Beats(0.0);

    pub fn format(self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for Beats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl std::ops::Add for Beats {
    type Output = Beats;

    fn add(self, rhs: Self) -> Self::Output {
        Beats(self.0 + rhs.0)
    }
}
