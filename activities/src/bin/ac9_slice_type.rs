// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection
// A slice is a kind of reference, so it does not have ownership.

fn main() {
    // a string slice is a reference to a part of a String
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!,   -- immutable borrow occurs first_word(&s), and mutable borrow occurs s.clear()
}
// write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
// if there is no space, the whole string should be returned

// the first_word function has a parameter &String(we don't want ownership)
fn first_word(s: &String) -> &str {
    // because we need to go through the String element by element and check whether a value is a space,
    // we'' convert our String to an array of bytes
    let bytes = s.as_bytes();

    // the iter method returns each element in a collection, and enumerate wraps the result of iter and returns each element as a part of a tuple instead
    // first tuple is the index, and the second element is a reference to the element
    for (i, &item) in bytes.iter().enumerate() {
        // search for the byte that represents the space(by using the byte literal syntax)
        if item == b' ' {
            // when we find a space, we return a string slice
            return &s[0..i];
        }
    }

    &s[..]
}
