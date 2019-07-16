/* ===========================================================
=========================String Types=========================
Primitive str = Immutable fixed-length string somewhere in
memory.
String = Growable, heap-allocated data structure
Use when you need to modify or own string data.
============================================================*/

pub fn run(){
    // By default compiler is inferring primitive (immutable) type
    let primitive_str = "I am immutable!";
    let mut growable_str = String::from("I am so growable!");

    // Get Length
    println!("Str length: {}", growable_str.len());

    // Add single character
    growable_str.push('C');

    // Add more than one single character
    growable_str.push_str("ome on!");

    // Capacity in bytes
    println!("Capacity: {}", growable_str.capacity());

    // Check if the string is empty
    println!("Is Empty: {}", growable_str.is_empty());

    // Containts
    println!("Contains 'growable' {}", growable_str.contains("growable"));

    // Replace
    println!("Replace: {}", growable_str.replace("growable", "universal"));

    // Loop through string by whitespace
    for word in growable_str.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing - 1st arg is expectation, 2nd arg is a value
    // In case assertion passes, nothing happens, when it fails, runtime stops
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", growable_str);

}