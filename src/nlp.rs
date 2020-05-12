
use rust_stemmers::{Algorithm, Stemmer}; // for stemming single words


/// EXAMPLE
/// let string_2 = String::from("I walked to San Diego slowly today!");
/// let tokens = text_tokens(&string_2);
/// println!("Sentence = {}", string_2);
/// for token in tokens {
///     println!("bigram= {}", token);
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


/// EXAMPLE
/// let string_2 = String::from("I walked to San Diego slowly today!");
/// let bigrams_2 = text_token_bigrams(&string_2);
/// println!("Sentence = {}", string_2);
/// for gram in bigrams_2 {
///    println!("bigram= {}", gram);
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

