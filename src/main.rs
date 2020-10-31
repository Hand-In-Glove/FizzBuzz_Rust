use std::io;
mod fizz_buzz;

fn main() {
    println!("Welcome, please enter a number");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read that input");

    let input: i32 = input
        .trim()
        .parse()
        .expect("Please make sure you entered a NUMBER");

    fizz_buzz::run(input);
}
