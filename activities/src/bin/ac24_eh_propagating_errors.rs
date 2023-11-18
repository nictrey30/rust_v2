#![allow(dead_code, unused_variables, unused_imports)]

use std::error::Error;
// when a function's implementation calls something that might fail, instead of handling the error within the function itself,
// you can return the error to the calling code so that it can decide what to do.
use std::fs::File;
use std::io::{self, Read};
fn main() {}

// if this functions succeeds, the function will return an Ok vaklue that holds a String - the username
// if the function encounters any problems, it will return an Err value that holds an instance of io::Error
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     // we don't need to explicitly say return, because this is the last expression in the function
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// implementation usin the ? operator
fn read_username_from_file() -> Result<String, io::Error> {
    // The ? placed after a Result value:
    // if the value is an Ok, the value inside the Ok will get returned from this expression and the program will continue.
    // if the value is an Err, the Err will be returned from the whole function as if we have used the return keyword
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// using fs::read_to_string("hello.txt")
