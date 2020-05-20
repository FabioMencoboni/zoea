# zoea

## Zoea is a crate to help baby Rustaceans get up and running.

#### It contains 'easy' buttons for common things like http get requests, key-value database persistence, and Natural Language Processing.

What do you call a baby crab? That's right! you call it a **Zoea**. **Zoea** is a package for baby Rustaceans, by a baby Rustacean.

Maybe coding is only a part-time thing for you. Maybe you have learned Python and want to try Rust because you heard all the hype. But then after spending two hours trying to make a simple http request and read the response you think "!$!@$!@# it- Rust is too hard."

If you have felt this way, **zoea** is for you. 

![Zoea](https://upload.wikimedia.org/wikipedia/commons/5/51/Carcinus_maenas%2C_zoea_larva.png)



#### Think of zoea as a collection of hackish, somewhat brittle 'easy buttons'

**zoea** does many different things, favoring ergonimics and simplicity over robustness and flexability. It helps you build something, achieve *"proof of concept"*, and then mature into a more natively Rustacean approach. The only common thread between the various modules in **zoea** is they are all things the author has struggled with or tried to implement. 

### http requests

```rust
use zoea::web;
fn main() {
    let url = String::from("http://dummy.restapiexample.com/api/v1/employees"); // can be &str or &String
    let resp: String = web::get_url(&url);
    println!("response = {}", resp);
}
```


### key-value database persistence

The **Zoea** kv_database (key-value) uses a sqlite backend for simple operations.

```rust
use zoea::kv_database;
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
```


### Natural Language Processing (NLP)

```rust
use zoea::nlp;
fn main() {
    let sentence = String::from("Today I walked slowly to the garden in San Diego."); 
    let tokenized_bigrams = nlp::text_token_bigrams(&sentence);
    for bigram in tokenized_bigrams {
        println!("bigram= {}", bigram);
    }
}
```


### Hashing

```rust
use zoea::hash;
fn main() {
    let mystring = String::from("Here is some string to hash"); // can be &str or &String
    let myhash = hash::hash_string(&mystring);
    println!("the hash is = {}", myhash);
}
```



