use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

pub fn input_target() -> Target {
    let input =
        fs::read_to_string("day_17/input").expect("Failed to read input file");
    string_target(&input)
}

fn string_target(s: &str) -> Target {
    let (x_range, y_range) = s
        .trim_end()
        .strip_prefix("target area: x=")
        .expect("Bad input")
        .split_once(", y=")
        .expect("Bad input");
    let (x_low, x_high) = x_range.split_once("..").expect("Bad input");
    let x_low = x_low.parse().expect("Bad input");
    let x_high = x_high.parse().expect("Bad input");
    let (y_low, y_high) = y_range.split_once("..").expect("Bad input");
    let y_low = y_low.parse().expect("Bad input");
    let y_high = y_high.parse().expect("Bad input");
    Target {
        x: x_low..=x_high,
        y: y_low..=y_high,
    }
}

#[derive(Debug)]
pub struct Target {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

// Number of steps required to reach target with corresponding initial
// horizontal velocity
pub fn possible_xs(target: &Target) -> HashMap<usize, i32> {
    (0..*target.x.end())
        .flat_map(|x_start_velocity| {
            can_ye_make_it(&target.x, x_start_velocity)
        })
        .collect()
}

pub fn possible_ys(
    target: &Target,
    x_step_map: HashMap<usize, i32>,
) -> Vec<(i32, i32)> {
    x_step_map
        .into_iter()
        .flat_map(|(steps, x_start_velocity)| {
            println!();
            target
                .y
                .clone()
                .into_iter()
                .map(move |dy| {
                    println!(
                        "dy: {}, steps: {}, start_velocity: {}",
                        dy,
                        steps,
                        (2 + dy) / steps as i32
                    );
                    (2 + dy) / steps as i32
                })
                //.filter(|y_start_velocity| *y_start_velocity > 0)
                .map(move |y_start_velocity| {
                    (x_start_velocity, y_start_velocity)
                })
        })
        .collect()
}

fn can_ye_make_it(
    target: &RangeInclusive<i32>,
    start_velocity: i32,
) -> impl Iterator<Item = (usize, i32)> {
    let mut pos = 0;
    let mut steps = 1;
    let mut vec = Vec::new();
    let mut started = false;
    for x_velocity in (1..=start_velocity).into_iter().rev() {
        pos += x_velocity;
        if target.contains(&pos) {
            vec.push((steps, start_velocity));
            started = true;
        } else if started {
            break;
        }
        steps += 1;
    }
    vec.into_iter()
}

pub fn triangle(n: i32) -> i32 {
    assert!(n > 0);
    n * (n + 1) / 2
}
