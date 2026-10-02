fn main() {
    hello_world();
    tell_height(12);
    human_id("Alan", 21, 180.0);

    // Expressions and Statements
    // Expression: Anything that returns a value.
    // Examples: 5, true/false, add(3, 4), if conditions
    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is: {}", x);

    let y = add(4, 6);
    println!("Value of y is: {}", y);
    println!("Value from add fn is: {}.", add(4, 6));

    // Statement: Anything that does not return a value.
    // Statements in Rust end with ;
    // Examples: Variable declarations, function definitions, control flow statements
    let weight = 72.5;
    let height = 1.8;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi);
}

// Rust auto-cleans memory allocated to any variable at the end of a function.

// FUNCTIONS: use "fn" + function name to define a function
// Functions should be written in snake_case
// In rust, you can call your function anywhere (hoisting) but should define
// methods above calling ones.

fn hello_world() {
    println!("Hello, Rust!")
}

// input values
fn tell_height(height: u32) {
    println!("My height is {} cm.", height);
}

// inserting more than one value
fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm.",
        name, age, height);
}

// functions returing values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
