// Primitive str = Immutable fixed-length string 
//                 somewhere in memory
// String = Growable, heap-allocated data structure 
// - Use when you need to modify or own
//   string data
pub fn run(){
    let hello = "Hello"; // &str
    let mut hello_grow = String::from("Hello ");

    // Get length
    println!("Length: {}", hello_grow.len());

    // push: for char type
    hello_grow.push('W') ;

    // push_str: for string
    hello_grow.push_str("orl");

    // Capacity in bytes
    println!("Capacity: {}", hello_grow.capacity());

    // Check if empty
    println!("Is Empty: {}", hello_grow.is_empty());

    // Contains
    println!("Contains 'World' {}", hello_grow.contains("World"));

    // Replace
    println!("Replace: {}", hello_grow.replace("Worl", "There"));

    // Loop through string by whitespace
    for word in hello_grow.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a')  ;
    s.push('b')  ;

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}",s);


    println!("{},{}",hello, hello_grow);
}