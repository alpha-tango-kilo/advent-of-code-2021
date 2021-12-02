use std::fs;
use std::str::FromStr;
use Direction::*;

fn main() {
    let input = fs::read_to_string("day_02/input")
        .expect("Failed to read input file")
        .lines()
        .map(Direction::from_str)
        .collect::<Result<Vec<Direction>, ()>>()
        .expect("Bad input");

    let final_pos = input.into_iter()
        .fold(PosXY(0, 0), |pos, dir| match dir {
            Forward(distance) => PosXY(pos.0 + distance as u16, pos.1),
            Up(distance) => PosXY(pos.0, pos.1 - distance as u16),
            Down(distance) => PosXY(pos.0, pos.1 + distance as u16),
        });

    println!("Final position:\n - Horizontal: {}\n - Depth: {}", final_pos.0, final_pos.1);
    println!("Product (answer): {}", final_pos.0 as u32 * final_pos.1 as u32);
}

struct PosXY(u16, u16);

enum Direction {
    Forward(u8),
    Up(u8),
    Down(u8),
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        if parts.len() == 2 {
            let operand = parts[1].parse::<u8>().map_err(|_| ())?;
            match parts[0] {
                "forward" => Ok(Forward(operand)),
                "up" => Ok(Up(operand)),
                "down" => Ok(Down(operand)),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}
