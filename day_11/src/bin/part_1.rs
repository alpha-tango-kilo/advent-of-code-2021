use day_11::input_octopi;

const STEPS: usize = 100;

fn main() {
    let mut octopi = input_octopi();
    octopi.simulate_n(STEPS);
    println!("Flashes after {} steps: {}", STEPS, octopi.flashes());
}
