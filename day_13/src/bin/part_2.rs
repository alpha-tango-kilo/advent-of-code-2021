use day_13::{fold_all, format_dots, input_co_ords_folds};

fn main() {
    let (co_ords, folds) = input_co_ords_folds();
    let co_ords = fold_all(co_ords, &folds);
    println!("After completing all folds:\n{}", format_dots(&co_ords));
}
