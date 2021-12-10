pub use BracketType::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Bracket(pub BracketType, pub bool);

impl TryFrom<char> for Bracket {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Bracket(Round, true)),
            ')' => Ok(Bracket(Round, false)),
            '[' => Ok(Bracket(Square, true)),
            ']' => Ok(Bracket(Square, false)),
            '{' => Ok(Bracket(Curly, true)),
            '}' => Ok(Bracket(Curly, false)),
            '<' => Ok(Bracket(Angled, true)),
            '>' => Ok(Bracket(Angled, false)),
            c => Err(c),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BracketType {
    Round,
    Square,
    Curly,
    Angled,
}

impl BracketType {
    pub fn get_score(&self) -> usize {
        match self {
            Round => 3,
            Square => 57,
            Curly => 1197,
            Angled => 25137,
        }
    }
}
