//use curl::easy::Easy;
use std::fmt;
use ureq; // minimal library for simple, blocking http GET/POST requests

/// ### get_url
/// This method is intended to be about as simple as a http request can get in Rust
/// Feed it a URL and it returns the page response as a string
/// #### EXAMPLE
/// ```
/// use zoea::web::get_url;
/// let url: String = String::from("http://dummy.restapiexample.com/api/v1/employees");
/// let resp_str: String = get_url(&url).unwrap();
/// println!("url={}, response={}", &url, &resp_str);
/// ```

#[derive(Debug, Clone)]
pub struct ErrorHTTP {
    msg: String,
}

impl fmt::Display for ErrorHTTP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}



pub fn get_url(url: &str) -> Result<String, ErrorHTTP> {
    let req = ureq::get(url);//::get("https://jsonplaceholder.typicode.com/posts")
    match req.call() { // can ureq understand the http request?
        Ok(resp) => { 
            match resp.into_string() { // can the response be converted to a string?
                Ok(string) => return Ok(string),
                Err(_) => return Err(ErrorHTTP{msg: String::from("could not parse response into string")}),
            }
        },
        Err(_) => return Err(ErrorHTTP{msg: String::from("unable to make this http request")}),
    };
}

#[test]
fn str_vs_string() {
    // ensure you spoort &str and &String
    let url: String = String::from("https://jsonplaceholder.typicode.com/posts/1");
    let v1 = get_url(&"https://jsonplaceholder.typicode.com/posts/1").unwrap();
    let v2 = get_url(&url).unwrap();
    assert_eq!(v1, v2);
    assert_eq!(v1.len() > 20, true); // ensure you actually got some text
}