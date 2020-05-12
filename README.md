# zoea


![Zoea](https://upload.wikimedia.org/wikipedia/commons/5/51/Carcinus_maenas%2C_zoea_larva.png)

### The worst crates.io package you will ever outgrow.

What do you call a baby crab? That's right! you call it a **Zoea**. **Zoea** is a package for baby Rustaceans. 

Maybe coding is only a part-time thing for you. Maybe you have learned Python and want to try Rust because you heard all the hype. But then after spending two hours trying to make a simple http request and read the response you think "!$!@$!@# it- Rust is too hard."

If you have felt this way, **zoea** is for you. 

### Think of zoea as a brittle, hackish "easy" button

**zoea** does many different things, favoring ergonimics and simplicity over robustness and flexability. That is why this is *the worst crates.io package you will ever outgrow* - It helps you get something working  more complex than *Hello World* so you can see a working example, inspect the code, and then mature into a more natively Rustacean approach.


The only common thread between the various modules in **zoea** is they are all things the author has struggled with or tried to implement. Here are some highlights:

### Dead-simple http requests

<pre><code>
use zoea::web;
fn main() {
    let url = String::from("http://dummy.restapiexample.com/api/v1/employees");
    let resp: String = web::get_url(&url);
    println!("response = {}", resp);
}
</code></pre>

### Hashing

<pre><code>
use zoea::hash;
fn main() {
    let mystring = String::from("Here is some string to hash");
    let myhash = hash::hash_string(&mystring);
    println!("the hash is = {}", myhash);
}
</code></pre>

### Natural Language Processing (NLP)

<pre><code>
use zoea::nlp;
fn main() {
    let sentence = String::from("Today I walked slowly to the garden in San Diego.");
    let tokenized_bigrams = nlp::text_token_bigrams(&sentence);
    for bigram in tokenized_bigrams {
        println!("bigram= {}", bigram);
    }
}
</code></pre>

