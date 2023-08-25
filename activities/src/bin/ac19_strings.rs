fn main() {
    // indexing into Strings is often a bad idea because it's not clear what the return type of the string-indexing operation should be: a byte value, a character, a graphene cluster, or a string slice.
    // rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes
    let hello = "Здравствуйте";
    // here s will be a &str that contains the first 4 bytes of the string. each of these characters was two bytes, which means s will be Зд
    // you should use caution when creating string slices with ranges, because doing so can crash your program.
    let s = &hello[0..4];
    println!("slice using range indexing: {s}");

    // Methods for iterating over Strings
    // The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes. bytes. For individual Unicode scalar values, use the chars method. Calling chars on “Зд” separates out and returns two values of type char, and you can iterate over the result to access each element:
    for c in "Зд".chars() {
        println!("{c}");
    } // З д
      // alternatively, the bytes method returns each raw byte:
    for b in "Зд".bytes() {
        println!("{b}");
    } // 208 151 208 180

    // Be sure to remember that valid Unicode scalar values may be made up of more than one byte.
}
