use serde_derive::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, SET_COOKIE, COOKIE, HeaderValue};

#[derive(Debug,Deserialize)]
struct Resp {
    code: u32,
    message: String,
    data: Option<String>,
}
#[tokio::main]
async fn main() {
    let init_client = reqwest::Client::builder().cookie_store(true).build().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str("davdian_ops2_token=eyJhbGciOiJIUzUxMiJ9.eyJjcmVhdGVkIjoxNjAxMjkyNjkwMTM3LCJleHAiOjE2MDEzNzkwOTAsInVzZXIiOiJsaXVjaGVuIn0.CHx9O0va04f4xLMXMrBb8w8DSEshB2R5hXCRlFcSnXg-itXDje_6-useHNId5sAh7W2SFJkRhlB5qpQYPHdRYw").unwrap());
    let mut map = HashMap::new();
    // {"operation":"push","root_flag":"7","env_flag":"beta","module":"admin","branch":"master"}
    map.insert("resId", "vpc_php_pre_release");
    map.insert("reason", "1111");
    map.insert("params", "1111");
    let resp = init_client.post("http://admin.ops.vyohui.com/api/workflow/normalApplication/apply")
        .headers(headers)
        .send().await.unwrap();
    println!("{:?}", resp.text().await);
    // let client = reqwest::blocking::Client::builder().build().unwrap();
    // let mut map = HashMap::new();
    // map.insert("a", "123");
    // let resp = client.post("http://admin.ops.vyohui.com/api/workflow/normalApplication/apply")..send().unwrap();
    // let r: Resp = resp.json().unwrap();
    // println!("{:?}", resp.bytes());
    // println!("{:?}", r);
}