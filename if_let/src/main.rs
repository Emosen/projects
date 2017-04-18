fn main() {
    // println!("Hello, world!");
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => (),
    }
}
