use std::io;

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Win,
    Loose,
    Draw
}

impl TryFrom<char> for Outcome {
    type Error = io::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Self::Loose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(io::Error::new(io::ErrorKind::Other,"Error occured".to_string()))
        }
    }
}

impl Outcome {
    pub fn points(self) -> usize {
        match self {
            Self::Win => 6,
            Self::Loose => 0,
            Self::Draw => 3,
        }
    }
}
