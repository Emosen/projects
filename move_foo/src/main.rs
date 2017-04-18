#[derive(Debug)]
struct Foo(i32);

impl Drop for Foo{
    fn drop(&mut self){
        println!("drop:{:p}",self);
    }
}

fn main() {
    let mut f = Foo(100);
    println!("f的地址:{:p},f={:?}",&f,f);
    let p = &mut f as *mut Foo;

    let mut f2=f;
    f2.0=10;
    println!("f2地址:{:p},f2={:?}",&f2,f2);

    unsafe{
        println!("开始unsafe操作");
        println!("给f2.0赋值后,解引用 *p = {:?}",*p);
        (*p).0=200;
        println!("给(*p).0赋值后,解引用 *p = {:?}",*p);
    };

    println!("给*p赋值后，f2={:?}",f2);
}
