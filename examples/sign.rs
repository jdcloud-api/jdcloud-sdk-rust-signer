use jdcloud_signer::{Credential, Signer};
use http::Request;
use std::env;

fn main() {
    let ak = env::var("JDCLOUD_AK").unwrap();
    let sk = env::var("JDCLOUD_SK").unwrap();
    let credential = Credential::new(ak, sk);
    let signer = Signer::new(credential, "vm".to_string(), "cn-north-1".to_string());

    let mut req = Request::builder();
    let mut req = req.method("GET")
        .uri("https://vm.jdcloud-api.com/v1/regions/cn-north-1/instances")
        .body("".to_string()).unwrap();
    signer.sign_request(&mut req).unwrap();
    println!("{:?}", req);
}
