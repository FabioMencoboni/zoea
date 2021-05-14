# zoea

## Zoea is a crate to help baby Rustaceans get up and running.

#### It contains 'easy' buttons for common things like http get requests, key-value database persistence, and Natural Language Processing.

What do you call a baby crab? That's right! you call it a **Zoea**. **Zoea** is a package for baby Rustaceans, by a baby Rustacean.

Maybe coding is only a part-time thing for you. Maybe you have learned Python and want to try Rust because you heard all the hype. But then after spending two hours trying to make a simple http request and read the response you think "!$!@$!@# it- Rust is too hard."

If you have felt this way, **zoea** is for you. 

![Zoea](https://upload.wikimedia.org/wikipedia/commons/5/51/Carcinus_maenas%2C_zoea_larva.png)



#### Think of zoea as a collection of hackish, somewhat brittle 'easy buttons'

**zoea** does many different things, favoring ergonimics and simplicity over robustness and flexability. It helps you build something, achieve *"proof of concept"*, and then mature into a more natively Rustacean approach. The only common thread between the various modules in **zoea** is they are all things the author has struggled with or tried to implement. 



### New in v0.1.0

The get_url functionality has been changed under the hood to use ureq instead of curl. This is a more pure-rust approach vs heritage C code. The method has also had an error wrapper applied.

### easy http requests

<pre><code>use zoea::web::get_url;
fn main() {
    let url: String = String::from("http://dummy.restapiexample.com/api/v1/employees"); // can be &str or &String
    let resp_str: String = get_url(&url).unwrap();
    println!("url={}, response={}", &url, &resp_str);
}
</code></pre>


### easy matrix operations

The ::mtx module of **Zoea** adds some syntactic sugar around *nalgebra* to make it easy to create matrices of variable sizes and types. Matrix multiplication and linear algebra made easy.

<pre><code>use zoea::mtx;
// Create a 3x1000 f32 random matrix with values between -1 and 1
// Create a 1000x4 f32 random matrix with values between 5 and 25
let a = mtx::new_f32_random(3, 1000, -1f32, 1f32); //type = mtx::DMatrix \< f32>
let b = mtx::new_f32_random(1000, 4, 5f32, 25f32); // type = mtx::DMatrix \< f32>
// multiply a and b and print the result
let c = a * b;
println!("{}", c);
// take one of the values and assign it to a float
let select_element: f32 = c[(1,3)];
</code></pre>


### key-value database persistence

The **Zoea** kv_database (key-value) uses a sqlite backend for simple operations.

<pre><code>use zoea::kv_database;
fn main() {
    let db = "MyTestDatabase"; // can be &str or &String
    let key = "Key1"; // can be &str or &String
    let value = "Value you want to insert"; // can be &str or &String

    // SET the value of key in database db
    kv_database::set(&db, &key, &value);
    // GET the value of key in database db
    let same_value = kv_database::get(&db, &key);
    println!("returned value = {}", same_value);
    // LIST keys in database db
    let keys = kv_database::list_keys(&db);
    // GETting an empty key returns an empty string
    let non_key = "KeyThatDoesNotExist";
    let non_value = kv_database::get(&db, &non_key); // returns String::new();
    // DELETE a key in database db
    kv_database::delete(&db, &key);
}
</code></pre>


### Natural Language Processing (NLP)

<pre><code>use zoea::nlp;
fn main() {
    let sentence = String::from("Today I walked slowly to the garden in San Diego."); 
    let tokenized_bigrams = nlp::text_token_bigrams(&sentence);
    for bigram in tokenized_bigrams {
        println!("bigram= {}", bigram);
    }
}
</code></pre>

##### New for v0.0.8+ : porter stems 

<pre><code>use zoea::nlp;
let port_stems = nlp::porter_stems("Totally dude!");
assert_eq!(port_stems[0], "total");
</code></pre>

### Hashing

<pre><code>use zoea::hash;
fn main() {
    let mystring = String::from("Here is some string to hash"); // can be &str or &String
    let myhash = hash::hash_string(&mystring);
    println!("the hash is = {}", myhash);
}
</code></pre>


