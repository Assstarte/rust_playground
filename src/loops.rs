// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count0 = 0;
    let mut count1 = 0;

    // Infinite loop
    println!("Infinite loop (with break condition)");
    loop {
        count0 += 1;
        println!("Number: {}", count0);

        if count0 == 20 { break };
    }

    // While loop (FizzBuzz example)
    println!("While loop");
    while count1 <= 100 {
        if count1 % 15 == 0 {
            println!("fizzbuzz")
        } else if count1 % 3 == 0 {
            println!("fizz")
        } else if count1 % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count1)
        }
        count1 += 1;
    }

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz")
        } else if x % 3 == 0 {
            println!("fizz")
        } else if x % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", x)
        }
    }

}