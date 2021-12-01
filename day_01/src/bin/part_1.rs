use day_01::{count_greater_than_prev, input_vec_u16};

fn main() {
    let input = input_vec_u16();
    let answer = count_greater_than_prev(&input);

    println!("There are {} increases in depth", answer);
}
