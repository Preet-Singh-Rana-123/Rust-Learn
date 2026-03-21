// COLLECTION TYPES IN RUST
// 1. Vector
// 2. UTF-8 Strings
// 3. Hash Map

use std::collections::HashMap;

fn main() {
    // VECTOR
    let mut v1: Vec<i32> = Vec::new();
    let v2: Vec<i32> = vec![1, 2, 3, 4, 5];

    v1.push(10);
    v1.push(20);
    v1.push(30);
    v1.push(40);
    v1.push(50);

    println!("{:?}", v1);
    println!("{:?}", v2);

    // here we take refrence because we do not want to take ownership from v2.
    let third: &i32 = &v2[2];
    println!("Third element in vector v2: {}", third);

    // STRINGS
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // now here s1 no longer exists because now s3 has ownership of s1.
    println!("{}", s3);

    // HASH MAP
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    for (key, values) in scores {
        println!("{} - {}", key, values);
    }
}
