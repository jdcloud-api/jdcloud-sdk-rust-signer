use http;
use reqwest::{self, Response, Error};

pub struct Client {
}

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn execute(&self, request: http::Request<String>) -> Result<Response, Error> {
        let request = into_reqwest_request(request);
        let client = reqwest::Client::new();
        client.execute(request)
    }
}

fn into_reqwest_request(req: http::Request<String>) -> reqwest::Request {
    let method = req.method().clone();
    let uri = format!("{}", req.uri());
    let mut res = reqwest::Request::new(method, url::Url::parse(&uri).unwrap());
    for header in req.headers().into_iter() {
        res.headers_mut().insert(header.0, header.1.clone());
    }
    res
}
