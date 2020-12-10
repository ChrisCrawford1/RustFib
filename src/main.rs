use std::io;
use std::time::Instant;

fn main() {
    println!("Generate Fibonacci to the nth");

    let input = get_user_input();
    let start = Instant::now();

    println!("Fibonacci to {} = {}", input, fibonacci(input));

    let finish = start.elapsed();
    println!("It took: {} seconds to calculate", finish.as_secs_f64());
}

fn get_user_input() -> u64 {
    println!("Provide a number to iterate fibonnaci to:");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Error processing number");

    let number: u64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse input to an integer!")
    };
    return number;
}

fn fibonacci (value: u64) -> u64 {
    if value <= 1 {
        return value;
    }

    return fibonacci(value - 1) + fibonacci(value - 2);
}
