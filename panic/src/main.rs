fn main() {
    use std::net::IpAddr;

    let home = "127.0.0.1".parse::<IpAddr>().unwrap();

    println!("The home is {:?}", home);
}
