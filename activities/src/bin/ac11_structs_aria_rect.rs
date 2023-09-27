// adding the attribute to derive debug trait and printing the Rectangle instance using debug formatting
#[derive(Debug)]
// calculate the area of a rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = build_rectangle(30, 50);
    println!("The aria of rect1 is : {}", calculate_area(&rect1));
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    // the dbg! macro takes ownership of an expression(as opposed to println!, which takes a reference), prints to error console stream,
    // then returns ownership of the value
    let scale = 2;
    let rect2 = Rectangle {
        // dbg! returns ownership of the expression value
        width: dbg!(30 * scale),
        height: 50,
    };
    // we don't want to take ownership of rect2, so we use a reference to rect2
    dbg!(&rect2);
}

fn build_rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}
// we want to borrow the struct rather than take ownership of it
// This way main retains its ownership and can continue using rect1
fn calculate_area(rectangle: &Rectangle) -> u32 {
    // accessing field of borrowed struct instance does not move the field values
    rectangle.width * rectangle.height
}
