// Structs - Used to create custom data types

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        // it returns value. So it should not be with semi-colon at the end.
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    //////////

    // let mut c = Color(255, 0, 0);

    // c.0 = 200;

    // println!("Color: {},{},{}", c.0, c.1, c.2);

    let first = "Mary";
    let last = "Doe";

    // Field Init Shorthand
    let p1 = Person::new(first, last);

    // Struct Update Syntax
    let p2: Person = Person { ..p1 };

    //////////
    let mut p = Person::new("Mary", "Doe");
    println!("Person: {} ", p.full_name());

    p.set_last_name("William");
    println!("Person: {} ", p.full_name());
    println!("Person Tuple: {:?} ", p.to_tuple());

    // new example
    let height = 50;
    let width = 30;
    let rect1 = Rectangle { width, height };
    let rect2 = Rectangle {
        width: width / 10,
        height: height / 10,
    };

    let square = Rectangle::square(50);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    println!("square is {:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method syntax
impl Rectangle {
    // associated function: Self
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // method
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    // method with more parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
}
