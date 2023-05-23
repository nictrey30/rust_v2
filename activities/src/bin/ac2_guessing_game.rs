use std::io;
// The "Rng" trait defines methods that random numbers generators implement, and this trait must be in scope for us to use those methods.
use rand::Rng;
// like Result, Ordering is another enum with the variants: Less, Greater, Equal
// the cmp method compares two values and it takes a reference to whatever you want to compare with.
// Then it returns a variant of the Ordering enum we brought into the scope with the use statement
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // generating a random number
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
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

        // "shadowing" lets us reuse the guess variable name rather than forcing us to create two uniquw variables,
        // such as guess_str and guess
        // the parse method converts a string to another type and we need to tell Rust the exact type number we want by using u32
        // the call to parse could easily return an error and because of that the parse method returns a Result type
        // if parse returns Err, the expect call will crash the game and print the msg,
        // expect will return the number that we want from the Ok value
        // but if successful, let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
