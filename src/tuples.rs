// Tuples group together values of different types
// Max 12 elements

pub fn run(){
    let person: (&str, &str, i8) = ("Max", "Aiden", 20);

    println!("{} {} is {} years old.", person.0, person.1, person.2);
}