use day_17::{input_target, possible_xs, possible_ys, triangle};

fn main() {
    let target = input_target();
    let xs = possible_xs(&target);
    println!("Number of steps required to reach target with corresponding initial horizontal velocity: {:?}", xs);
    let answers = possible_ys(&target, xs);
    println!("Initial velocities that'd reach the target: {:?}", &answers);
    let goes_highest = *answers
        .iter()
        .max_by(|(_, y_a), (_, y_b)| y_a.cmp(y_b))
        .unwrap();
    println!(
        "The initial velocity that goes highest is: {:?}",
        goes_highest
    );
    println!(
        "The highest point it reaches is: {}",
        triangle(goes_highest.1)
    );
    // 3570, too low
    // 3916, too high
}
