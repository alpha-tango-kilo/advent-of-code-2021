use day_13::input_dot_grid_instructions;

fn main() {
    let (mut grid, instructions) = input_dot_grid_instructions();
    instructions.into_iter().for_each(|fold| grid.fold(fold));
    println!("Dots visible after applying all the folds: {}", grid.dots());
    // 82, wrong
    // 62, wrong
    // 143, too low
}
