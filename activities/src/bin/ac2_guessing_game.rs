use std::io;
// The "Rng" trait defines methods that random numbers generators implement, and this trait must be in scope for us to use those methods.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // generating a random number
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("The secret number is {secret_number}");

    println!("Please input your guess:");

    // String::new() -> returns a new instance of the growable String type
    let mut guess = String::new();

    // reading from the line
    // read_line() returns a "Result" value(an enum), which is a type that can be in one of multiple states(variants)
    io::stdin()
        // the & indicates that "guess" argument is a reference, which gives you a way to let multiple parts of your code to access
        // one piece of data without needing to copy the data into memory multiple times.
        // Like variables, references are immutable by default
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {guess}");
}
