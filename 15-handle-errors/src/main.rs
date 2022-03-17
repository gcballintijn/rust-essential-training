use std::io;
use rand::prelude::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        if let Err(_) = io::stdin().read_line(&mut guess) {
            println!("Failed to read input line.");
            continue;
        }
        
        let guess = match guess.trim().parse::<u32>() {
            Ok(guess) => guess,
            Err(_) => {
                println!("Failed to parse the guess.");
                continue;
            }
        };

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }    
}
