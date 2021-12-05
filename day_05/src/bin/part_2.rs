use day_05::*;

fn main() {
    let input = input_line_vec();
    let mut grid = get_empty_grid(&input);
    input
        .into_iter()
        .flat_map(|line| line.points_between().into_iter())
        .for_each(|point| {
            // Array indexes by y, then x
            grid[(point.y as usize, point.x as usize)] += 1;
        });
    let points_over_two = grid.iter().filter(|cell| **cell >= 2).count();
    println!("Number of dangerous points: {}", points_over_two);
}
