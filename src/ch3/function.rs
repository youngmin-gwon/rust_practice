pub fn run() {
    // snake case
    hello_world();

    // parameter
    print_labeled_measurement(5, 'h');

    // statements and expressions
    // - function bodies are made up of a series of statements optionally
    //   ending in an expression.
    // - important distinctions to understand since other languages don't have
    //   the same.
    // - `Statements` are instructions that perform some action and do not
    //   return a value.
    //   - Creating a variable and assigning a value to it with the let
    //     keyword is a statement.
    //   - `let y = 6` is statement.
    //   - function definitions are also statements.
    //   - statements do not return values.
    //     => can't assign a let statement to another variable
    //     let x = (let y = 6); // error
    //     => (impossible) x = y = 6;
    // - `Expressions` evaluate to a resultant value.
    //   - Expressions make up most of the rest of the code that you'll write.
    //   - Expressions can be part of statements.
    //     6 is expression in ```let y = 6;```
    //     calling a function is an expression.
    //     calling a macro is an expression.
    //     a new scope block created with curly brackets is an expression
    let y = {
        // this is expression
        let x = 3;
        x + 1 // Note that this does not have semicolon.
              // if you put semicolon, it turns to a statement, then will not
              // return a value.
    };
    println!("The value of y is {y}");

    // functions with return values
    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}")
}

fn hello_world() {
    println!("Hello world");
}

// type of parameter must be declared
// then, compiler almost never needs you to use them elsewhere in the code
// to figure out what type you mean.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
