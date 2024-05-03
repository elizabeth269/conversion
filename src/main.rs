use std::io;

fn main() {
    println!("Enter the temperature you want to convert (e.g. 100F or 37.5C):");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input = user_input.trim().to_uppercase();

    if user_input.ends_with("F") {
        // Convert Fahrenheit to Celsius
        let fahrenheit = user_input
            .trim_end_matches("F")
            .parse::<f64>()
            .expect("Please type a number");
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!(
            "{} degrees Fahrenheit is equal to {} degrees Celsius",
            fahrenheit, celsius
        );
    } else if user_input.ends_with("C") {
        // Convert Celsius to Fahrenheit
        let celsius = user_input
            .trim_end_matches("C")
            .parse::<f64>()
            .expect("Please type a number");
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        println!(
            "{} degrees Celsius is equal to {} degrees Fahrenheit",
            celsius, fahrenheit
        );
    } else {
        println!("Invalid input. Please enter a temperature with 'F' or 'C'.");
    }
}

//we want to ask build a software that asks the user whether they are converting from "f" or "c"
// F = (C * 9/5) + 35 , C = (F - 35) + (9/5)
