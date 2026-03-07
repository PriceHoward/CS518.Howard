use std::collections::HashSet;
use std::hash::Hash;

/*
    The complexity of the hashing version is O(n) for the same reasons as C++ and as Clojure. You only need to loop and compare values once. 
    This is because we make a new vector to store the unique values in (the first occurance).

    So like in C++ we make a second version of the stable_dedup for values such as f32 and f64. This second version is O(n^2) just like in C++ due to the fact
    we have to double the loop (Using a function to so the second looping). The function that causes it to make the second pass through the list is
    .contains(). This has to go through the list linerally. The reason this function has to be a different name is in Rust you cannot have two functions 
    of the same name with different parameters or types. (As a C++ developer by trade. I am not a fan of this design personally.)

    The reason to choose the hashing is for the ability to have the O(n) speed. Rust makes it optional to use the hashing except for f32 and f64.
    They are not included in the documentation for hash. https://doc.rust-lang.org/std/hash/trait.Hash.html#tymethod.hash. 
    After doing some digging on this, I found https://practice.course.rs/collections/hashmap.html which states: 
    "Note that f32 and f64 do not implement Hash, likely because floating-point precision errors would 
    make using them as hashmap keys horribly error-prone."
*/

pub fn stable_dedup<'a, T: Eq + Hash>(input: &'a [T]) -> Vec<&'a T> {
    let mut seen_value: HashSet<&T> = HashSet::new();
    let mut unique_values: Vec<&T> = Vec::new();
    for current_value in input {
        if seen_value.insert(current_value) {
            unique_values.push(current_value);
        }
    }
    unique_values
}

pub fn stable_dedup_eq<'a, T: PartialEq>(input: &'a [T]) -> Vec<&'a T> {
    let mut unique_values: Vec<&T> = Vec::new();
    for current_value in input {
        if !unique_values.contains(&current_value) {
            unique_values.push(current_value);
        }
    }
    unique_values
}

fn main() {
    println!("-----------Integer Test-----------");
    println!("{:?}", stable_dedup(&[5, 4, 2, 1, 6, 4, 3, 2, 1, 7, 6, 5, 4, 3, 2, 1]));

    println!("-----------String Test-----------");
    println!("{:?}", stable_dedup(&["Hello", "Bonjour", "Hello", "Adios", "Wazup", "Adios"]));

    println!("-----------f64 Test-----------");
    let floats: Vec<f64> = vec![1.1, 2.2, 3.3, 1.1, 4.4, 2.2, 5.5];
    println!("{:?}", stable_dedup_eq(&floats));
}
