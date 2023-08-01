fn main() {
    // passing a variable to a function will move or copy, just as assignment does

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function..
                        // ... and is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); //x would move into function, but i32 is Copy, so it's oc to still use x afterward
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // here, some_string goes out of scope and drop is called

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // here, some_integer goes out of scope. Nothing special happens
