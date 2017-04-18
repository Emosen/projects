pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;
// use a::series::of;
use a::series::of::nested_modules;

fn main() {
    nested_modules();
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
