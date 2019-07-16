pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Hello, my name is {}, I am from {}.", "Max", "Kharkiv");

    // Positional Arguments
    println!("Hello, my name is {0}. I am from {1}. Yes, you heard it right, my name is {0}", "Max", "Kharkiv");

    // Named Arguments
    println!("I am {age} years old. I love {lang} programming language!", age = 20, lang = "Rust");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello!"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}