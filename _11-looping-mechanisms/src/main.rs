fn main() {
    // LOOPS
    let mut counter = 0;
    let result = loop { // loop runs forever until broken
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels to disambiguate nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    };

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
        // break;
    }

    // loop thru collection using for loop
    let a = [1, 2, 3, 4, 5, 6];
    for element in a {
        println!("{element}");
    }
}
