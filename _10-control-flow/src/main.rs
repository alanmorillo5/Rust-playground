fn main() {
    // IF ELSE STATEMENTS
    let age: u16 = 18;
    if age >= 18 {
        println!("You are old enough to vote");
    } else {
        println!("You cannot vote.")
    }

    let number = 1;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2")
    }

    let condition = false;
    let number = if condition {5} else {6}; // types must be the same
    println!("Number: {number}");
}
