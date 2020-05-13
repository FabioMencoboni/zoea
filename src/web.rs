use curl::easy::Easy;

/// EXAMPLE
/// let url: String = String::from("http://dummy.restapiexample.com/api/v1/employees");
/// let resp_str: String = get_url(&url);
/// println!("url={}, response={}", &url, &resp_str);
pub fn get_url(url: &str) -> String {
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

#[test]
fn str_vs_string() {
    // ensure you spoort &str and &String
    let url: String = String::from("http://dummy.restapiexample.com/api/v1/employees");
    let v1 = get_url(&"http://dummy.restapiexample.com/api/v1/employees");
    let v2 = get_url(&url);
    assert_eq!(v1, v2);
    assert_eq!(v1.len() > 20, true); // ensure you actually got some text
}