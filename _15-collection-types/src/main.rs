use std::collections::HashMap;

fn main() {
    // COLLECTIONS

    // 1. VECTORS
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3, 4];

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(11);
    println!("{:?}", v);
    
    let third: &i32 = &v[2]; // Direct indexing
    println!("The third element is {}", third);

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    // UTF-8
    // let s = "whatever".to_string();
    // let s = String::from("whatever");
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("\nValue of s: {}", s);

    // Can also concatenate Strings using +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and can no longer be used.
    println!("Value of s3 using +: {}", s3);

    let s3 = format!("{} {}", "Hello,".to_string(), "world!".to_string());
    println!("Value of s3 using format: {s3}");

    // HASH MAPS
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}, {value}");
    }
}
