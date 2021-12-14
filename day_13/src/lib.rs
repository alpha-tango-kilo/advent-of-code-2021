use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

pub type CoOrds = HashSet<(u32, u32)>;

pub fn input_co_ords_folds() -> (CoOrds, Vec<Fold>) {
    let input =
        fs::read_to_string("day_13/input").expect("Failed to read input file");
    string_co_ords_folds(&input)
}

fn string_co_ords_folds(s: &str) -> (CoOrds, Vec<Fold>) {
    let (co_ords, instructions) = s.split_once("\n\n").expect("Bad input");
    let co_ords = co_ords
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').expect("Bad input");
            let x = x.parse().expect("Bad input");
            let y = y.parse().expect("Bad input");
            (x, y)
        })
        .collect();
    let instructions = instructions
        .lines()
        .map(|line| Fold::from_str(line).expect("Bad input"))
        .collect();
    (co_ords, instructions)
}

pub fn fold(dots: CoOrds, Fold(axis, centre): Fold) -> CoOrds {
    dots.into_iter()
        .map(|(x, y)| match axis {
            Axis::X if x > centre => (2 * centre - x, y),
            Axis::Y if y > centre => (x, 2 * centre - y),
            _ => (x, y),
        })
        .collect()
}

pub fn fold_all(dots: CoOrds, folds: &[Fold]) -> CoOrds {
    folds.iter().fold(dots, |acc, f| fold(acc, *f))
}

pub fn format_dots(dots: &CoOrds) -> String {
    let cols = dots.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let rows = dots.iter().map(|(_, y)| *y).max().unwrap() + 1;
    let mut output = String::with_capacity((rows * cols) as usize);
    (0..rows).into_iter().for_each(|y| {
        (0..cols).into_iter().for_each(|x| {
            if dots.contains(&(x, y)) {
                output.push('█');
            } else {
                output.push(' ');
            }
        });
        output.push('\n');
    });
    output.pop(); // Remove last newline
    output
}

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Copy, Clone)]
pub struct Fold(Axis, u32);

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix("fold along ") {
            let (axis, operand) = stripped.split_once('=').ok_or(())?;
            let operand = operand.parse().map_err(|_| ())?;
            match axis {
                "x" => Ok(Fold(Axis::X, operand)),
                "y" => Ok(Fold(Axis::Y, operand)),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{fold, format_dots, string_co_ords_folds};

    const WEBSITE_EXAMPLE: &str = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5\n";

    #[test]
    fn one_fold() {
        let (co_ords, folds) = string_co_ords_folds(WEBSITE_EXAMPLE);
        let co_ords = fold(co_ords, folds[0]);
        let actual = format_dots(&co_ords);
        println!("After:\n{}", &actual);
        assert_eq!(co_ords.len(), 17, "Wrong number of dots");
    }
}
