use http;
use reqwest::{self, Response, Error};

pub struct Client {
}

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn execute(&self, request: http::Request<String>) -> Result<Response, Error> {
        //let request = into_reqwest_request(request);
        let client = reqwest::Client::new();
        let uri = format!("{}", request.uri());
        client.request(request.method().clone(), url::Url::parse(&uri).unwrap())
            .headers(request.headers().clone())
            .body(request.body().clone())
            .send()
    }
}
