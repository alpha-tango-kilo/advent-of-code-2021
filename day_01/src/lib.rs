use std::fs;

pub fn input_vec_u16() -> Vec<u16> {
    fs::read_to_string("day_01/input")
        .expect("Need to provide input file")
        .lines()
        .map(|line| line.parse::<u16>().expect("Bad input"))
        .collect()
}

pub fn count_greater_than_prev(list: &[u16]) -> usize {
    list.iter()
        .skip(1)
        .zip(list.iter())
        .map(|(current, prev)| (current > prev) as usize)
        .sum::<usize>()
}
