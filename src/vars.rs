// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // string works
    let name = "Brad";

    println!("My name is {name}", name = &name);

    // int works
    let age = 37;

    println!("I am {}", age);

    // shadow
    // * shadowing is different from marking a variable as mut because we'll get compile-time error
    //   if we accidentally try to reassign to this variable without using the let keyword
    {
        let age = age * 2;
        println!("I am {age}, age doubled");
    }

    println!("But back to original age: {age}");

    // but if changed
    // age = 38;
    // make it mutable => mut
    let mut new_age: i32 = 10;
    new_age = 110;
    println!("I am {}", new_age);

    // const keyword
    // - isn't used too much
    const ID: i32 = 001; // upper case const
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!(
        "My Name: {name}, My Age: {age}",
        name = my_name,
        age = my_age
    );
}
