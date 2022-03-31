// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    // string works
    let name = "Brad";

    println!("My name is {name}",name=&name);

    // int works
    let age = 37;

    println!("I am {}", age);

    // but if changed
    // age = 38;
    // make it mutable => mut
    let mut new_age = 10;
    new_age = 110;
    println!("I am {}", new_age);

    // const keyword
    // - doesn't use to much
    const ID: i32 = 001;  // upper case const  
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age ) = ("Brad", 37);
    println!("My Name: {name}, My Age: {age}", name=my_name, age=my_age);

}