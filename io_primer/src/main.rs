// use std::io;
// fn main() {
//     print!("请输入一个字符串：");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("读取失败");
//     print!("您输入的字符串是：{}\n", input);
// }

// Example 2
// use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

// fn main() {
//     // 创建一个文件路径
//     let path = Path::new("hello.txt");
//     let display = path.display();

//     // 打开文件只读模式, 返回一个 `io::Result<File>` 类型
//     let mut file = match File::open(&path) {
//         // 处理打开文件可能潜在的错误
//         Err(why) => panic!("couldn't open {}: {}", display,
//                                                    Error::description(&why)),
//         Ok(file) => file,
//     };
// // 文件输入数据到字符串，并返回 `io::Result<usize>` 类型
//     let mut s = String::new();
//     match file.read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read {}: {}", display,
//                                                    Error::description(&why)),
//         Ok(_) => print!("{} contains:\n{}", display, s),
//     }
// }

// 输出文本
static LOREM_IPSUM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    // 用只写模式打开一个文件，并返回 `io::Result<File>` 类型
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // 写入 `LOREM_IPSUM` 字符串到文件中, 并返回 `io::Result<()>` 类型
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}",
                   display,
                   Error::description(&why))
        }
        Ok(_) => println!("successfully wrote to {}", display),
    }
}