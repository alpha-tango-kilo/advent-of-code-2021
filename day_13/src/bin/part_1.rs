use day_13::{fold, input_co_ords_folds};

fn main() {
    let (co_ords, folds) = input_co_ords_folds();
    let co_ords = fold(co_ords, folds[0]);
    println!("Dots after one fold: {}", co_ords.len());
}
