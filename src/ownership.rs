pub fn run() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let num1 = 1;
    let num2 = 2;
    let sum = add(num1, num2);

    println!("{:?}", (num1, num2, sum));

    let mut s2: String = String::from("Hello");
    println!("s2: {s2}");

    change(&mut s2);

    println!("s2: {s2}");

    let word = first_word(&s2);
    println!("{word}");

    let word = first_word(&s2[..3]);
    println!("{word}");
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn add(num_1: i32, num_2: i32) -> i32 {
    return num_1 + num_2;
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// this is a better way to work both in string literal and String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    return &s[..];
}
