pub fn run() {
    // if expression
    //
    // blocks of code associated with the conditions in `if` expressions
    // are sometimes called `arms`, just like `arms` in `match` expressions
    //
    // unlike languages like js and rb, rust will not automatically try to
    // covert non-Boolean types to a Boolean.
    let number = 5u8;
    if number < 5 {
        println!("Number is smaller than 5");
    } else if number > 5 {
        println!("Number is bigger than 5");
    } else {
        println!("Number is equal to 5");
    }

    // Ternary operator does not work in rust
    //
    let condition = true;
    // results from each arm should be same type
    let number = if condition { 3 } else { 4 };
    println!("The value of number is: {number}");

    // loop expression
    // - loop, while, for

    // returning value from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop labels
    // - if you have loops within loops, break and continue apply to
    //   innermost loop at that point. You can optionally specify a
    //   loop label on a loop that you can then use with `break` or
    //   `continue`.
    // - labels must begin with a single quote.

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
    }

    println!("End count = {count}");

    // conditional loops with while

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF");

    // looping through a collection with for
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("The value of element: {element}");
    }

    // reverse the range

    for element in (1..4).rev() {
        println!("The value of element: {element}");
    }
    println!("LIFTOFF!");
}
