fn main() {
    // SHADOWING

    // Can declare a variable using the previous definition of it.
    let x: i32 = 5;
    // when shadowing, type must be the same.
    let x: i32 = x + 1;

    {
        let x = x * 2;
        println!("The value of x in this inner scope is {}", x);
    }
    // These changes never escape the scope
    println!("The value of x is {}", x);
}
