extern crate hyper;

// #[macro_use]
// extern crate futures;

use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper::Body;
use std::env;

use jdcloud_signer::{Credential, Signer};
use http::Request;


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
    let chunks = res.and_then(|res2| {
        assert_eq!(res2.status(), 200);
        res2.into_body().concat2()
    });
    chunks.map(|chunks| {
        println!("Body: \n{:?}", chunks);
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
