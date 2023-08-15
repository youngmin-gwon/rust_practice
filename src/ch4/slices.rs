// A slice is a kind of reference, so it does not have ownership.

pub fn run() {
    // first_trial();
    second_trail();
    slice_equal();
}

// wrong
fn first_trial() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
} // that is why we have string slices in Rust

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn second_trail() {
    let s = String::from("Hello world");

    let word = first_word_improved(&s[1..]);

    // s.clear(); // error!

    println!("the first word is: {word}");
}

fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn slice_equal() {
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
