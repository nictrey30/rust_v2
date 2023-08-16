// enums - match, patterns that bind to values
// match arms can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    New_Hampshire,
    New_Jersey,
    New_Mexico,
    New_York,
    North_Carolina,
    North_Dakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    Rhode_Island,
    South_Carolina,
    South_Dakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    West_Virginia,
    Wisconsin,
    Wyoming,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be Coin::Quarter(UsState::Alaska).
            // The binding for state will be the value UsState::Alaska. We can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.
            println!("State quarter from {:?}", state);
            25
        }
    }
}
// write a function that takes an Option<i32> and, if there is a value inside, adds 1 to the value.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // if this arm matches, then i binds to  the value contained in Some
        Some(i) => Some(i + 1),
    }
}
