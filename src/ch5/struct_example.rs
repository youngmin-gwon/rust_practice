pub fn run() {
    // let width1 = 30;
    // let height1 = 30;
    //
    // let rect1 = (30, 50);
    //
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect)
    // );

    // Originally throws an error saying `Rectangle` doesn't implement
    // `std:fmt:display`
    println!("rect is {:#?}", rect);

    // Another way to print out a value using the `Debug` format is `dbg!` macro
    // it takes a reference.
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // method
    println!("The area of the rectangle is {}", rect.area());

    // method with more parameters
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // associated functions
    // - all functions defined within an `impl` block are called
    // `associated functions` because tey are associated with te type
    // after the `impl`
    // - we can define associated functions that don't need an instance of
    // the type to work with
    let square = Rectangle::square(3);
    println!("{:#?}", square);
}

// Original
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Refactoring 1: with tuple
// fn area(rect: (u32, u32)) -> u32 {
//     rect.0 * rect.1
// }

// Refactoring 2: with struct
fn area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

// refactoring 3: with derived traits
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method syntax
impl Rectangle {
    // &self is short form of self: &Self
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
