extern crate libc;

use libc::c_int;
use libc::c_void;
use libc::size_t;

#[repr(C)]
struct RustObject {
    a: c_int, 
    // other members
}
extern "C" fn callback(a: c_int) {
    println!("hello {}!", a);
}

#[link(name = "snappy")]
extern "C" {
    fn run_callback(data: i32, cb: extern "C" fn(i32));
}
fn main() {
    unsafe {
        run_callback(1 as i32, callback);
    }
}
