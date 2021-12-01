use day_01::{count_greater_than_prev, input_vec_u16};

fn main() {
    let input = input_vec_u16();

    let tuple_sums = input
        .iter()
        .skip(2)
        .zip(input.iter().skip(1))
        .zip(input.iter())
        .map(|((c, b), a)| a + b + c)
        .collect::<Vec<_>>();

    let answer = count_greater_than_prev(&tuple_sums);

    println!("There are {} increases in depth", answer);
}
