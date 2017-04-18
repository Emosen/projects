fn main() {
    let spaces = "    ";
    let spaces = spaces.len();

    println!("The number of spaces are: {} ", spaces);

    //addition
    let sum = 5 + 10;
    println!("The value of sum is: {} ", sum);

    //subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {} ", difference);

    //multiplication
    let product = 4 * 30;
    println!("The value of product is: {} ", product);

    //division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {} ", quotient);
    
    //remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {} ", remainder);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {} ", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {} ", y);
    println!("The value of tup 2 is: {} ", tup.2);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("The value of months 2 is: {} ", months[2]);

        
}
