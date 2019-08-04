use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Command not specified. Aborted.");
        std::process::exit(0);
    }
    let command = args[1].clone();
    let name = "Max";
    let status = "100%";


    println!("Args: {:?}", args);
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name)
    } else if command == "status" {
        println!("Status: {}", status);
    } else {
        println!("{} is not a valid command.", command);
    }
}