fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // x = 6; not working - cannot assign twice to immutable variable(we need to make it mutable by placing mut in front)
    x = 6;
    println!("The value of x is: {}", x);
}
