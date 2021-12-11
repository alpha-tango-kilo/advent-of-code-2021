use day_11::input_octopi;

fn main() {
    let mut octopi = input_octopi();
    let answer = octopi.simulate_until_epilepsy();
    println!("The first step when all octopi flash is: {}", answer);
}
