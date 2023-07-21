#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Win,
    Loose,
    Draw
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
