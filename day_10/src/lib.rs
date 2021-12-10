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
    fn score_part_one(&self) -> usize {
        match self {
            Round => 3,
            Square => 57,
            Curly => 1197,
            Angled => 25137,
        }
    }

    fn score_part_two(&self) -> usize {
        match self {
            Round => 1,
            Square => 2,
            Curly => 3,
            Angled => 4,
        }
    }
}

pub enum ParseResult {
    Ok,
    Corrupt(usize),
    Incomplete(usize),
}

pub fn parse_line(line: &str) -> ParseResult {
    use ParseResult::*;
    let mut stack = Vec::with_capacity(line.len() / 2);

    for Bracket(shape, open) in line
        .chars()
        .map(|c| Bracket::try_from(c).expect("Bad input"))
    {
        if open {
            stack.push(shape);
        } else if stack.pop() != Some(shape) {
            return Corrupt(shape.score_part_one());
        }
    }

    if stack.is_empty() {
        Ok
    } else {
        let total_score = stack
            .into_iter()
            .rfold(0, |acc, shape| acc * 5 + shape.score_part_two());
        Incomplete(total_score)
    }
}
