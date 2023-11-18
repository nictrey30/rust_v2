#![allow(dead_code, unused_variables)]
// Generics allows us to replace specific types with a placeholder that represents mutiple types to remove code duplication

// find the largest number
fn largest_number(vector: &Vec<i32>) -> i32 {
    let mut largest = &vector[0];
    for number in vector {
        if number > largest {
            largest = number;
        }
    }
    *largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!(
        "The largest_number in list: {}",
        largest_number(&number_list)
    );
    println!(
        "The largest_number in list: {}",
        largest_number(&number_list_2)
    );
}
