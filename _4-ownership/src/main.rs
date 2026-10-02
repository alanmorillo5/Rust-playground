fn main() {
    // OWNERSHIP

    // Each value in Rust has an owner.
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);

    // There is only one owner of a value at a given time.
    let _s2 = s1;
    // println!("{}", s1); // compile error

    // When the owner goes out of scope, the value will be dropped.
}

fn calculate_length(s:&String) -> usize {
    s.len()
}
