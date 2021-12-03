use day_03::{bool_slice_to_int, input_matrix, most_common_by_col};

fn main() {
    let matrix = input_matrix();
    let most_common_by_col = most_common_by_col(&matrix);
    let gamma_rate = bool_slice_to_int(&most_common_by_col);
    let epsilon_rate = bool_slice_to_int(&not_bool_slice(&most_common_by_col));
    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Product (answer): {}", gamma_rate * epsilon_rate);
}

fn not_bool_slice(bools: &[bool]) -> Vec<bool> {
    bools.iter().map(|b| !*b).collect()
}
