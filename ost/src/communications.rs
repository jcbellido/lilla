#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;

#[cfg(not(target_arch = "wasm32"))]
pub fn get_string_from_network(url: &str) -> Result<String, String> {
    let mut res = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(err) => return Err(err.to_string()),
    };
    let mut body = String::new();
    match res.read_to_string(&mut body) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }
    Ok(body)
}

#[cfg(target_arch = "wasm32")]
pub fn get_string_from_network(_url: &str) -> Result<String, String> {
    todo!()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn post_string_to_network(url: &str, payload: String) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let mut res = match client.post(url).body(payload).send() {
        Ok(r) => r,
        Err(err) => return Err(err.to_string()),
    };
    let mut body = String::new();
    match res.read_to_string(&mut body) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }
    Ok(body)
}

#[cfg(target_arch = "wasm32")]
pub fn post_string_to_network(_url: &str, _payload: String) -> Result<String, String> {
    todo!()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn post_to_network(url: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let mut res = match client.post(url).send() {
        Ok(r) => r,
        Err(err) => return Err(err.to_string()),
    };
    let mut body = String::new();
    match res.read_to_string(&mut body) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }
    Ok(body)
}

#[cfg(target_arch = "wasm32")]
pub fn post_to_network(_url: &str) -> Result<String, String> {
    todo!()
}
