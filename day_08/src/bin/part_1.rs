use day_08::SEGMENTS_NEEDED;
use std::fs;

fn main() {
    let answer = fs::read_to_string("day_08/input")
        .expect("Failed to read input file")
        .lines()
        .flat_map(|line| {
            line.split(" | ")
                .nth(1)
                .expect("Bad input")
                .split_ascii_whitespace()
        })
        .filter(|str| {
            let len = str.len() as u8;
            len == SEGMENTS_NEEDED[1]
                || len == SEGMENTS_NEEDED[4]
                || len == SEGMENTS_NEEDED[7]
                || len == SEGMENTS_NEEDED[8]
        })
        .count();
    println!(
        "Number of times the digits 1, 4, 7, or 8 appear: {}",
        answer
    );
}
