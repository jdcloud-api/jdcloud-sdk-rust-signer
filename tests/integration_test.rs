use std::env;

use jdcloud_signer::{Credential, Signer};
use http::Request;
use reqwest::{self, Client, header::HeaderValue};
use serde_json::Value;

#[test]
fn test_vm() {
    let ak = match env::var("JDCLOUD_AK") {
        Ok(val) => val,
        Err(_e) => {
            println!("no JDCLOUD_AK env, ignore test");
            return
        }
    };
    let sk = match env::var("JDCLOUD_SK") {
        Ok(val) => val,
        Err(_e) => {
            println!("no JDCLOUD_SK env, ignore test");
            return
        }
    };
    let credential = Credential::new(ak, sk);
    let signer = Signer::new(credential, "vm".to_string(), "cn-north-1".to_string());

    let mut req = Request::builder();
    let mut req = req.method("GET")
        .uri("http://vm.jdcloud-api.com/v1/regions/cn-north-1/instances")
        .body("".to_string()).unwrap();
    assert!(signer.sign_request(&mut req).unwrap());
    println!("{:?}", req);

    let req = into_reqwest_request(req);
    let client = Client::new();
    let mut res = client.execute(req).unwrap();
    assert_eq!(res.status(), 200);
    for header in res.headers().into_iter() {
        println!("{}: {:?}", header.0, header.1);
    }
    assert_eq!(res.headers().get("content-type"),
        Some(&HeaderValue::from_str("application/json; charset=utf-8").unwrap()));
    let text = res.text().unwrap();
    let json: Value = serde_json::from_str(&text).unwrap();
    assert!(json["requestId"].is_string());
    println!("requestId: {}", json["requestId"]);
}

fn into_reqwest_request(req: Request<String>) -> reqwest::Request {
    let method = req.method().clone();
    let uri = format!("{}", req.uri());
    let mut res = reqwest::Request::new(method, url::Url::parse(&uri).unwrap());
    for header in req.headers().into_iter() {
        res.headers_mut().insert(header.0, header.1.clone());
    }
    res
}
