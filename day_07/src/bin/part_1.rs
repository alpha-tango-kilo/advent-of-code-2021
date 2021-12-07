use day_07::*;
use std::collections::HashMap;

fn main() {
    let freq_map = input_hash_map();
    let (pos, fuel_use) = least_fuel_use(&freq_map, fuel_use_to);
    println!(
        "The cheapest position to get to is {}, costing {} fuel",
        pos, fuel_use
    );
}

fn fuel_use_to(map: &HashMap<u16, u16>, target_pos: u16) -> u32 {
    map.iter()
        .map(|(pos, count)| {
            let diff = (*pos as i32 - target_pos as i32).abs() as u32;
            diff * *count as u32
        })
        .sum()
}
