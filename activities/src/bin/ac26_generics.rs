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

// generics in method definitions
// Note that we have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
impl<T> Point<T> {
    // implementing a method named x on the Point<T> struct that will return a reference to the x field of type T
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also specify constraints on generic types when defining methods on the type. We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type, meaning we don’t declare any types after impl.
impl Point<f32> {
    // This code means the type Point<f32> will have a distance_from_origin method; other instances of Point<T> where T is not of type f32 will not have this method defined.
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    // the fields x and y must be the same type because both have the same generic data type T, if the struct Point is declared only with Point<T>
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("p.x = {}", integer.x());

    // To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters.
    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };
}
