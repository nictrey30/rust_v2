fn main() {
    // shadowing

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x in the inner scope = {}", x);
    }
    println!("x in the outer scope = {}", x);

    // compound types - tuples and arrays
    // tuples have a fixed length; once declared they cannot grow or shrink in size
    let tup = (500, 1.45, true);
    let first_tup_el = tup.0;
    let second_tup_el = tup.1;

    // destructuring
    let (x, _y, z) = tup;
    println!("The value of x from the tuple is {}", x);

    // accesing a tuple by . notation
    println!("The value of z from the tuple is : {}", z);

    // the array type, unlike the tuple, every element of the array has the same type and a fixed length
    // arrays are allocated on the stack

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let first_array_el = a[0];
    let second_array_el = a[1];

    // a vector is a similar collection to an array that is allowed to grow or shrink in size
}
