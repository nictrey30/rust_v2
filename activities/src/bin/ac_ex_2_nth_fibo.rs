use std::io;

fn main() {
    println!("please input a number between 0 and 100");
    let nth_fibo: u32 = read_input();
    let result = calc_fibo(nth_fibo);
    println!("The {nth_fibo}th fibo number is {result}");
}
fn read_input() -> u32 {
    let mut input = String::new();
    let number: u32 = loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line.");
        let input: u32 = match input.trim().parse() {
            Ok(num) => {
                if num > 100 {
                    println!("please input a number between 0 and 100 only!");
                    input.clear();
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("please input a number between 0 and 100 only!");
                input.clear();
                continue;
            }
        };
        break input;
    };
    number
}
fn calc_fibo(num: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut current: u32 = 2;
    let mut old: u32 = 0;
    let mut new: u32 = 1;

    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    } else {
        while current < num {
            sum = old + new;
            old = new;
            new = sum;
            current += 1;
        }
    }
    sum
}
