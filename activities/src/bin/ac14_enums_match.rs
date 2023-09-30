// enums allow you to define a type by enumerating its possible variants, giving you a way to say a value is one of possible set of values
// Option enum - expresses that a value can be either something or nothing

// we can put data in each enum variant, and they can have diffrent types and amount of associated data
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// equivalent to

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// enum Option{
//     None,
//     Some(T)
// }

// let some_number = Some(5);
// let some_char = Some('e');
// Rust need the typein the case of absent_number
// let absent_number: Option<i32> = None;

// match - the values go through each pattern in a match, and at the first pattern the value "fits", the value falls in the associated code block to be used
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // the code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned from the entire match expression.
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
