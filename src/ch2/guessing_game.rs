use rand::Rng;
use std::cmp::Ordering; // enum (grater,equal,less)
use std::io;

pub fn run() {
    println!("Guess the number!");

    // local to the current thread
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // gen_range takes range as an argument
    // range: [1,100]

    loop {
        println!("Please input your guess.");

        // :: indicate that keyword after :: is an associated
        // function of the keyword front
        let mut guess = String::new(); // utf-8 encoded

        io::stdin()
            .read_line(&mut guess) // read_line append input, not overrides
            .expect("Failed to read line");
        // & indicates a reference
        // expecet() handles potential failure with result
        // red_line() returns a Result
        // Result is an enumeration, which is a type
        // that can be in one of multiple possible states
        // Result's variants are `Ok` and `Err`

        // rust cannot compare string with integer
        // that is why this variable is converted into unsigned number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // guess is created again: `Shadowing`
        // Shadowing let user reuse the variable name
        // rather than creating two unique variables
        //
        // When using `parse()`, user needs to tell Rust the exact number type
        // by using `let --: u32`

        println!("You guessed: {guess}");

        // match: decide what to do next based on the enum variants.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
