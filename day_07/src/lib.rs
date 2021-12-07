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
