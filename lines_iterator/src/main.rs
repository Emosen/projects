use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // let lines = BufReader::new(File::open("/etc/hosts").unwrap()).lines();

    let mut f = File::open("/etc/hosts");
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line);
    // println!("{:?}", lines.buf);

    println!("First line is {:?} bytes long", len);

    // parse(do_some_other_thing_or_nothing(lines));
}

// fn parse(T：do_some_other_thing_or_nothing){

// }

// fn do_some_other_thing_or_nothing(){

// }
