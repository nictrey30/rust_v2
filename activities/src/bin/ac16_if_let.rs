fn main() {
    let num = Some(10);
    match num {
        Some(value) => println!("The num is {value}"),
        _ => (),
    }
    // the if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
    // the if let syntax takes a pattern and an expression separated by equal sign.
    // you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    if let Some(value) = num {
        println!("The num is {value}");
    }
}
