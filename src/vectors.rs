pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3 ,4];

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    println!("{:?}", numbers);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?} Numbers vector: ", numbers)
}