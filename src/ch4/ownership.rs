// Ownership is a set of rules that govern how a Rust program manages memory.
// Memory is managed through a system of ownership with a set of rules that the
// compiler checks.
// None of the features of ownership will slow down your program while it’s running.
//
// Ownership rules
// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
pub fn run() {
    explain_memory_alloc();
    string_with_move();
    deep_copy();

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    // very tedious to use same variable again
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}");
} // Here, x goes out of scope, then s. But because s's value was moved,
  // nothig special happens.

// this only works for data type whose size is not set in compile-time.
fn explain_memory_alloc() {
    // The double colon :: operator allows us to namespace this particular from
    // function under the String type rather than using some sort of name like string_from
    let mut s = String::from("hello"); // memory allocation at runtime
    s.push_str(", world!");
    // let s = "hello";
    // let s = String::from("hello");
    //
    // Why can String be mutated but literals cannot?
    // The difference is in how these two types deal with memory.
    //
    // In the case of a string literal, we know the contents at compile time,
    // so the text is hardcoded directly into the final executable.
    // This is why string literals are fast and efficient.
    // But these properties only come from the string literal’s immutability.
    println!("{}", s);
} // when a variable goes out of scope, rust calls a special function called
  // `drop` for us to free memory

fn string_with_move() {
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, {s2} again");

    // This function brings error since string in rust(or all other languages)
    // does not copy every string to heap. it just refers common heap.
    //
    // When s2 and s1 go out of scope,
    // they will both try to free the same memory.
    // This is known as a double free error
    // and is one of the memory safety bugs we mentioned previously.

    // it looks like shallow copy, but because Rust also invalidates the first
    // variable, instead of being called a shallow copy,
    // it is known as a `move`.
    // s1 is `moved` into s2.
    // With only s2 valid, when it goes out of scope it alone will free the
    // memory, and we're done.
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2} again");
}

fn deep_copy() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // this code may be expensive.
                         // a visual indicator that something different is going on

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// multiple value returns
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
