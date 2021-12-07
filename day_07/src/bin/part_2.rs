use day_07::*;
use std::collections::HashMap;

fn main() {
    let freq_map = input_hash_map();
    let max = *get_max_key(&freq_map);
    let (pos, fuel_use) = (0..=max)
        .into_iter()
        .map(|target_pos| (target_pos, fuel_use_to(&freq_map, target_pos)))
        .min_by(|(_, fuel_use_a), (_, fuel_use_b)| fuel_use_a.cmp(fuel_use_b))
        .unwrap();
    println!(
        "The cheapest position to get to is {}, costing {} fuel",
        pos, fuel_use
    );
}

fn fuel_use_to(map: &HashMap<u16, u16>, target_pos: u16) -> u32 {
    map.iter()
        .map(|(pos, count)| {
            let diff = (*pos as i32 - target_pos as i32).abs() as u32;
            triangle_number(diff) * *count as u32
        })
        .sum()
}

const fn triangle_number(index: u32) -> u32 {
    index * (index + 1) / 2
}
