pub fn run() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("You got a lucky coin");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("You've got the quarter from {:#?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn dice_roll(x: u8) {
    match x {
        3 => println!("do something for 3"),
        7 => println!("do something for 7"),
        _ => (),
    }
}
