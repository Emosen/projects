fn main() {
    let mut x: Vec<i32> = vec![1i32, 2, 3];

    //更新数组
    //push中对数组进行了可变借用，并在push函数退出时销毁这个借用
    x.push(10);

    {
        //可变借用1
        let mut y = &mut x;
        y.push(100);

        //可变借用2，注意：此外是对y的借用，不可再对x进行借用
        //因为y在此时依然存活。
        let z = &mut y;
        z.push(1000);

        println!("{:?}", z); //打印：[1, 2, 3, 10, 100, 1000]
    } // y 和 z 在此处被销毁，并释放借用。

    //访问x正常
    println!("{:?}", x); //打印：[1, 2, 3, 10, 100, 1000]
}
