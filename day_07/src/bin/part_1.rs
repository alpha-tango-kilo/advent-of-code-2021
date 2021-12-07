use day_07::{fuel_use_to, get_max_key, input_hash_map};

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
