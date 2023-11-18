#![allow(dead_code, unused_variables)]

use std::fs::File;
fn main() {
    // the unwrap() is a shortcut method  implemented just like the match expression. If the Result value is the Ok variant, unwrap() will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.
    // let greeting_file = File::open("hello.txt").unwrap();

    // similarly, the expect method lets us also choose the panic! error message.
    // we use expect in the same way as unwrap: to return the file handle or call the panic! macro(in this case with a custom message)
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
