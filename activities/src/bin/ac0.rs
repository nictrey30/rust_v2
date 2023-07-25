fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // x = 6; not working - cannot assign twice to immutable variable(we need to make it mutable by placing mut in front)
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing - you can declare a new variable with the same name as the previous variable
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y); // 12
    }
    println!("The value of y is: {}", y); // 6
}
