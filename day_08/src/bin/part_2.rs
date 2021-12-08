#![allow(
    clippy::borrow_interior_mutable_const,
    clippy::declare_interior_mutable_const
)]

use day_08::SEGMENTS_NEEDED;
use once_cell::unsync::Lazy;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use Segment::*;

type PossibleMapping = HashMap<char, HashSet<Segment>>;

fn main() {
    let mut panels = fs::read_to_string("day_08/input")
        .expect("Unable to read input file")
        .lines()
        .map(|line| Panel::from_str(line).expect("Bad input"))
        .collect::<Vec<_>>();
    let sum = panels
        .iter_mut()
        .map(|panel| {
            panel.unique_digit_check();
            panel.five_letter_check();
            panel.six_letter_check();
            panel.deduce();
            panel.translate_right_digits()
        })
        .sum::<u32>();
    println!("The sum of all the displayed numbers is: {}", sum);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Segment {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

impl Segment {
    pub const ALL_SEGMENTS: &'static [Segment] = &[
        Top,
        TopLeft,
        TopRight,
        Middle,
        BottomLeft,
        BottomRight,
        Bottom,
    ];
    pub const FORMATIONS: [&'static [Segment]; 10] = [
        &[Top, TopLeft, TopRight, BottomLeft, BottomRight, Bottom],
        &[TopRight, BottomRight],
        &[Top, TopRight, Middle, BottomLeft, Bottom],
        &[Top, TopRight, Middle, BottomRight, Bottom],
        &[TopLeft, TopRight, Middle, BottomRight],
        &[Top, TopLeft, Middle, BottomRight, Bottom],
        &[Top, TopLeft, Middle, BottomLeft, BottomRight, Bottom],
        &[Top, TopRight, BottomRight],
        Segment::ALL_SEGMENTS,
        &[Top, TopLeft, TopRight, Middle, BottomRight, Bottom],
    ];
    // Can't use sets, so use &[Segment] but ordered
    pub const TRANSLATE_MAP: Lazy<HashMap<&'static [Segment], char>> =
        Lazy::new(|| {
            let mut map = HashMap::with_capacity(10);
            map.insert(Self::FORMATIONS[0], '0');
            map.insert(Self::FORMATIONS[1], '1');
            map.insert(Self::FORMATIONS[2], '2');
            map.insert(Self::FORMATIONS[3], '3');
            map.insert(Self::FORMATIONS[4], '4');
            map.insert(Self::FORMATIONS[5], '5');
            map.insert(Self::FORMATIONS[6], '6');
            map.insert(Self::FORMATIONS[7], '7');
            map.insert(Self::FORMATIONS[8], '8');
            map.insert(Self::FORMATIONS[9], '9');
            map
        });

    fn complement(not_segs: &[Segment]) -> impl Iterator<Item = &Segment> {
        Self::ALL_SEGMENTS
            .iter()
            .filter(|seg| !not_segs.contains(seg))
    }
}

#[derive(Debug)]
struct Panel {
    left_digits: [HashSet<char>; 10],
    right_digits: [HashSet<char>; 4],
    possible_mappings: PossibleMapping,
}

impl Panel {
    const LETTERS: &'static str = "abcdefg";
    const DEFAULT_MAPPINGS: Lazy<PossibleMapping> = Lazy::new(|| {
        let seg_set = HashSet::from_iter(Segment::ALL_SEGMENTS.iter().cloned());
        let mut map = HashMap::with_capacity(10);
        Self::LETTERS.chars().for_each(|c| {
            map.insert(c, seg_set.clone());
        });
        map
    });

    fn unique_digit_check(&mut self) {
        self.left_digits.iter().for_each(|char_set| {
            let len = char_set.len();
            if len == SEGMENTS_NEEDED[1] {
                Self::remove_impossibles(
                    &mut self.possible_mappings,
                    char_set.iter().cloned(),
                    Segment::FORMATIONS[1],
                );
            } else if len == SEGMENTS_NEEDED[4] {
                Self::remove_impossibles(
                    &mut self.possible_mappings,
                    char_set.iter().cloned(),
                    Segment::FORMATIONS[4],
                );
            } else if len == SEGMENTS_NEEDED[7] {
                Self::remove_impossibles(
                    &mut self.possible_mappings,
                    char_set.iter().cloned(),
                    Segment::FORMATIONS[7],
                );
            }
            // No point in doing 8 segments as it's a no-op
        })
    }

    // 2, 3, and 5 have 3 segments in common
    fn five_letter_check(&mut self) {
        let five_letters = self
            .left_digits
            .iter()
            .filter(|set| set.len() == 5)
            .collect::<Vec<_>>();
        let in_common = Self::letters_in_common(&five_letters);
        Self::remove_impossibles(
            &mut self.possible_mappings,
            in_common,
            &[Top, Middle, Bottom],
        );
    }

    // 0, 6, and 9 have 4 segments in common
    fn six_letter_check(&mut self) {
        let six_letters = self
            .left_digits
            .iter()
            .filter(|set| set.len() == 6)
            .collect::<Vec<_>>();
        let in_common =
            Self::letters_in_common(&six_letters).collect::<Vec<_>>();
        Self::remove_impossibles(
            &mut self.possible_mappings,
            in_common,
            &[Top, TopLeft, BottomRight, Bottom],
        );
    }

    fn deduce(&mut self) -> bool {
        let mut changes_made = true;
        while changes_made {
            changes_made = false;
            let certain_segs = self
                .possible_mappings
                .iter()
                .map(|(_, snd)| snd)
                .filter(|set| set.len() == 1)
                .flat_map(|set| set.iter().cloned())
                .collect::<Vec<_>>(); // Single element iter
            self.possible_mappings
                .iter_mut()
                .map(|(_, snd)| snd)
                .filter(|set| set.len() > 1)
                .for_each(|set| {
                    certain_segs.iter().for_each(|seg| {
                        changes_made |= set.remove(seg);
                    });
                });
        }
        self.solved()
    }

    fn letters_in_common<'a>(
        sets: &'a [&HashSet<char>],
    ) -> impl Iterator<Item = char> + 'a {
        Self::LETTERS
            .chars()
            .filter(|c| sets.iter().all(|set| set.contains(c)))
    }

    fn remove_impossibles<I: IntoIterator<Item = char>>(
        possible_mappings: &mut PossibleMapping,
        chars: I,
        possible_segs: &[Segment],
    ) {
        chars.into_iter().for_each(|char| {
            let possibles = possible_mappings.get_mut(&char).unwrap();
            Segment::complement(possible_segs).for_each(|seg| {
                possibles.remove(seg);
            });
        });
    }

    fn solved(&self) -> bool {
        self.possible_mappings.iter().all(|(_, set)| set.len() == 1)
    }

    fn translate_right_digits(&self) -> u32 {
        assert!(self.solved(), "Can't translate before being solved");
        let num_string = self
            .right_digits
            .iter()
            .map(|char_set| {
                let mut seg_vec = char_set
                    .iter()
                    .map(|char| {
                        *self
                            .possible_mappings
                            .get(char)
                            .unwrap()
                            .iter()
                            .next()
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                // Has to be sorted as TRANSLATE_MAP needs the &[Segment] to be
                // sorted
                seg_vec.sort();
                seg_vec
            })
            .map(|seg_vec| {
                *Segment::TRANSLATE_MAP.get(seg_vec.as_slice()).unwrap()
            })
            .collect::<String>();
        num_string.parse().unwrap()
    }
}

impl FromStr for Panel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pipeless = s.replacen('|', "", 1);
        let mut digits = pipeless
            .split_ascii_whitespace()
            .map(|digit_str| digit_str.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        // Cludgy hack to get Vec into array without cloning
        let left_digits = [
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
        ];
        let right_digits = [
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
            digits.remove(0),
        ];
        Ok(Panel {
            left_digits,
            right_digits,
            possible_mappings: Panel::DEFAULT_MAPPINGS.clone(),
        })
    }
}
