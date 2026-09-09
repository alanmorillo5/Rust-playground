// run "cargo new _" to create project
// run "cargo run" to compile and run project

fn main() {
    println!("Hello, 💀 from CARGO !");

    // signed 32 bit int, range of -2^31 to 2^31 - 1
    let x: i32 = -42;

    // unsigned 64 bit int, range of 0 to 2^64- 1
    let y: u64 = 32;

    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    // 32 bit floating point, range of -3.40282347e38 to 3.40282347e38
    let pi: f32 = 3.1415;
    println!("Value of pi: {}", pi);

    // boolean
    let is_rust: bool = true;
    println!("Am I coding in Rust? {}", is_rust);

    // char - represents a single unicode character
    let fav_letter: char = 'a';
    println!("My favorite letter is {}", fav_letter);
}
