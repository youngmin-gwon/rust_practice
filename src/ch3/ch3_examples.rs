pub fn fibonacci(order: i32) -> i32 {
    if order <= 1 {
        return order;
    }
    fibonacci(order - 1) + fibonacci(order - 2)
}

pub fn celsius_to_fahrenheit(degree: i32) -> i32 {
    (degree * 9 / 5) + 32
}

pub fn fahrenheit_to_celsius(degree: i32) -> i32 {
    (degree - 32) * 5 / 9
}
