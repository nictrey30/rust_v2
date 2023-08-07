// Return Values and Scope
// returning values can also transfer ownership

fn main() {
    let s1 = gives_ownership(); // gives_ownership() moves its return value into s1
    println!("s1: {s1}");

    let s2 = String::from("hello"); // s2 comes into scope

    // s2 is moved into takes_and_gives_back(), which also move its returned value into s3
    let _s3 = takes_and_gives_back(s2);
}

// gives_ownership() will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out of the calling function
}
// this function takes a String and returns a String
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out of the calling function
}
