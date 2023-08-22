// struct - custom data type that lets you package together and name multiple related values that make up a meaningful group
struct User {
    active: bool,
    // in the User struct definition, we used the owned String rather than athe string slice type .&str
    // this is deliberate, because we want each instance of this struct to own all of its data
    // it is also possible for structs to store references to data owned by something else, but to do so require the use of lifetimes
    username: String,
    email: String,
    sign_in_count: u64,
}
// using tuple structs without named fields to create diffrent types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// unlike structs - can be useful when you need to implement a trait on some type but you don't have any data that you want to store in the type itself
struct AlwaysEqual;

fn main() {
    // creating an instance of the struct
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // in order to change the value in a field, the whole instance must be mutable
    // Rust doesn't allow us to mark only certain fields as mutable
    user1.email = String::from("anotheremail@example.com");

    // creating instances from other instances with Struct Update Syntax
    // in this example we can no longer use user1 after creating user2 because the string in the username field of user1 was moved into user2
    let user2 = User {
        email: String::from("anacumere@example.com"),
        // the ..user1 must come last to specify that any remaining fields should get their values from user1
        ..user1
    };

    // tuple structs - useful when you want to give the whole tuple a name and make the tuple a diffrent type from other tuples,
    // and when naming each field would be redundant
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // uni-like struct
    let subject = AlwaysEqual;
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
