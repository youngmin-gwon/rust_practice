pub fn run() {
    let rect1 = Rectangle::square(32);
    let rect2 = Rectangle {
        width: 12,
        height: 12,
    };
    let rect3 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("The area of the rectangle is {}", rect1.area());

    println!("Can rect1 contain rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 contain rect3: {}", rect1.can_hold(&rect3));

    let color = Color(255, 255, 255);
    println!("Hex Color: {}", color.to_hex());
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Color(u8, u8, u8);

impl Color {
    fn to_hex(&self) -> String {
        format!("{:x}{:x}{:x}", self.0, self.1, self.2)
    }
}
