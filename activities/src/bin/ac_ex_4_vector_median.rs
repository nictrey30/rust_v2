// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list

use rand::Rng;
use std::io;
// use std::cmp::Ordering;
fn main() {
    let num_elements = number_of_integers();
    println!("The num of elements is {num_elements}");
    let mut my_list: Vec<i32> = Vec::new();
    for _i in 0..num_elements {
        my_list.push(rand::thread_rng().gen_range(-100..=100));
    }
    // for i in &my_list {
    //     println!("{i}");
    // }
}
fn number_of_integers() -> u32 {
    println!("Choose a number of integers to generate the list(between 1..100");
    loop {
        let mut num_integers = String::new();
        io::stdin()
            .read_line(&mut num_integers)
            .expect("Failed to readline!");
        let num_integers: u32 = match num_integers.trim().parse() {
            Ok(num) => {
                if num >= 20 {
                    println!("error: please input a number 0..20");
                    num_integers.clear();
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("try again");
                num_integers.clear();
                continue;
            }
        };
        return num_integers;
    }
}
