use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Randomly generate a secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    // Prompt for the user to guess the number
    loop {
        let mut guess = String::new();
        println!("Guess the secret number:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Parse and convert user input.
        // let guess: u32 = guess.trim().parse().expect("Guess the secret number:");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Error: {}. Please try again.", error);
                continue;
            }
        };

        // Compare guess to secret number
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
