pub fn run() {
    // enums give you a way of saying
    // a value is one of a possible set of values.
    //
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // Better Approach
    let home = IpAddr::V4(255u8, 255u8, 0u8, 0u8);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    // Option Types
    let some_number = Some(5);
    let some_char = Some('e');

    // Rust requires us to annotate the overall Option type:
    // the compiler can’t infer the type that the corresponding
    // Some variant will hold by looking only at a None value.
    let absent_number: Option<i32> = None;

    // why is having Option<T> any better than having null?
    // In short, because Option<T> and T (where T can be any type)
    // are different types, the compiler won’t let us use an Option<T> value
    // as if it were definitely a valid value.
    //
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    //
    // let sum = x + y; // it cause an error!
    //
    // You have to convert an Option<T> to a T
    // before you can perform T operations with it.
    // Generally, this helps catch one of the most common issues with null:
    // assuming that something isn’t null when it actually is.
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddrKind {
//     V4,
//     V6,
// }

// Better Approach
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Another Example
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// Option Enum and Its Advantages over Null Values
// - another enum defined by the standard library
// - very common scenario in which value could be something or nothing.
// - Rust doesn't have Null feature because it causes error. Instead,
//   it has an enum that concept of a value being present or absent.
// - That is Option<T> defined by the standard library.
//
//   enum Option<T> {
//     None,
//     Some(T),
//   }
