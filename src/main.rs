use std::io::{self, Write};

fn get_string() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn get_int() -> i32 {
    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Retry: Please enter a valid number."),
        }
    }
}

fn main() {
    println!("Print Number");
    let mut num1: i32 = get_int();
    println!("Print Another Number");
    let mut num2: i32 = get_int();
    println!("1. a + b");
    println!("2. a - b");
    println!("3. a * b");
    println!("4. a + b");
    let answer = get_int();
    match answer {
        1 => println!("{} + {} = {}", num1, num2, num1 + num2),
        2 => println!("{} - {} = {}", num1, num2, num1 - num2),
        3 => println!("{} * {} = {}", num1, num2, num1 * num2),
        4 => println!("{} / {} = {}", num1, num2, num1 / num2),
        _ => println!("Error"),
    }
}
