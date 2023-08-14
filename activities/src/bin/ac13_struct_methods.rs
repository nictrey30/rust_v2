// struct methods with more parameters
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // the method can_hold checks if a Rectangle can hold another Rectangle;
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // we can define associated functions that don't have self as the first parameter(and thus are not methods) because they don't need an instance of the type to work with.
    // They are often used for constructors that will return a new instance of the struct
    fn square(size: u32) -> Self {
        // the Self are aliases for the type that appears after impl(Rectangle in this case)
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("the area of rec1 is {}", rect1.area());
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let square1 = Rectangle::square(40);
    println!("The square1 is {:?}", square1);
}
