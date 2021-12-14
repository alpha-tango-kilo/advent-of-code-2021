use day_14::smart_perhaps::*;

fn main() {
    let (chars, rules) = input_chars_rules();
    let freq_map = freq_map(&chars, &rules, 40);
    let lowest = freq_map.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let highest = freq_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    println!(
        "The count difference between most and least common element is {}",
        highest - lowest
    );
}
