use rand::Rng;
use std::io;

// Define the `GuessingGame` struct
struct GuessingGame {
    secret_number: u32,
    attempts: u32,
}

impl GuessingGame {
    // Create a new game with a random secret number
    fn new(min: u32, max: u32) -> Self {
        let secret_number = rand::thread_rng().gen_range(min..=max);
        GuessingGame {
            secret_number,
            attempts: 0,
        }
    }

    // Increment attempts and check the guess
    fn check_guess(&mut self, guess: u32) -> String {
        self.attempts += 1;
        if guess < self.secret_number {
            "Too low!".to_string()
        } else if guess > self.secret_number {
            "Too high!".to_string()
        } else {
            format!(
                "Correct! You guessed the number in {} attempts!",
                self.attempts
            )
        }
    }

    // Parse user input into a number
    fn parse_guess(input: &str) -> Result<u32, &'static str> {
        input.trim().parse::<u32>().map_err(|_| "Invalid input. Enter a number.")
    }
}

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("I'm thinking of a number between 1 and 100.");

    let mut game = GuessingGame::new(1, 100);

    loop {
        println!("Enter your guess:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match GuessingGame::parse_guess(&input) {
            Ok(guess) => {
                let feedback = game.check_guess(guess);
                println!("{}", feedback);

                if feedback.contains("Correct!") {
                    break;
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    println!("Game over! Thanks for playing.");
}

