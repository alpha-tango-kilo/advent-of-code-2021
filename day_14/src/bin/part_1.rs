use day_14::input_polymer_rules;

fn main() {
    let (mut polymer, rules) = input_polymer_rules();
    polymer.pair_insertion_many(&rules, 10);
    let freq_map = polymer.freq_map();
    let lowest = freq_map.iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .1;
    let highest = freq_map.iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .1;
    println!("The count difference between most and least common element is {}", highest - lowest);
}
