// struct methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// unlike functions, methods are defined within the context of a struct, and their first parameter is always self, which represents the instance of the struct the method its being called on.
impl Rectangle {
    // &self short for self: &Self
    // methods can take ownership of self, borrow self immutably, or borrow self mutably
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    // getters are useful because you can make the field private, but the method public, and thus enable read-only access to that field as a part of the type's public API.
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("the area of the rectangle is {}", rect1.area());
    if rect1.width() {
        println!("The rectangle has a nonzero width of {}", rect1.width);
    }
}

// when you call a method with object.somethiong(), Rust automatically adds in &, &mut or * so object matches the signature of the method
