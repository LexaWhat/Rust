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
    let name: &str = "Alex";
    println!("{}", char_dis(name, 2));
}

fn char_dis(x: &str, y: usize) -> char {
    x.chars().nth(y).unwrap_or('_')
}
