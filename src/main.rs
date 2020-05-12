

fn main() {
    println!("Hello world! I'm a new baby Zoea");

    let url: String = String::from("https://qlsv4412xi.execute-api.us-east-1.amazonaws.com/default/ClientIP");
    let resp_str: String = get_url(&url);
    println!("url={}, response={}", &url, &resp_str);

    let string_1 = String::from("Here is a string you want to hash.");
    let h_1 = hash_string(&string_1);
    println!("{} and its hash is {}",string_1, h_1);




    let string_2 = String::from("I walked to San Diego slowly today!");
    let tokens = text_tokens(&string_2);
    println!("Sentence = {}", string_2);
    for token in tokens {
        println!("bigram= {}", token);
    };

    let string_2 = String::from("I walked to San Diego slowly today!");
    let bigrams_2 = text_token_bigrams(&string_2);
    println!("Sentence = {}", string_2);
    for gram in bigrams_2 {
        println!("bigram= {}", gram);
    };

}

/// EXAMPLE
/// let url: String = String::from("https://qlsv4412xi.execute-api.us-east-1.amazonaws.com/default/ClientIP");
/// let resp_str: String = get_url(&url);
/// println!("url={}, response={}", &url, &resp_str);
use curl::easy::Easy;
pub fn get_url(url: &String) -> String {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    {
    easy.url(&url).unwrap();
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        dst.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    }
    let resp_str = String::from_utf8(dst).unwrap();
    // return the response
    resp_str
}

/// EXAMPLE:
/// let string_1 = String::from("Here is a string you want to hash.");
/// let h_1 = hash_string(&string_1);
/// println!("{} and its hash is {}",string_1, h_1);
use seahash::hash as shash;
pub fn hash_string(string: &String) -> u64 {
    let h: u64 = shash(&string.as_bytes());
    h
}




use rust_stemmers::{Algorithm, Stemmer}; // for stemming single words
pub fn text_tokens(text: &String) -> Vec<String> {
    // string goes in, list of tokens comes out

    // declare some variables and bring them into context
    let en_stemmer = Stemmer::create(Algorithm::English); // english langage stemmer- one word at a time please!
    let mut stem: std::borrow::Cow<str>; // copy-on-write pointer to a word stem
    let mut token: String;
    let mut tokens =  Vec::new();

    // convert text to lower case and iterate over words
    let text_clean = text.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'','!','?'][..], "");
    let text_lower = text_clean.to_lowercase();
    let words = text_lower.split_whitespace();
    for gram in words  {

        // find the word stem
        stem = en_stemmer.stem(&gram);
        token = stem.to_string(); 
        
        // append this token to tne reesults
        tokens.push(token);     
    }
    // return the vector of bigrams
    tokens
}

//use rust_stemmers::{Algorithm, Stemmer}; // for stemming single words
pub fn text_token_bigrams(text: &String) -> Vec<String> {
    // string goes in, tokenized bigrams come out

    // declare some variables and bring them into context
    let en_stemmer = Stemmer::create(Algorithm::English); // english langage stemmer- one word at a time please!
    let mut stem: std::borrow::Cow<str>; // copy-on-write pointer to a word stem
    let mut bigram = String::from("!NewDoc"); // bigram of last two grams
    let mut bigrams =  Vec::new();

    // convert text to lower case and iterate over words
    let text_clean = text.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'','!','?'][..], "");
    let text_lower = text_clean.to_lowercase();
    let words = text_lower.split_whitespace();
    for gram in words  {

        // find the word stem and bigram with the previous stem
        stem = en_stemmer.stem(&gram);
        bigram.push_str(" ");
        bigram.push_str(&stem);
        
        // append this bigram to tne reesults
        bigrams.push(bigram);
        // replace the "previous gram" so bigram is ready for the next loop
        bigram = stem.to_string();        
    }
    // return the vector of bigrams
    bigrams
}

use std::collections::HashMap; // for dictionaries
pub fn sparsevec_cosine_similarity(u: HashMap<u32, f32>, v: HashMap<u32, f32>) -> f32 {
    // return the similarity of two sparse vectors as defined by (u*v)/(||u||*||v||)

    let mut dot_prod: f32 = 0f32;      // dot product
    let mut u_norm: f32 = 0f32;    // norm of vector u
    let mut v_norm: f32 = 0f32;     // norm of vector v

    for (key, u_element) in &u {
        let v_element = match &v.get(&key){
            Some(element) => element,
            None => &0f32,
        };
        dot_prod = dot_prod + (u_element * v_element);
        u_norm = u_norm + u_element;
        println!("{}.{}.{}", key, u_element, v_element);
    }
    for (_, v_element) in &v{
        v_norm = v_norm + v_element;
    }

    // calculate and return the similarity
    let similarity:f32 = 100.0f32*dot_prod/(u_norm * v_norm); // as percentage
    //println!("u_norm={}, v_norm={}, dot_prod={}", u_norm, v_norm, dot_prod);
    similarity

}
