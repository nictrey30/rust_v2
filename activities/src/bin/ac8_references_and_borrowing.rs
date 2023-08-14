// what if we want to let a function use a value but not take ownership?

// a reference is like a pointer in that it's an address we can follow to access the data stored at the address; that data is owned by another variable.

// the & represent references, and allow you to refer to some value without taking ownership of it
fn main() {
    let s1 = String::from("hello");

    // the &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    // Because it does not own it, the value it points to will not be dropped when the reference stops being used.
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // here s goes out of scope. But because it does not have ownership of what it refers to, the String is not dropped.

// when functions have references as parameters instead of the actual values, we won't need to return the values in order to give back ownership, because we never had ownership.
// we call the action of creating a reference borrowing. If we try to modify something we're borrowing it won't work.
// just as variables are immutable by default, so are references. We are not allowed to modify something we have a reference to.
