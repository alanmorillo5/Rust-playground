// run "cargo new _" to create project
// run "cargo run" to compile and run project

fn main() {
    println!("Hello, 💀 from CARGO !");

    // PRIMITIVE DATA TYPES: int, float, bool, char

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

    // COMPOUND DATA TYPES: array, tuple, slice, string (slice string)

    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    // Compile error
    // let mix = [1, 2, "apple", true];
    // println!("Mix Array: {:?}", mix)

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    
    // Accessing from an array
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple)
}
