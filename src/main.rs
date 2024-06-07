use std::io::{self, Empty};

fn main() {
    println!("Type the temperature you want to convert! To convert Celsius to Farenheit type with a C on the end and F to convert vice versa");
loop {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input = input.trim();

    if input.ends_with("C") {
        println!("Print a first letter of temperature you want to convert to.");
        let mut input2 = String::new();

        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line!");

        let input2 = match input2.chars().next() {
            Some(c) => c,
            None => {
                println!("No characters entered! Please enter a character!");
                return;
            }
        };

        if input2 == 'F' {
            let input: String = input.chars().take(input.len() - 1).collect();
            let input: f64 = input.parse().expect("Failed to convert input into a number!");
            let formula: f64 = input * 1.8 + 32.0;

            println!("{input} Celsius is {formula} Farenheit.");
        } else if input2 == 'K' {
            let input: String = input.chars().take(input.len() - 1).collect();
            let input: f64 = input.parse().expect("Failed to convert input into a number!");
            let formula: f64 = input + 273.15;

            println!("{input} Celsius is {formula} Kelvin.")
        }

    } else if input.ends_with("F") {
        println!("Print a first letter of temperature you want to convert to.");
        let mut input2 = String::new();

        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line!");

        let input2 = match input2.chars().next() {
            Some(c) => c,
            None => {
                println!("No characters entered! Please enter a character!");
                return;
            }
        };

        if input2 == 'C' {
            let input: String = input.chars().take(input.len() - 1).collect();
            let input: f64 = input.parse().expect("Failed to convert input into a number!");
            let formula: f64 = (input - 32.0) / 1.8;
            println!("{input} Farenheit is {formula} Celsius.");
        } else if input2 == 'K' {
            let input: String = input.chars().take(input.len() - 1).collect();
            let input: f64 = input.parse().expect("Failed to conver input into a nubmer!");
            let formula: f64 = (input - 32.0) / 1.8 + 273.15;
            println!("{input} Farenheit is {formula} Kelvin.")
        }
        
    } else if input.ends_with("K") {
        println!("Print a first letter of temperature you want to convert to.");
        let mut input2 = String::new();

        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read line!");

        let input2 = match input2.chars().next() {
            Some(c) => c,
            None => {
                println!("No characters entered! Please enter a character!");
                return;
            }
        };

        if input2 == 'C' {
            let input: String = input.chars().take(input.len() - 1).collect();
            let input: f64 = input.parse().expect("Falied to convert input into a number!");
            let formula: f64 = input - 273.15;
            println!("{input} Kelvin is {formula} Celsius.");
        } else if input2 == 'F' {
            let input: String = input.chars().take(input.len() - 1).collect();
            let input: f64 = input.parse().expect("Failed to convert input into a number!");
            let formula: f64 = (input - 273.15) * 1.8 + 32.0;
            println!("{input} Kelvin is {formula} Farenheit.")
        } else {
            println!("Print a valid character!")
        }
        
    } 
    
    else {
        println!("Wrong input! Remember to print C on the end for Celsius, F for Farenheit, K for Kelvin");
    }
}
    
}
