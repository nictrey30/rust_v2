#![allow(dead_code, unused_variables)]

use std::fs::File;
// the enum io::ErrorKind has variants representing the diffrent kinds of errors that might result from an io operation
use std::io::ErrorKind;
fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    // File::open returns a Result value because the function could fail
    // The type of the value that File::open returns inside the Err variant is an io::Error, which is a struct provided by the standard library. This struct has a methond kind that we can call to get an io:ErrorKind value.
    let greeting_file_result = File::open("hello.txt");

    // this code will panic! no matter what why File::open failed
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // };

    // matching on diffrent errors
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
