use std::collections::HashMap;
use std::fs;

pub fn input_hash_map() -> HashMap<u16, u16> {
    let mut freq_map = HashMap::new();
    fs::read_to_string("day_07/input")
        .expect("Failed to read input file")
        .trim_end()
        .split(',')
        .map(|num_str| num_str.parse::<u16>().expect("Bad input"))
        .for_each(|n| match freq_map.get_mut(&n) {
            None => {
                freq_map.insert(n, 1);
            }
            Some(freq) => *freq += 1,
        });
    freq_map
}

pub fn get_max_key<K: Ord, V>(map: &HashMap<K, V>) -> &K {
    map.keys().max().expect("Max key requested for empty map")
}

pub fn least_fuel_use(
    freq_map: &HashMap<u16, u16>,
    fuel_fn: fn(&HashMap<u16, u16>, u16) -> u32,
) -> (u16, u32) {
    let max = *get_max_key(freq_map);
    (0..=max)
        .into_iter()
        .map(|target_pos| (target_pos, fuel_fn(freq_map, target_pos)))
        .min_by(|(_, fuel_use_a), (_, fuel_use_b)| fuel_use_a.cmp(fuel_use_b))
        .unwrap()
}
