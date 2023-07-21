use std::io;

use crate::outcome::Outcome;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Move {
    type Error = io::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(io::Error::new(io::ErrorKind::Other,"Error occured".to_string()))
        }
    }
}

impl Move {
    pub fn points(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors) |
            (Self::Paper, Self::Rock) |
            (Self::Scissors, Self::Paper)
        )
    }

    pub fn outcome(self, theirs: Self) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loose
        } else {
            Outcome::Draw
        }
    }
}
