use std::collections::HashMap;
fn main() {
    // Updating a hash map
    // Each unique key can have one value associated with it at a time. When you want to change the data in a hash map, you have to decide how to handle the case when a key has already a values assigned. Youcould replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key doesn't already have a value. Or you could combine the new value with the old value.

    // Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 25);
    // if we insert a a key and a value into a hash map and then insert the same key with a diffrent value, the value associated with that key will be replaced.
    scores.insert(String::from("Blue"), 10);

    // Adding a key and a value only if a key isn't present
    // It's comon to check whether a particular key already exists in the hash map with a value and then to take the following actions: if the key does exist in the hash map, the existing value should remain the way it is; if the key doesn't exist, insert it and a value for it.
    // Hash maps have a special API -> entry, that takes the key you want to check as a a parameter. The return value of the entry method is an enum called Entry that represents a value that might or might not exist.

    // the or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // {"Yellow": 50, "Blue": 10}

    println!("{:?}", scores);

    // Updating a value based on the old value
    // Another common use case for hash maps is to look up a key's value and then update it based on the old value.
}
