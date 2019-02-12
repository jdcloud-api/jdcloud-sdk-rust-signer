extern crate hyper;

// #[macro_use]
// extern crate futures;

use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper::Body;
use std::env;

use jdcloud_sdk_rust_signer::{Credential, Signer};
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
    let signer = Signer::new(credential, "vm".to_string(), "cn-north-1".to_string());

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
    let mut req2 = Request::new(Body::empty());
    *req2.uri_mut() = req.uri().clone();
    for header in req.headers().into_iter() {
        req2.headers_mut().insert(
            header.0,
            header.1.clone()
        );
    }
    let res = client
        .request(req2)
        .map_err(|_| {
            panic!("should not error");
        });
    let body = res.and_then(|res2: hyper::Response<hyper::Body>| {
        assert_eq!(res2.status(), 200);
        let body = res2.into_body();
        body.into_future()
    });
    body.map(|res| {
        println!("Body: \n{:?}", res.0.unwrap());
    }).map_err(|_|{
        panic!("should not error");
    })
}

// type Error = Box<dyn std::error::Error>;

// fn example(body: &hyper::Body) -> impl Future<Item = String, Error = Error> {
//     body.map_err(Error::from)
//         .concat2()
//         .and_then(|c| {
//             str::from_utf8(&c).map(str::to_owned).map_err(Error::from)
//         })
// }
