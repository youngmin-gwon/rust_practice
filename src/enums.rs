// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

    let m = Message::Write(String::from(""));

    // Option<T>
    let some_number = Some(5);
    let some_string = Some("a string");

    // None should be explicitly declared
    let absent_number: Option<i32> = None;
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum method
impl Message {
    fn call(&self) {}
}

// match example
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn in_cent_by_state(&self) -> u32 {
        return match self {
            Coin::Penny => {
                println!("Lucky penny!");
                return 1;
            }
            Coin::Nickel => 5,
            other => 2,
            // Coin::Dime => 10,
            // Coin::Quarter(state) => {
            //     println!("The state is: {:?}", state);
            //     25
            // }
        };
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
