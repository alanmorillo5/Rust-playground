// Can declare const in any scope
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // CONSTANTS

    // UPPER_SNAKE_CASE, must explicitly declare type
    const Y: i32 = 10;
    println!("The value of Y is: {}", Y);

    println!("The value of PI is: {}", PI);
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}
