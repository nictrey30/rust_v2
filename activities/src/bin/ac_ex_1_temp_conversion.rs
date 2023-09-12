use std::io;

// convert temp between F and C
fn main() {
    let mut temp = String::new();
    println!("Please enter the temperature as a number: ");
    'conversion: loop {
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read the line");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only a number!");
                temp.clear();
                continue;
            }
        };

        let mut input = String::new();
        loop {
            println!("choose conversion: celsius(c)/fahrenheit (f)");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line.");

            // the chars() method of a string returns an iterator of characters which make up a string. The iterator returned allows us to loop through each character of the string
            if input.trim().chars().count() < 2 {
                let char_vec: Vec<char> = input.trim().chars().collect();
                let ch = char_vec[0];

                match ch {
                    'c' => {
                        let temp_in_fahr: f64 = (temp * (9 as f64 / 5 as f64)) + 32 as f64;
                        println!("The temp {temp}C is in fahrenheit: {temp_in_fahr}F ");
                        break 'conversion;
                    }
                    'f' => {
                        let temp_in_celsius: f64 = (temp - 32 as f64) * 5 as f64 / 9 as f64;
                        println!("The temp {temp}F is in celsius: {temp_in_celsius}C ");
                        break 'conversion;
                    }
                    _ => {
                        println!("Please input 'c' or 'f' only!");
                        input.clear();
                        continue;
                    }
                }
            } else {
                println!("please don't enter more than 2 characters!");
                input.clear();
                continue;
            }
        }
    }
}
