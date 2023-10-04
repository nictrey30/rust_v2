use std::fs::File;
// the enum io::ErrorKind has variants representing the diffrent kinds of errors that might result from an io operation
use std::io::ErrorKind;
fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    // File::open returns a Result value because the function could fail
    // The return type of File::open is a Result<T, E>. The generic parameter T has been filled in by the implementation of File::open with the type of the success value, std::fs::File, which is a file handle. The type of E used in the error value is std::io::Error.
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
