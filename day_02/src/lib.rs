use std::fs;
use std::str::FromStr;
use Direction::*;

pub fn input_vec_direction() -> Vec<Direction> {
    fs::read_to_string("day_02/input")
        .expect("Failed to read input file")
        .lines()
        .map(Direction::from_str)
        .collect::<Result<_, _>>()
        .expect("Bad input")
}

pub enum Direction {
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
