// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list
use rand::Rng;
use std::collections::HashMap;
use std::io;

fn main() {
    let my_list: Vec<i32>;
    if return_answer() == true {
        my_list = generate_vector();
    } else {
        my_list = input_vector();
    }
    print_vector(&my_list);
    println!("The median of the vector: {}", vector_median(&my_list));
    vector_mode(&my_list);
}
fn num_elements() -> u32 {
    println!("Choose a number of integers to generate the list (1..20)");
    loop {
        let mut num_integers = String::new();
        io::stdin()
            .read_line(&mut num_integers)
            .expect("Failed to readline!");
        let num_integers: u32 = match num_integers.trim().parse() {
            Ok(num) => {
                if num > 20 {
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
    #[derive(Debug)]
    struct VectorMode {
        element: i32,
        apperances: i32,
    }
    let mut max: VectorMode = VectorMode {
        element: 0,
        apperances: 0,
    };
    let mut num_of_apperances: i32 = 0;
    let mut map = HashMap::new();
    for i in vect {
        // the or_insert method returns a mutable reference (&mut v) to the value for the specified key.
        let count = map.entry(i).or_insert(0);
        // we store the returned mutable reference in the count variable, so in order to assign to that value, we must first deference it.
        *count += 1;
    }
    println!("The vector elements freq: {:?}", map);
    for (key, value) in map {
        if value > num_of_apperances {
            max = VectorMode {
                element: key.clone(),
                apperances: value,
            };
            num_of_apperances = value;
        }
    }
    println!(
        "Mode of the vector => element: {:?}, apperances: {:?}",
        max.element, max.apperances
    );
}
fn generate_vector() -> Vec<i32> {
    let mut my_list: Vec<i32> = Vec::new();
    let num_elements = num_elements();
    println!("The num of elements is {num_elements}");
    for _i in 0..num_elements {
        my_list.push(rand::thread_rng().gen_range(-100..=100));
    }
    my_list
}
fn check_input_is_integer() -> i32 {
    let mut input = String::new();
    let number: i32 = loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line.");
        let input: i32 = match input.trim().parse() {
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
fn input_vector() -> Vec<i32> {
    // read a num of elements from input 1..20
    let num = num_elements();
    // the vector to be generated;
    let mut my_list: Vec<i32> = Vec::new();
    // read num_elements elements, check the element and if valid push it to the vector
    for i in 0..num {
        println!("input the {i} element out of {num}:");
        let new_elem = check_input_is_integer();
        my_list.push(new_elem);
    }
    // return the generated vector
    my_list
}
fn print_vector(v: &Vec<i32>) {
    println!("The vector elements: ");
    for i in v {
        println!("{i}");
    }
}
