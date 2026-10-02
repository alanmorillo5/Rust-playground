fn main() {
    // ERROR HANDLING
    
    /*
    option 1:  OPTION
    enum Option<T> { // define generic Option type
        Some(T), // Represents a value
        None // Represents absence of a value
    }

    approach 2: RESULT
    enum Result<T, E> { // define generic Result type
        Ok(T), // Represents a value
        Err(E) // Represents an error
    }
    */

    let result = divide_option(1.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero.")
    }

    match divide_result(100.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err)
    };
}

fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero.".to_string())
    } else {
        Ok(numerator / denominator)
    }
}