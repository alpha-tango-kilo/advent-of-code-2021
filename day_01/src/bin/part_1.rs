use std::fs;

fn main() {
    let input = fs::read_to_string("day_01/input")
        .expect("Need to provide input file")
        .lines()
        .map(|line| line.parse::<u16>().expect("Bad input"))
        .collect::<Vec<_>>();

    let answer = input.iter()
        .skip(1)
        .zip(input.iter())
        .map(|(current, prev)| (current > prev) as usize)
        .sum::<usize>();

    println!("There are {} increases in depth", answer);
}
