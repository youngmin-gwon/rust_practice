use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

pub fn run() {
    // use panic in exceptional circumstances in which recovering from error is
    // not possible
    // or example code
    trace_a();
    error_matching();
    println!("{:?}", error_propagation());
}

fn trace_a() {
    trace_b();
}

fn trace_b() {
    trace_c(22);
}

// if you put `RUST_BACKTRACE=1 cargo run`, then you can trace call stack
fn trace_c(num: i32) {
    if num == 21 {
        panic!("Don't pass in 22");
    }
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn error_matching() {
    let f = File::open("hello.txt");
    // 1. using match
    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("problem creating the file: {:?}", e),
            },
            other_error => panic!("problem opening the file: {:?}", other_error),
        },
    };

    // 2. using closure
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 3. other ways
    let f = File::open("hello.txt").expect("Failed to open file");
}

fn error_propagation() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");
    //
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(err) => return Err(err),
    // };
    //
    // let mut result = String::new();
    //
    // match f.read_to_string(&mut result) {
    //     Ok(_) => Ok(result),
    //     Err(e) => Err(e),
    // }
    // --------- verbose ---------

    // ?: return Err early instead of panicking
    // let mut result = String::new();
    // File::open("hello.txt")?.read_to_string(&mut result)?;
    // Ok(result)
    // -------- shorter ---------
    fs::read_to_string("hello.txt")

    // ? operator cannot be used in main function
    // because it does not return anything
    // fn main() -> Result<(), Box<dyn Error>> { // Box means Trait objects: any type of error
    //     Ok(())
    // }
}

// guess example for solid code
struct Guess {
    value: i32,
}

impl Guess {
    fn value(&self) -> i32 {
        self.value
    }

    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Guess { value }
    }
}
