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
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
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


    pub fn expected(self, outcome: Outcome) -> Move {
        match outcome {
            Outcome::Draw => self,
            Outcome::Win => {
                match self {
                    Self::Rock => Self::Paper,
                    Self::Paper => Self::Scissors,
                    Self::Scissors => Self::Rock,
                }
            }
            Outcome::Loose => {
                match self {
                    Self::Rock => Self::Scissors,
                    Self::Paper => Self::Rock,
                    Self::Scissors => Self::Paper,
                }
            }
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
