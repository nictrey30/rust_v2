// Ownership Rules:
// Each value in Rust has an owner.
// There can be only one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// scope - the range within a program for which an item is valid.

fn main() {
    // you can create a String from a string literal using the from function
    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}");

    // When a variable goes out of scope, Rust calls automatically a special function for us at the curly bracket: drop
    // a String is made up of 3 parts: a pointer to the memory that holds the content, length and capacity. This group of data is stored on the stack.
    // The memory that holds the content is stored on the heap

    let s1 = String::from("hello");
    // when we assign s2 to s1, the String metadata is copied, but not the data on the heap.
    // Both data pointers point to the same location on the heap.
    // when s1 and s2 goes out of scope, they will both try to free the same memory(bug aka double free error)

    let s2 = s1; // value moved here

    // to ensure memory safety, after the line s2 = s1; Rusts considers s1 no longer valid
    // println("{s1}"); - won't work

    // move - instead of a shallow copy, because Rust also invalidates the data

    // clone - if we do want to deeply copy the heap data, not just the stack data
    let _s3 = s2.clone();

    // if a type implements the Copy trait, variables that use it do not move, making them still valid after assignment to another variable.
    // Rust won't lt us annotate atype with Copy if the type, or any of its parts, has implemented the Drop trait.
}
