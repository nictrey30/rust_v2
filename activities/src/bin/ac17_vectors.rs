// vectors allows you to store more than one value in a single data structure that puts all the values next to each other in memory.
// vectors can only store values of the same type, they are useful when you have lists of items
fn main() {
    // More often, you'll create a vec<T> with initial values and Rust will infer the type of value you want to store
    let _v1: Vec<i32> = Vec::new();
    // vec! macro will create a new vector that creates the values you give it
    let mut v2 = vec![1, 2, 3, 4, 5];
    // using push to add values to a vector
    v2.push(6);
    // reading elements of vectors
    // there are 2 ways to reference a value stored in a vector: via indexing or by using the get method.
    // This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.
    // When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid. Recall the rule that states you can’t have mutable and immutable references in the same scope.

    let third: &i32 = &v2[2];
    println!("The third element of v2 is : {third}");
    // You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element of v2 is : {third}"),
        None => println!("There is no third element"),
    }

    // iterating over values in a vector, using a for loop to get immutable references to each element in a vector
    for i in &v2 {
        println!("{i}");
    }
    // we can also iterate over mutable references to each element in order to make changes to all the elements
    for i in &mut v2 {
        // to change the value that the mutable reference refers to, we have to use the * deference operator to get to the value in i before we can use the += operator.
        *i += 50;
    }

    // Using an enum to store multiple types
    // Vectors can only store values that are of the same type. Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of diffrent types, we can define and use an enum.
    #[derive(Debug)]
    // Rust needs to know what types will be in the vector at compile time so it knows exactly how much mem on the heap will be needed to store each element
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }

    // dropping a vector drops its elements
    {
        let _v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope here and its elements are dropped
}
