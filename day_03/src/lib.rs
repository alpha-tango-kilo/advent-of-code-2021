use ndarray::Array2;
use std::fs;

pub fn input_matrix() -> Array2<bool> {
    let input =
        fs::read_to_string("day_03/input").expect("Failed to read input file");

    let rows = input.lines().count();
    let cols = input.split('\n').next().expect("Bad input").len();
    let mut matrix = Array2::<bool>::default((rows, cols));

    input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '0' => false,
                '1' => true,
                _ => unreachable!(),
            })
        })
        .zip(matrix.iter_mut())
        .for_each(|(val, cell)| *cell = val);

    matrix
}

pub fn bool_slice_to_int(bools: &[bool]) -> usize {
    bools
        .iter()
        .rev()
        .enumerate()
        .map(|(pow2, val)| (pow2 as u32, *val as usize))
        .fold(0, |acc, (pow2, val)| acc + val * 2usize.pow(pow2))
}
