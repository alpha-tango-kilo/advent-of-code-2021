use std::fs;
use day_10::*;

fn main() {
    let total_score = fs::read_to_string("day_10/input")
        .expect("Failed to read input file")
        .lines()
        .filter_map(|line| parse_line(line).err())
        .sum::<usize>();
    println!("Total syntax error score: {}", total_score);
}

fn parse_line(line: &str) -> Result<(), usize> {
    let mut stack = Vec::with_capacity(line.len() / 2);
    line.chars()
        .map(|c| Bracket::try_from(c).expect("Bad input"))
        .try_for_each(|Bracket(shape, open)| {
            if open {
                stack.push(shape);
                Ok(())
            } else if stack.pop().ok_or_else(|| shape.get_score())?.eq(&shape) {
                Ok(())
            } else {
                Err(shape.get_score())
            }
        })
}
