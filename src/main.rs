// #[path = "ch2/guessing_game.rs"] mod guessing_game;
// #[path = "ch3/variable.rs"] mod variable;
// #[path = "ch3/data_types.rs"] mod data;
// #[path = "ch3/function.rs"] mod function;
// #[path = "ch3/control_flow.rs"] mod control;
// #[path = "ch3/ch3_examples.rs"] mod example;
// #[path = "ch4/ownership.rs"] mod ownership;
// #[path = "ch4/borrowing.rs"] mod borrowing;
// #[path = "ch4/slices.rs"] mod slices;
// #[path = "ch5/structs.rs"] mod structs;
// #[path = "ch5/practice.rs"] mod practice;
// #[path = "ch6/enums.rs"] mod enums;
// #[path = "ch6/match.rs"] mod matches;
// #[path = "ch6/if_let.rs"]
// mod if_let;
// pub mod ch7;
// use ch7::crate_import::Asparagus;
// use ch7::restaurant;
// #[path = "ch8/collections.rs"]
// mod collections;
// #[path = "ch9/error_handling.rs"]
// mod error_handling;
// #[path = "ch10/generics.rs"]
// mod generics;
// #[path = "ch10/traits.rs"]
// mod traits;
#[path = "ch10/validating_references_with_lifetimes.rs"]
mod validating_references_with_lifetimes;

fn main() {
    // guessing_game::run();
    // variable::run();
    // control::run();
    // fibonacci_example();
    // temperature_example();
    // ownership::run();
    // borrowing::run();
    // slices::run();
    // structs::run();
    // practice::run();
    // matches::run();
    // if_let::run();
    // ch7_run();
    // restaurant::front_of_house::hosting::add_to_waitlist();
    // restaurant::back_of_house::fix_incorrect_order();
    // collections::run();
    // error_handling::run();
    // generics::run();
    // traits::run();
    validating_references_with_lifetimes::run();
}
