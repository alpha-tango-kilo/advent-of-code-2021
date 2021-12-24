use day_18::input_snailfish_pairs;
use std::ops::Add;

fn main() {
    let pair = input_snailfish_pairs()
        .into_iter()
        .reduce(Add::add)
        .expect("Bad input");
    println!("The magnitude of the final sum is {}", pair.magnitude());
}
