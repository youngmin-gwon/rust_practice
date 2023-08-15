pub fn run() {
    // less verbose way to handle values that match one pattern
    // while ignoring the rest.
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    //
    // we can shorten it
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // But remember
    // gaining conciseness is an appropriate trade-off
    // for losing exhaustive checking
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //     _ => count += 1,
    // }
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}

enum Coin {
    Penny,
    Dime,
    Nickel,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
