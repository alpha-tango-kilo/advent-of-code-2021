use day_10::*;
use std::fs;

fn main() {
    let total_score = fs::read_to_string("day_10/input")
        .expect("Failed to read input file")
        .lines()
        .filter_map(|line| {
            use ParseResult::*;
            match parse_line(line) {
                Ok | Incomplete(_) => None,
                Corrupt(n) => Some(n),
            }
        })
        .sum::<usize>();
    println!("Total syntax error score: {}", total_score);
}
