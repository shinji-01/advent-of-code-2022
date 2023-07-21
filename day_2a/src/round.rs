use std::str::FromStr;
use std::io;

use crate::r_move;
use crate::outcome;

#[derive(Debug)]
pub struct Round {
    theirs: r_move::Move,
    ours: r_move::Move,
}

impl Round {
    fn outcome(self) -> outcome::Outcome {
        self.ours.outcome(self.theirs)
    }

    pub fn score(self) -> usize {
        self.ours.points() + self.outcome().points()
    }
}


impl FromStr for Round {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(io::Error::new(io::ErrorKind::Other, "Error while parsing round".to_string()));
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}
