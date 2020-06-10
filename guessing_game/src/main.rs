// GUESSING GAME

use rand::Rng;
use std::cmp::Ordering;
use std::io;

// ask for user input
// process that input
// check that the input is in the expected form

// Processing a Guess
// Generating a Secret Number

fn main() {
    println!("Hola! Welcome to a guessing game! Guess the hidden number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// A  crate is a collection of Rust source code files.
// An enumeration is a type that can have a fixed set of values, and those values are called the enum’s variants.
// Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess for example. 
