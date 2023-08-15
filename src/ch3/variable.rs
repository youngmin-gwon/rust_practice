pub fn run() {
    // mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    //
    // not allowed to use mut with constants
    //
    // const can be declared in any scope, including the global scope
    //
    // constants may be set only to a constant expression,
    // not the result of a value that could only be computed at runtime
    //
    // watch naming convention
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("seconds of 3 hours: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    //
    // the first variable is shadowed by the second,
    // which means that the second variable is what the compiler will see
    // when you use the name of the variable.
    //
    // difference from marking a variable a `mut`
    // shadowing let users reuse same name again for different type.
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}")
    }

    println!("The value of y {y}");

    // it works thanks to shadowing, but mut makes error because
    // it cannot change its type
    let spaces = "     ";
    let spaces = spaces.len();
    println!("length of spaces: {spaces}");
}
