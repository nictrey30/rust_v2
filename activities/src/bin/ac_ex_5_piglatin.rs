// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
use std::io;
const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
fn main() {
    let phrase = read_string();
    for slice in phrase.split_whitespace() {
        println!("{}", transform_word(slice.to_owned()))
    }
}
fn read_string() -> String {
    println!("Please enter your phrase: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input.trim().to_string()
}
fn transform_word(mut word: String) -> String {
    let ch = word.chars().next().unwrap();
    // or
    // let ch = text.chars().nth(0).unwrap();
    if VOWELS.contains(&ch) {
        return word + "-hay";
    } else {
        word.remove(0);
        return word.to_owned() + &format!("-{}ay", ch);
    }
}
