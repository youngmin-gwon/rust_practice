use std::fmt::Display;

// lifetimes
//
// Rather than ensuring that a type has the behavior we want,
// lifetimes ensure that references are valid as long as we need them to be
// Every reference in Rust has a lifetime, which is the scope for which
// that reference is valid.
pub fn run() {
    // 1. preventing dangling references with lifetimes
    invoke_dangling_reference();
    // 2. generic life time in function
    generic_lifetime();

    // 4. lifetime annotation in struct
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // 7. The static lifetime
    // `'static` denotes that the affected reference can live for the entire duration of the program.
    println!("first sentence is {}", i.part);
}

// 1. preventing dangling references with lifetimes
fn invoke_dangling_reference() {
    // 1. preventing dangling references with lifetimes
    //
    // This code below invokes compiler error since
    // it has a dangling reference.
    //
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r)
}
// How does Rust determine that this code is invalid?
// - It uses a borrow checker

// 2. generic lifetimes in functions
fn generic_lifetime() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());

    println!("The longest string is {}", result);

    let static_word: &'static str = "I have a static lifetime.";
}

// $ cargo run
//    Compiling chapter10 v0.1.0 (file:///projects/chapter10)
// error[E0106]: missing lifetime specifier
//  --> src/main.rs:9:33
//   |
// 9 | fn longest(x: &str, y: &str) -> &str {
//   |               ----     ----     ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider introducing a named lifetime parameter
//   |
// 9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//   |           ++++     ++          ++          ++
// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `chapter10` due to previous error
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
//
// So, it needs lifetime annotation syntax!
//
// Having function signatures contain the lifetime contract means
// the analysis the Rust compiler does can be simpler.

// 3. Lifetime annotation syntax
// - Lifetime annotations don’t change how long any of the references live.
//   Rather, they describe the relationships of the lifetimes of multiple references
//   to each other without affecting the lifetimes.
//   Just as functions can accept any type when the signature specifies a generic type parameter,
//   functions can accept references with any lifetime by specifying a generic lifetime parameter.
// - Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters
//   must start with an apostrophe(') and are usually lowercase and very short.
//   ```rust
//   &i32 // a reference
//   &'a i32 // a reference with an explicit lifetime
//   &'a mut i32 // a mutable reference with an explicit lifetime
//   ```
// - To use lifetime annotations in function signatures,
//   we need to declare the generic lifetime parameters inside angle brackets
//   like generic type parameters.
// - Remember, when we specify the lifetime parameters
//   in this function signature,
//   we’re not changing the lifetimes of any values passed in or returned.
//   Rather, we’re specifying that
//   the borrow checker should reject any values that don’t adhere to these constraints.
// - When annotating lifetimes in functions,
//   the annotations go in the function signature, not in the function body.
fn longest<'a>(x: &'a str, y: &'a /*input lifetime */ str) -> &'a /* output lifetime */ str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 4. lifetime annotation in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 5. lifetime elision
// What about this? why is this compiled?
fn first_word(words: &str) -> &str {
    let bytes = words.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &words[0..i];
        }
    }

    words
}
// It wouldn't have compiled till pre-1.0 version, but it is now okay because
// lifetime annotation is elided because of the redundancy.
//
// The compiler uses 3 rules to figure out the lifetimes of the references when
// there aren't explicit annotations. If the compiler gets to the end of the three rules
// and there are still references for which it can’t figure out lifetimes,
// the compiler will stop with an error.
//
//   1) (input lifetimes) the compiler assigns a lifetime parameter to each parameter that's a reference.
//   2) (output lifetimes) If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
//   3) (output lifetimes) f there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

// 6. lifetime annotation in method definitions
// - In method signatures inside the impl block,
//   references might be tied to the lifetime of references
//   in the struct’s fields, or they might be independent.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 8. generic type parameters, trait bounds, and lifetimes together
fn longest_with_an_annotation<'a, T>(x: &'a str, y: &'a str, ann: T,) -> &'a str
where
T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}