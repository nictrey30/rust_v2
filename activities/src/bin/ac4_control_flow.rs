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

    // loop labels
    // if you have loops within loops, break apply to the most inner loop at that point.
    // You can optionally specify a loop label to break/continue that specified loop

    let mut counter_2 = 0;
    'counting_up: loop {
        println!("count: {counter_2}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if counter_2 == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter_2 += 1;
    }
    println!("End count: {counter_2}");

    // while - while condition evaluates to "true", the code runs; otherwise it exits the loop
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    // for loops
    let a = [10, 20, 30, 40, 50];
    for el in a {
        println!("the value is: {el}");
    }

    println!("using for loop with range:");
    for number in (1..5).rev() {
        println!("{number}");
    }
}
