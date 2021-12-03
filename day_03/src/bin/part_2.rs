use day_03::*;
use ndarray::Array2;

fn main() {
    let matrix = input_matrix();
    let oxygen_rating_vec = get_rating(&matrix, false);
    let co2_scrubber_rating_vec = get_rating(&matrix, true);
    let oxygen_rating = bool_slice_to_int(&oxygen_rating_vec);
    let co2_scrubber_rating = bool_slice_to_int(&co2_scrubber_rating_vec);
    println!("Oxygen rating: {}", oxygen_rating);
    println!("CO2 rating: {}", co2_scrubber_rating);
    println!("Product (answer): {}", oxygen_rating * co2_scrubber_rating);
}

fn get_rating(matrix: &Array2<bool>, invert: bool) -> Vec<bool> {
    let possibles = matrix.rows().into_iter().map(|row| row.to_vec()).collect();
    filter_recursive(possibles, 0, invert)
}

fn filter_recursive(
    mut possibles: Vec<Vec<bool>>,
    col_index: usize,
    invert: bool,
) -> Vec<bool> {
    if possibles.len() == 1 {
        return possibles.pop().unwrap();
    }
    assert!(!possibles.is_empty(), "No solution found");
    assert!(col_index < possibles[0].len(), "No solution found");

    let mut trues = Vec::new();
    let mut falses = Vec::new();
    possibles.into_iter().for_each(|row| match row[col_index] {
        true => trues.push(row),
        false => falses.push(row),
    });

    use std::cmp::Ordering::*;
    if !invert {
        match trues.len().cmp(&falses.len()) {
            Greater | Equal => filter_recursive(trues, col_index + 1, invert),
            Less => filter_recursive(falses, col_index + 1, invert),
        }
    } else {
        match trues.len().cmp(&falses.len()) {
            Greater | Equal => filter_recursive(falses, col_index + 1, invert),
            Less => filter_recursive(trues, col_index + 1, invert),
        }
    }
}
