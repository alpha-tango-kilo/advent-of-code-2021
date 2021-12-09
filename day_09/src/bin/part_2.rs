use day_09::input_grid;

fn main() {
    let grid = input_grid();
    let mut basin_sizes = grid
        .get_basins()
        .map(|mut basin| {
            basin.propogate_all();
            basin.basin_size().unwrap()
        })
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    let answer = basin_sizes.iter().rev().take(3).product::<usize>();
    println!(
        "The product of the sizes of the three largest basins was: {}",
        answer
    );
}
