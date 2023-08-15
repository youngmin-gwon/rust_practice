// common import
use rand::{CryptoRng, Error, Rng};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// glob operator
use std::io::*;

use crate::ch7::front_of_house;
// pub mod front_of_house {
// moving this module to new file
// go to front_of_house.rs
// pub mod hosting {
//     pub fn add_to_waitlist() {}
// }
// }

pub fn eat_at_restaurant() {
    // absolute path
    crate::ch7::restaurant::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();
}
fn server_order() {}

pub mod back_of_house {

    pub fn fix_incorrect_order() {
        cook_order();
        super::server_order();
    }

    fn cook_order() {}
    // privacy rule for struct
    //
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_summer() {
    let mut meal = back_of_house::Breakfast::summer("egg");
    meal.toast = String::from("Wheat");
}

pub fn eat_appetizer() {
    let order1 = back_of_house::Appetizer::Soup;
    println!("{:?}", order1);
}

pub fn eat_onsite() {
    front_of_house::hosting::add_to_waitlist();
}
