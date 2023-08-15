pub fn run() {
    // Returning referenced value every time is tedious
    // We can provide a reference to the String value
    // Reference is like a pointer in that it's an address
    // we can follow to access the data stored at that address.
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");

    // First, notice that all the tuple code
    // in variables and the function return value in `owership.rs` is gone.
    // Second, note that we pass &s1 into calculate_length and,
    // in its definition, we take &String rather than String.
    // These ampersands represent references,
    // and they allow you to refer to some value without taking ownership of it.
    //
    // The &s1 syntax lets us create a reference that refers to the value of s1
    // but does not own it. Because it does not own it, the value
    // it points to will not be dropped when the reference stops being used.
    //
    // We call the action of creating a reference borrowing

    // Mutable Reference
    let mut s1 = String::from("Hello");
    change(&mut s1);

    println!("{s1}");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(s: &mut String) {
    s.push_str(", world");
}

fn failing_multiple_mutable_reference() {
    let mut s = String::from("Hello");

    let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow `s` as mutable
    // more than once at a time

    // The first mutable borrow is in r1 and
    // must last until it’s used in the println!

    // we can use curly brackets to create a new scope,
    // allowing for multiple mutable references,
    // just not simultaneous ones:
    //
    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;

    println!("{r1}");

    let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    println!("{r2}")

    // The benefit of having this restriction is
    // that Rust can prevent data races at compile time.
    // A data race is similar to a race condition and
    // happens when these three behaviors occur:
    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.
}

fn failing_immutable_reference() {
    let mut s = String::from("Hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    // We cannot have a mutable reference while we have an immutable one to
    // the same time

    println!("{r1}, {r2}");

    // a reference’s scope starts from where it is introduced and
    // continues through the last time that reference is used
    // The scopes of the immutable references r1 and r2 end
    // after the println! where they are last used,
    // which is before the mutable reference r3 is created.
    let r3 = &mut s;
    println!("{r3}");
}

// Dangling References
// a dangling pointer — a pointer that references a location in memory
// that may have been given to someone else
// —by freeing some memory while preserving a pointer to that memory
// In Rust, by contrast, the compiler guarantees that references
// will never be dangling references:
// if you have a reference to some data,
// the compiler will ensure that the data will not go out of scope
// before the reference to the data does.

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// Solution
fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}
