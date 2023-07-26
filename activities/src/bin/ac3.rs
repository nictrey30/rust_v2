fn main() {
    another_fn(5); // The value of x is: 5
    print_label_measurement(3, 'm');

    // a new scope block is an expression
    let x = {
        let y = 3;
        y + 1 // this is an expression(does not include a semicolon, if it did it would be a statement, and then it will not return a value)
    };
    println!("The value of x is: {x}"); // The value of x is: 4

    let x1 = plus_one(9);
    println!("The value of x1 taken from a return value fn is : {x1}");
}

// in function signatures, you must declare the  type of each parameter
fn another_fn(x: i32) {
    println!("The value of x is: {x}");
}
fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// statements - are instructions that perform some action and do not return a value
// expressions - evaluate to a resultant value.
// ex of expressions: calling a function, calling a macro, a new scope block created with {} is an expression

// functions with return values
fn plus_one(x: i32) -> i32 {
    x + 1
}
