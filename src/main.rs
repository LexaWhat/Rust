fn main() {
    let name: &str = "Alex";
    println!("{}", char_dis(name, 2));
}

fn char_dis(x: &str, y: usize) -> char {
    x.chars().nth(y).unwrap_or('_')
}
