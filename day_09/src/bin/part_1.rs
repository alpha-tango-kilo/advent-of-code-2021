use day_09::input_grid;

fn main() {
    let grid = input_grid();
    let risk_sum = grid
        .get_local_minima_risk_level()
        .map(|digit| digit as u16)
        .sum::<u16>();
    println!("The sum of the low level risk points is: {}", risk_sum);
}
