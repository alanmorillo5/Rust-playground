fn main() {
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
    println!("Slice value: {}", slice);
}
