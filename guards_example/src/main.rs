
fn main() {
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These two are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom"),
        (x, _) if x % 2 == 0 => println!("The first one is odd"),
        _ => println!("no correlation"),
    }
}
