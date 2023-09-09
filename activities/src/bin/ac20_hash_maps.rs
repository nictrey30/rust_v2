use std::collections::HashMap;
// just like vectors, hash maps storetheir data on the heap.
fn main() {
    // Storing keys with associated values in hash maps
    // The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function.
    // Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.

    // Creating a new hash map
    // One way to create an empty hash map is to use new and to add elements with insert.
    let mut scores = HashMap::new();
    // Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // We can get the value out of the hash map by providing its key to the get method
    let team_name = String::from("Blue");
    // the get method returns an Option<&V>; it there is no value for that key in the hash map, get will return None.
    // This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unrawp_or to set score to zero if score doesn't have an entry for the key.
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score of the blue team: {score}");

    // we can iterate over each key-value pair in a hash map in as we do with vectors. The code will print each pair in arbitrary order
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash maps and ownership
    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For ones like String, the values will be moved and the hash map will be the owner of those values.
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map: {:?}", map); //map: {"Favorite Color": "Blue"}
                                // field_name and field_value are invalid at this point
                                // println!("field_name: {field_name}"); // doesn't work, value borrowed here after move
}
