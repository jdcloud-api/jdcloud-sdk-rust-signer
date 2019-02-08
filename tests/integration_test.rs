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
        Err(e) => return
    };
    let sk = match env::var("JDCLOUD_SK") {
        Ok(val) => val,
        Err(e) => return
    };
    let credential = Credential::new(ak, sk);
    let signer = JdcloudSigner::new(credential, "vm".to_string(), "cn-north-1".to_string());

    let mut req = Request::builder();
    let req = req.method("GET")
        .uri("http://vm.jdcloud-api.com/v1/regions/cn-north-1/instances")
        .body(Body::empty()).unwrap();
    signer.sign_request(&mut req);

    let client = Client::new();
    client.request(req);

}

