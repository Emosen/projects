fn main() {
    // Build a vector of the strings "101", "102"..."105"
    // let mut v = Vec::new();
    // for i in 101 .. 105 {
    //     v.push(i.to_string());
    // }
    // // Pull out random elements from the vector.
    // let third = v[2];
    // let fifth = v[4];

    let mut v: Vec<String> = (101..106).map(|i| i.to_string()).collect();

    // Pop  a value off the end of the vector:
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // Move a value out of the middle of the vector, and move the last
    //element into its spot:
    let third = v.swap_remove(2);
    assert_eq!(third, "103");

    // Remaining elements in v are now: ["101", "102", "104"]

    // Swap in another value for the one we're taking out.
    let second = std::mem::replace(&mut v[1], "substitute".to_string());
    assert_eq!(second, "102");

    // Let's see what's left of our vector
    assert_eq!(v, vec!["101", "substitute", "104"]);

    println!("The vector's content is {:?}", v);


}
