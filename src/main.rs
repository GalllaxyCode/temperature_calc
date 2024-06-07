use std::io::{self, Empty};

fn main() {
    println!("Type the temperature you want to convert! To convert Celsius to Farenheit type with a C on the end and F to convert vice versa");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input = input.trim();

    if input.ends_with("C") {
        let input: String = input.chars().take(input.len()-1).collect();
        let input: f64 = input.parse().expect("Failed to convert input into a number!");
        let formula: f64 = input * 1.8 + 32.0;

        println!("{input} Celsius is {formula} Farenheit.")

    } else if input.ends_with("F") {
        let input: String = input.chars().take(input.len()-1).collect();
        let input: f64 = input.parse().expect("Failed to convert input into a number!");
        let formula: f64 = (input - 32.0) / 1.8;
        println!("{input} Farenheit is {formula} Celsius.")
    }
}
