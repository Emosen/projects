#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     // panic!("crash and burn");
//     // let v = vec![1, 2, 3];

//     // v[100];
//     let f = File::open("hello.txt");
//     // let f: u32 = File::open("hello.txt");

//     // let f = match f {
//     //     Ok(file) => file,
//     //     Err(error) => {
//     //         panic!("There was a problem opening the file: {:?}", error);
//     //     },
//     // };

//     let f = match f {
//         Ok(file) => file,
//         Err(ref error) if error.kind() == ErrorKind::NotFound => {
//             match File::create("hello.txt"){
//                 Ok(fc) => fc,
//                 Err(e) => {
//                     panic!("There was a problem opening the file: {:?}", e)
//                 },
//             }
//         },
//         Err(error) => {
//             panic!("There was a problem opening the file: {:?}", error)
//         },
//     };

// }
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {   
    let f = File::open("hello.txt");
}

fn read_username_from_file() -> Result<String, io::Error>{
    let mut s = String::new();

    File::open("hello.txt").read_to_string(&mut s);

    Ok(s)
}
