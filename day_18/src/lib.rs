use std::ops::Add;
use std::{fmt, fs};
use Snailfish::*;

pub fn input_snailfish_pairs() -> Vec<SnailfishPair> {
    let input =
        fs::read_to_string("day_18/input").expect("Failed to read input file");
    input.lines().map(string_snailfish_pair).collect()
}

fn string_snailfish_pair(s: &str) -> SnailfishPair {
    debug_assert!(
        s.starts_with('[') && s.ends_with(']'),
        "Bad input/recursion"
    );
    let (left, right) = top_level_split(&s[1..s.len() - 1]);
    let left = match left.parse::<u64>() {
        Ok(n) => Absolute(n),
        Err(_) => Nested(Box::new(string_snailfish_pair(left))),
    };
    let right = match right.parse::<u64>() {
        Ok(n) => Absolute(n),
        Err(_) => Nested(Box::new(string_snailfish_pair(right))),
    };
    SnailfishPair(left, right)
}

/// Finds the top level comma splitting the current pair of snailfish and
/// returns the string either side of it for recursive parsing
///
/// `s` must not have outermost brackets e.g. "[1, 2]", instead use "1, 2"
fn top_level_split(s: &str) -> (&str, &str) {
    let mut depth = 0;
    for (index, char) in s.chars().enumerate() {
        match char {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => return (&s[..index], &s[index + 1..]),
            _ => {}
        }
    }
    unreachable!("Couldn't find top level comma")
}

#[derive(Debug, Eq, PartialEq)]
pub struct SnailfishPair(Snailfish, Snailfish);

impl SnailfishPair {
    // Returns true if it changed anything
    fn explode(&mut self) -> bool {
        Self::_explode(self, None, None, 0)
    }

    fn _explode<'a>(
        parent_pair: &'a mut SnailfishPair,
        mut left_absolute: Option<&'a mut u64>,
        mut right_absolute: Option<&'a mut u64>,
        depth: usize,
    ) -> bool {
        // We have to check at depth 3 (instead of 4) so we have access to the
        // parent of the node we want to explode
        if depth == 3 {
            println!("\nExplosion depth reached");
            match parent_pair {
                SnailfishPair(Nested(a), Nested(b)) => {
                    println!("Both nested");
                    if let Some(left) = left_absolute {
                        *left += a.0.get_absolute().unwrap();
                    }
                    let right = b.1.get_mut_absolute().unwrap();
                    *right += a.1.get_absolute().unwrap();
                    parent_pair.0 = Absolute(0);
                    true
                }
                SnailfishPair(Absolute(left), Nested(pair)) => {
                    println!("Only right nested");
                    *left += pair.0.get_absolute().unwrap();
                    if let Some(right) = right_absolute {
                        *right += pair.1.get_absolute().unwrap();
                    }
                    parent_pair.1 = Absolute(0);
                    true
                }
                SnailfishPair(Nested(pair), Absolute(right)) => {
                    println!("Only left nested");
                    if let Some(left) = left_absolute {
                        *left += pair.0.get_absolute().unwrap();
                    }
                    *right += pair.1.get_absolute().unwrap();
                    parent_pair.0 = Absolute(0);
                    true
                }
                _ => false,
            }
        } else {
            println!("\nExplosion depth not yet reached");
            match parent_pair {
                SnailfishPair(Nested(a), Nested(b)) => {
                    println!("Both nested");
                    Self::_explode(a, None, None, depth + 1)
                        || Self::_explode(b, None, None, depth + 1)
                }
                SnailfishPair(Absolute(n), Nested(pair)) => {
                    println!("Only right nested");
                    left_absolute.insert(n);
                    Self::_explode(
                        pair,
                        left_absolute,
                        right_absolute,
                        depth + 1,
                    )
                }
                SnailfishPair(Nested(pair), Absolute(n)) => {
                    println!("Only left nested");
                    right_absolute.insert(n);
                    Self::_explode(
                        pair,
                        left_absolute,
                        right_absolute,
                        depth + 1,
                    )
                }
                _ => false,
            }
        }
    }

    fn left_most_absolute_ref(
        parent_pair: &mut SnailfishPair,
    ) -> Option<&mut u64> {
        match parent_pair {
            SnailfishPair(Absolute(n), _) | SnailfishPair(_, Absolute(n)) => {
                Some(n)
            }
            SnailfishPair(Nested(a), Nested(b)) => {
                Self::left_most_absolute_ref(a)
                    .or_else(|| Self::left_most_absolute_ref(b))
            }
        }
    }

    fn right_most_absolute_ref(
        parent_pair: &mut SnailfishPair,
    ) -> Option<&mut u64> {
        match parent_pair {
            SnailfishPair(_, Absolute(n)) | SnailfishPair(Absolute(n), _) => {
                Some(n)
            }
            SnailfishPair(Nested(a), Nested(b)) => {
                Self::right_most_absolute_ref(b)
                    .or_else(|| Self::right_most_absolute_ref(a))
            }
        }
    }

    // Returns true if it changed anything
    fn split(&mut self) -> bool {
        todo!()
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    pub fn magnitude(&self) -> u64 {
        let left = match &self.0 {
            Absolute(n) => 3 * n,
            Nested(pair) => 3 * pair.magnitude(),
        };
        let right = match &self.1 {
            Absolute(n) => 2 * n,
            Nested(pair) => 2 * pair.magnitude(),
        };
        left + right
    }
}

impl Add for SnailfishPair {
    type Output = SnailfishPair;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new =
            SnailfishPair(Nested(Box::new(self)), Nested(Box::new(rhs)));
        new.reduce();
        new
    }
}

impl fmt::Display for SnailfishPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Snailfish {
    Absolute(u64),
    Nested(Box<SnailfishPair>),
}

impl Snailfish {
    fn split(&mut self) {
        let n = match self {
            Absolute(n) => *n,
            _ => panic!("Split should only be called on Snailfish::Absolute"),
        };
        let left = n / 2;
        let right = left + n % 2;
        *self =
            Nested(Box::new(SnailfishPair(Absolute(left), Absolute(right))));
    }

    fn get_mut_nested(&mut self) -> Option<&mut SnailfishPair> {
        match self {
            Nested(pair) => Some(pair),
            Absolute(_) => None,
        }
    }

    fn get_absolute(&self) -> Option<u64> {
        match self {
            Absolute(n) => Some(*n),
            Nested(_) => None,
        }
    }

    fn get_mut_absolute(&mut self) -> Option<&mut u64> {
        match self {
            Absolute(n) => Some(n),
            Nested(_) => None,
        }
    }
}

impl fmt::Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Absolute(n) => write!(f, "{}", n),
            Nested(pair) => write!(f, "{}", pair),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        let input = "[[1,9],[8,5]]";
        let actual = string_snailfish_pair(input);
        assert_eq!(
            actual,
            SnailfishPair(
                Nested(SnailfishPair(Absolute(1), Absolute(9)).into()),
                Nested(SnailfishPair(Absolute(8), Absolute(5)).into()),
            ),
        );
    }

    #[test]
    fn magnitood() {
        let input = "[[1,2],[[3,4],5]]";
        let pair = string_snailfish_pair(input);
        assert_eq!(pair.magnitude(), 143);
    }

    #[test]
    fn explosions() {
        /*
        let input = "[[[[[9,8],1],2],3],4]";
        let mut pair = string_snailfish_pair(input);
        pair.explode();
        let expected = string_snailfish_pair("[[[[0,9],2],3],4]");
        assert_eq!(pair, expected);
         */

        let mut pair =
            string_snailfish_pair("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        println!("{:#?}", pair);
        pair.explode();
        let expected =
            string_snailfish_pair("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");

        println!("Start: [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]\nExpected: [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]\nActual: {}", pair);
        assert_eq!(pair, expected);
    }
}
