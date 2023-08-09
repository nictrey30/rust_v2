// just as variables are immutable by default, so are references. We are not allowed to modify something we have a reference to.
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    // mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references(mutable or immutable) to that value
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems

    // a reference's scope starts from where it is introduced and continues through the last time the reference is used
    let r2 = &s; // no problem
    let r3 = &s; // no problem
                 // let r4 = &mut s; // BIG PROBLEM
    println!("r2: {r2} and r3: {r3}");
    // variables r2 and r3 will no longer be used after this point, their scope end here where they are last used

    let r4 = &mut s; // no problem
    println!("r4: {r4}");
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// The Rules of References
// At any given time, you can have either one mutable reference or any number of immutable references.
//  References must be always be valid.
