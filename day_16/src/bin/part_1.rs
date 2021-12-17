use day_16::input_packets;

fn main() {
    let packets = input_packets();
    let answer = packets.into_iter()
        .map(|p| p.version_total())
        .sum::<u32>();
    println!("The total of all packet version numbers is: {}", answer);
}
