use day_06::input_lantern_fish_school;

fn main() {
    let mut school = input_lantern_fish_school();
    school.simulate_n(80);
    println!("After 80 days, there are {} fish", school.count());
}
