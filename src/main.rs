use rand::Rng; //Crate to generate random numbers
use std::io; //Crate to take inputs 

// Entrypoint function
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    // Here we are receiving user input alongside with error handling (expect)
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}")
}
