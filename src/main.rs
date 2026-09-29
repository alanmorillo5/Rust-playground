// run "cargo new _" to create project
// run "cargo run" to compile and run project


// main() is the project's entry point.
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
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices - a dynamically sized view into an adjacient sequence of elements in memory.
    // They borrow data from the original list, not create a new one.
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book slices: {:?}", book_slices);

    // NOTE: all variables are immutable unless specified when declared.

    // Strings vs String Slices (&str)
    // Strings are growable, mutable, and owned.
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // String Slices are references to Strings.
    // Great for memory efficiency, are always immutable.
    let string: String = String::from("Hello, World");
    let slice: &str = &string[0..5]; // ampersand is a reference key
    println!("Slice value: {}\n", slice);

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
    println!("\nResult is: {}", x);

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

    // OWNERSHIP

    // Each value in Rust has an owner.
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("\nLength of '{}' is {}.", s1, len);

    // There is only one owner of a value at a given time.
    let s2 = s1;
    // println!("{}", s1); // compile error

    // When the owner goes out of spoce, the value will be dropped.

    // REFERENCES AND BORROWING
    
    // References borrow values without taking ownership using &. Can be immutable or mutable.
    let mut x = 5;
    let r = &mut x;

    *r += 1;
    *r -= 3;

    println!("\nValues of x: {}\n", x);

    let mut account = BankAccount {
        owner: "Alan".to_string(),
        balance: 150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();
    // Mutable borrow to withdraw money
    account.withdraw(45.5);
    account.check_balance();

    // VARIABLES AND MUTABILITY
    let mut a = 5;
    println!("\nThe value of a is {}.", a);
    a = 10;
    println!("The value of a is {}.", a);

    // CONSTANTS

    // UPPER_SNAKE_CASE, must explicitly declare type
    const Y: i32 = 10;
    println!("\nThe value of Y is: {}", Y);

    println!("The value of PI is: {}", PI);
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // SHADOWING

    // Can declare a variable using the previous definition of it.
    let x: i32 = 5;
    // when shadowing, type must be the same.
    let x: i32 = x + 1;

    {
        let x = x * 2;
        println!("\nThe value of x in this inner scope is {}", x);
    }
    // These changes never escape the scope
    println!("The value of x is {}", x);

    // COMMENTS

    // this is for printing "hello, world"
    println!("Hello, World!") // this is for printing "Hello, World!"
    /*
    This is a block comment.
    multiple lines can go here.
    */


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

fn calculate_length(s:&String) -> usize {
    s.len()
}

// struct - a data structure that allows you to group multiple fields together under one name.
struct BankAccount {
    owner: String,
    balance: f64,
}

// impl - attaches methods and functions for a struct or enum.
impl BankAccount {
    
    fn withdraw(&mut self, amt: f64) {
        println!("Withdrawing {} from account owned by {}.", amt, self.owner);
        self.balance -= amt;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {:.2}.", self.owner, self.balance);
    }
}

// Can declare const in any scope
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;