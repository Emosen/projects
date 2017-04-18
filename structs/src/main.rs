// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     // let user1 = User {
//     //     email: "someone@example.com",
//     //     username: "someusername123",
//     //     active: true,
//     //     sign_in_count: 1,
//     // };

//     let user1 = User {
//     email: String::from("someone@example.com"),
//     username: String::from("someusername123"),
//     active: true,
//     sign_in_count: 1,
// };
// }

// fn main(){
//     let length1 = 50;
//     let width1 = 30;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(length1,width1)
//     );
// }

// fn area(length: u32, width: u32) -> u32{
//     length * width
// }

// fn main(){
//     let rect1 = (50, 30);
    

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32{
//     dimensions.0 * dimensions.1
// }

struct Rectangle{
    length: u32,
    width: u32,
}

fn main(){
    let rect1 = Rectangle{length: 50, width: 30};
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.length * rectangle.width
}