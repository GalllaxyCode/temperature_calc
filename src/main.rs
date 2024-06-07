use std::io;

fn main() {
    println!("Type the temperature you want to convert! To convert Celsius to Farenheit type with a C on the end and F to convert vice a versa");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input: &str = input.trim();

    if input.ends_with("C") {
        let input: f64 = input.pop().trim().parse().expect("Write a number!");
        let formula: f64 = input * 1.8 + 32.0;
        println!("")
    }
}
