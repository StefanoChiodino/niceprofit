extern crate reqwest;

pub fn get_nicehash_response() -> Result<String, self::reqwest::Error> {
    use std::io::Read;
    const API_URL: &str = "https://api.nicehash.com/api?method=simplemultialgo.info";

    let mut resp = reqwest::get(API_URL)?;
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);

    Ok(content)
}

#[test]
fn can_get_nicehash_response() {
    let response = get_nicehash_response();

    assert!(response.is_ok());
}
