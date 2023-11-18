#![allow(dead_code, unused_variables)]

// to define the generic largest function, we place the name declarations inside <>, between the name of the function and the parameter list

// the function largest is generic over some type T
// we restrict the types valid for T to only those that implement PartialOrd and this example will compile, because the standard library implements PartialOrd on both i32 and char.

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        // because we want to compare values of type T, we can only use types whose values can be ordered
        // To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types
        if item > largest {
            largest = item;
        }
    }
    largest
}

// first we declare the name of the type parameter inside <> just after the name of the struct. Then we use the generic type in the struct definition where we would otherwise specify concrete data types
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    // the fields x and y must be the same type because both have the same generic data type T
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
