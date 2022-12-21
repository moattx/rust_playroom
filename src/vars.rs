/*
 * Variables hold primitive data or references to data
 * Variables are immutable by default (CONST)
 * Rust is a block-scoped language
 */

pub fn run() {
    let name = "Brad"; // this is const or immutable

    //In Rust, the default integer type is i32 and the default float type is f64.
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001; /* outputs 1 for some reason probably println? */
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
