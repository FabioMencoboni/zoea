use seahash::hash as shash;

/// ```
/// // EXAMPLE:
/// use zoea::hash::hash_string;
/// let string_1 = String::from("Here is a string you want to hash.");
/// let h_1 = hash_string(&string_1);
/// println!("{} and its hash is {}",string_1, h_1);
/// ```
pub fn hash_string(string: &str) -> u64 {
    let h: u64 = shash(&string.as_bytes());
    h
}

#[test]
fn str_or_string() {
    // verify hashing works with &str and &String
    let h1: u64 = hash_string(&"fish");
    let fish = String::from("fish");
    let h2: u64 = hash_string(&fish);
    assert_eq!(h1, h2);
}

#[test] // uncomment "use zoea..." and replace "demo" with "main" in README.md
//use zoea::hash::hash_string
fn demo() { 
    let mystring = String::from("Here is some string to hash");
    let myhash = hash_string(&mystring); // add hash:: in Readme.md
    println!("the hash is = {}", myhash);
}