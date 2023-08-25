// String is actually implemented as a wraper around a vector of bytes with some extra guarantees, restrictions and capabilities.
fn main() {
    // create a new empty string
    let _s = String::new();
    // often, we'll have some initial data with which we want to start the string. For that we use to_string()
    let data = "initial contents";
    let s = data.to_string();
    let s2 = String::from("second contents");
    println!("string: {s}, second string: {s2}");

    // updating a string
    // A String can grow in size and its contents can change, jus like the contentns of a Vec<T>, if you push more data into it. In addition, you can use the + operator or the format! macro to concatenate String values.

    // appending to a String with push_str and push
    let mut s3 = String::from("foo");
    // the push_str method takes a string slice because we don't necessarily want to take ownership of the parameter
    s3.push_str("bar");
    // the push method takes a single character as a parameter and adds it to the String.
    s3.push('s');
    println!("s3: {s3}"); // foobars

    // concatenation with the + operator
    let s4 = String::from("Hello");
    let s5 = String::from(" world!");
    let s6 = s4 + &s5; // note that s4 has been moved here and can no longer be used

    // The reason s4 is no longer valid after the addition, and the reason we used a reference to s5, has to do with the signature of the method that's called when we use the + oprator, which uses the add method, who's signature looks like this: fn add(self, s: &str) -> String {
    // we’re adding a reference of the second string to the first string. This is because of the s parameter in the add function: we can only add a &str to a String; we can’t add two String values together.
    // The reason we’re able to use &s5 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s5 into &s5[..].
    // Second, we can see in the signature that add takes ownership of self because self does not have an &. This means s4 will be moved into the add call and will no longer be valid after that. So, although let s6 = s4 + &s5; looks like it will copy both strings and create a new one, this statement actually takes ownership of s4, appends a copy of the contents of s5, and then returns ownership of the result.
    println!("s6: {s6}");
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    // for combining strings in more complicated ways, we can use the format! macro
    let s10 = format!("{s7}-{s8}-{s9}");
    println!("s10: {s10}");
}
