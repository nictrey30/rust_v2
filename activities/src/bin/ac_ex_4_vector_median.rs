// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list

use rand::Rng;
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let mut my_list: Vec<i32> = Vec::new();
    if return_answer() == true {
        let num_elements = number_of_integers();
        println!("The num of elements is {num_elements}");
        for _i in 0..num_elements {
            my_list.push(rand::thread_rng().gen_range(-100..=100));
        }
        println!("The vector elements:");
        for i in &my_list {
            println!("{i}");
            io::stdout().flush().expect("Something went wrong");
        }
    }
    // println!("The median of the vector: {}", vector_median(&my_list));
    // vector_mode(&my_list);
}
fn number_of_integers() -> u32 {
    println!("Choose a number of integers to generate the list (1..20)");
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
fn return_answer() -> bool {
    let mut answer = String::new();
    println!("Do you want to generate a vector?(y/n)");
    loop {
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line!");
        // the chars() method of a string returns an iterator of characters which make up a string. The iterator returned allows us to loop through each character of the string
        if answer.trim().chars().count() < 2 {
            let char_vec: Vec<char> = answer.trim().chars().collect();
            let ch = char_vec[0];
            println!("current: {ch}");
            match ch {
                'n' => return false,
                'y' => return true,
                _ => {
                    println!("Please input y/n only!");
                    answer.clear();
                    continue;
                }
            };
        } else {
            println!("please don't enter more than 1 character!");
            answer.clear();
            continue;
        }
    }
}
fn vector_median(vect: &Vec<i32>) -> i32 {
    let mut vect_clone = vect.clone();
    let median: usize;
    median = vect.len() / 2;
    vect_clone.sort();
    vect_clone[median]
}
fn vector_mode(vect: &Vec<i32>) {
    let mut map = HashMap::new();
    for i in vect {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
