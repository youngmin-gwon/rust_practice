pub fn run() {
    // let user1 = User{
    //     email: String::from("someone@example.com"),
    //     username: String::from("someone"),
    //     active: true,
    //     sign_in_count: 1,
    // };
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    // accessing field
    user1.active = false;
    // rust only allow all mutable or all immutable struct

    // shorthand to create new instance from other instance
    let user2 = User {
        email: String::from("another@example.com"),
        // it must come last
        ..user1
    };

    let black = Color(0, 0, 0);
    let mut origin = Point(0, 0, 0);

    // fields in tuple-like struct can be accessed through index
    origin.0 = 2;
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: i64,
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     username: username,
    //     email: email,
    //     active: true,
    //     sign_in_count: 1,
    // }
    // shorthand for init because it has same parameter name
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
// - useful when you need to implement a trait on some type but
//   don't have any data that you want to store in the type itself.
struct AlwaysEqual;
