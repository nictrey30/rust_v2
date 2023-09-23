// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
use std::collections::HashMap;
use std::io;
fn main() {
    let phrase = read_string();
    println!("{phrase}");
}
fn read_string() -> String {
    println!("Please enter your phrase: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input
}
