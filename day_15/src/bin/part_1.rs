use day_15::input_chiton_grid;

fn main() {
    let mut grid = input_chiton_grid();
    let weight = grid.most_efficient_route((0, 0));
    println!("The least risky path from top left to bottom right has a total risk of {}", weight);
}
