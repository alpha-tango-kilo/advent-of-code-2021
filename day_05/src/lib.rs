use ndarray::Array2;
use std::cmp::max;
use std::error::Error;
use std::str::FromStr;
use std::{fmt, fs, iter};

pub fn input_line_vec() -> Vec<Line> {
    fs::read_to_string("day_05/input")
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let mut co_ords = line.split(" -> ");
            let left = Vec2::from_str(co_ords.next().expect("Bad input"))
                .expect("Bad co-ord");
            let right = Vec2::from_str(co_ords.next().expect("Bad input"))
                .expect("Bad co-ord");
            Line(left, right)
        })
        .collect()
}

pub fn get_empty_grid(lines: &[Line]) -> Array2<u32> {
    let max = lines
        .iter()
        .map(Line::max)
        .fold(Vec2::default(), |acc, point| acc.max(point));
    Array2::zeros((max.x as usize + 1, max.y as usize + 1))
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    fn max(self, rhs: Self) -> Self {
        Vec2 {
            x: max(self.x, rhs.x),
            y: max(self.y, rhs.y),
        }
    }
}

impl FromStr for Vec2 {
    type Err = Box<dyn Error>;

    // e.g. "1,3" or "9,0"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().ok_or("No first part")?.parse()?;
        let y = split.next().ok_or("No second part")?.parse()?;
        Ok(Vec2 { x, y })
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Line(Vec2, Vec2);

impl Line {
    pub fn horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    pub fn vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    pub fn points_between(&self) -> impl Iterator<Item = Vec2> {
        let movement = self.traversal_vec2();
        let steps = self.steps_between();

        iter::repeat(movement)
            .take(steps)
            .enumerate()
            .zip(iter::repeat(self.0))
            .map(|((co_eff, vec), start)| {
                // Add and Mul aren't implemented so I have to do this by hand
                let co_eff = co_eff as isize;
                Vec2 {
                    x: start.x + co_eff * vec.x,
                    y: start.y + co_eff * vec.y,
                }
            })
    }

    fn traversal_vec2(&self) -> Vec2 {
        use std::cmp::Ordering::*;
        let x = match self.0.x.cmp(&self.1.x) {
            Greater => -1,
            Equal => 0,
            Less => 1,
        };
        let y = match self.0.y.cmp(&self.1.y) {
            Greater => -1,
            Equal => 0,
            Less => 1,
        };
        Vec2 { x, y }
    }

    fn steps_between(&self) -> usize {
        let x_diff = (self.0.x - self.1.x).abs() as usize + 1;
        let y_diff = (self.0.y - self.1.y).abs() as usize + 1;
        if x_diff == 1 {
            y_diff
        } else if y_diff == 1 {
            x_diff
        } else {
            assert_eq!(x_diff, y_diff);
            x_diff
        }
    }

    fn max(&self) -> Vec2 {
        self.0.max(self.1)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}
