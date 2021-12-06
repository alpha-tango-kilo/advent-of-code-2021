use day_06::input_lantern_fish_school;

fn main() {
    let mut school = input_lantern_fish_school();
    school.simulate_n(256);
    println!("After 256 days, there are {} fish", school.count());
}
