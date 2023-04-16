const MISSILES_COUNT:i32 = 8;
const READY_COUNT:i32 = 2;

fn main() {
    let ( missiles, ready): (i32, i32) = (MISSILES_COUNT, READY_COUNT);
    println!("Firing {} of my {} missiles...", ready, missiles - ready);
}
