fn main() {
    // because if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is: {number}");
}
