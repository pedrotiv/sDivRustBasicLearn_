
use std::io;

pub fn run() {
    // It convert temperatures between C, F or K
    let mut temp_in: f64 = 0.0;
    let mut temp_out: f64 = 0.0;
    let mut unit_in: char = 'f';
    let mut unit_out: char = 'c';
    let mut enter_str = String::new();

    loop{
        println!("Imput temperature to convert and input and output units.");
        println!("for exemple: 41.2 C F, it will convert the 41.2 Celsius to Fahrenheit");
        println!("default condition: input: f output: c (if we imput only a number");
        io::stdin().read_line(&mut enter_str).expect("Failed to read line");
        let mut inter = enter_str.split_whitespace();
    }

}