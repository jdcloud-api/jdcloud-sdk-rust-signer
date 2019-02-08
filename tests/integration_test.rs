extern crate hyper;

use hyper::{Client, Uri, client::HttpConnector};
use hyper::rt::{self, Future, Stream};
use hyper::Body;
use std::env;

use jdcloud_sdk_rust_signer::{Credential, JdcloudSigner};
use http::Request;


#[test]
fn test_vm() {
    let ak = match env::var("JDCLOUD_AK") {
        Ok(val) => val,
        Err(_e) => return
    };
    let sk = match env::var("JDCLOUD_SK") {
        Ok(val) => val,
        Err(_e) => return
    };
    let credential = Credential::new(ak, sk);
    let signer = JdcloudSigner::new(credential, "vm".to_string(), "cn-north-1".to_string());

    let mut req = Request::builder();
    let mut req = req.method("GET")
        .uri("http://vm.jdcloud-api.com/v1/regions/cn-north-1/instances")
        .body("".to_string()).unwrap();
    signer.sign_request(&mut req);
    println!("{:?}", req);
    rt::run(fetch_req(&req));
}

fn fetch_req(req: &http::Request<String>) -> impl Future<Item=(), Error=()> {
    let client = Client::new();
    let uri = req.uri().to_string().parse().unwrap();
    client
        .get(uri)
        .map(|res| {
          println!("Response: {}", res.status());
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
}

