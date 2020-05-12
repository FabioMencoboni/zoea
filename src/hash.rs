use seahash::hash as shash;

/// EXAMPLE:
/// let string_1 = String::from("Here is a string you want to hash.");
/// let h_1 = hash_string(&string_1);
/// println!("{} and its hash is {}",string_1, h_1);
pub fn hash_string(string: &String) -> u64 {
    let h: u64 = shash(&string.as_bytes());
    h
}