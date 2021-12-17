use day_16::input_packet;

fn main() {
    let packet = input_packet();
    println!("The total of all packet version numbers is: {}", packet.version_total());
}
