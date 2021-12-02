use day_02::input_vec_direction;
use day_02::Direction::{self, *};
use std::ops::AddAssign;

fn main() {
    let input = input_vec_direction();
    let mut submarine = SubState::default();

    input.into_iter().for_each(|dir| submarine += dir);

    println!("Submarine's final state: {:?}", submarine);
    println!(
        "Product of height & depth (answer): {}",
        submarine.horizontal as u64 * submarine.depth as u64
    );
}

#[derive(Debug, Default)]
struct SubState {
    horizontal: u16,
    depth: i32,
    aim: i16,
}

impl AddAssign<Direction> for SubState {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Forward(operand) => {
                self.horizontal += operand as u16;
                self.depth += self.aim as i32 * operand as i32;
            }
            Up(operand) => self.aim -= operand as i16,
            Down(operand) => self.aim += operand as i16,
        }
    }
}
