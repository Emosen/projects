fn example(x: Box<i32>, y: i32) -> i32 {
    *x + y
}

fn main() {
    let mut num = 1;
    loop {
        let x = Box::new(example(Box::new(4i32), 21));
        num = num + 1;
        // println!("{:?}", num);
        if num >= 1000 {
            println!("{:?}", x);
            break;
        }
    }
    // for n in 1..101 {
    //     println!("the n value is {}", n);
    // }
}
