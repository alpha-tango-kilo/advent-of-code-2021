use day_02::input_vec_direction;
use day_02::Direction::{self, *};
use std::fs;

fn main() {
    let input = input_vec_direction();

    let final_pos = input.into_iter().fold(PosXY(0, 0), |pos, dir| match dir {
        Forward(distance) => PosXY(pos.0 + distance as u16, pos.1),
        Up(distance) => PosXY(pos.0, pos.1 - distance as u16),
        Down(distance) => PosXY(pos.0, pos.1 + distance as u16),
    });

    println!(
        "Final position:\n - Horizontal: {}\n - Depth: {}",
        final_pos.0, final_pos.1
    );
    println!(
        "Product (answer): {}",
        final_pos.0 as u32 * final_pos.1 as u32
    );
}

struct PosXY(u16, u16);
