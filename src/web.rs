//use curl::easy::Easy;
use ureq; // minimal library for simple, blocking http GET/POST requests

/// ### get_url
/// This method is intended to be about as simple as a http request can get in Rust
/// Feed it a URL and it returns the page response as a string
/// #### EXAMPLE
/// ```
/// use zoea::web::get_url;
/// let url: String = String::from("http://dummy.restapiexample.com/api/v1/employees");
/// let resp_str: String = get_url(&url);
/// println!("url={}, response={}", &url, &resp_str);
/// ```
/*pub fn get_url(url: &str) -> String {
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
}*/

pub fn get_url(url: &str) -> String {
    let req = ureq::get(url);//::get("https://jsonplaceholder.typicode.com/posts")
    let s: String = req.call().unwrap().into_string().unwrap();//.into_string().unwrap();//resp.unwrap().into_string().unwrap();
    println!("{}", s);
    s
}

#[test]
fn str_vs_string() {
    // ensure you spoort &str and &String
    let url: String = String::from("https://jsonplaceholder.typicode.com/posts/1");
    let v1 = get_url(&"https://jsonplaceholder.typicode.com/posts/1");
    let v2 = get_url(&url);
    assert_eq!(v1, v2);
    assert_eq!(v1.len() > 20, true); // ensure you actually got some text
}