// struct - custom data type that lets you package together and name multiple related values that make up a meaningful group
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
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
        ..user1
    };
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
