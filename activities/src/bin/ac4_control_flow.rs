fn main() {
    // because if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is: {number}");

    // loop
    // break - placed within th loop to stop executing the loop
    // continue - tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration

    // returning values from loops
    // one of the other uses of a loop is to retry an operation you know might fail. You might also need to pass the result
    // of the operation out of the loop to the rest of your code.
    // To do this, you can add the value you want returned after the break expression you use to stop the loop
    let mut counter_1 = 0;
    let result = loop {
        counter_1 += 1;
        if counter_1 == 10 {
            break counter_1;
        }
    }; // after the loop we use a semicolon to end the statement that assigns the value to "result"
    println!("The result is {result}");
}
