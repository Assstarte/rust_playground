// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language
// Variable naming convention is underscore - my_variable

pub fn run() {
    let name = "Max";
    let mut age = 20;
    println!("Just in case you forgot, my name is {} and I am {} years old", name, age);
    age = 21; // Yay! Birthday.
    println!("Happy Birthday! I am {} years old now", age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("Max", 21);
    println!("{} is {}", my_name, my_age);
}