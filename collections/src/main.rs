fn main() {
//    let s = String::new();
   let data = "initial contents";

   let mut s = data.to_string();

   // the method also works on a literal directly:
   let strs = "test initial contents".to_string();

   s.push('l');
   println!("The s value {:?}", s);
   println!("The strs value {:?}", strs);

   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");

   let s_new = format!("{}-{}-{}",s1, s2, s3);
   println!("The s_new value {:?}", s_new);

   let hello = "Здравствуйте";
   let answer = &hello[0..4];
   println!("The answer value {:?}", answer);

//    for c in "नमस्ते".chars() {
//         println!("{}", c);
//     }

//     for b in "नमस्ते".bytes() {
//         println!("{}", b);
//     }

    // below are for hashmap
    use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("The score value {:?}", score);

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;

    }

    println!("{:?}", map);
}
