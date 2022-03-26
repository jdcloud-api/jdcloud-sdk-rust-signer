use http;
use reqwest::{self, Response, Error};

#[derive(Default)]
pub struct Client {
}

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn execute(&self, request: http::Request<String>) -> Result<reqwest::blocking::Response, Error> {
        //let request = into_reqwest_request(request);
        let client = reqwest::blocking::Client::new();
        let uri = format!("{}", request.uri());
        let resp =   client.request(request.method().clone(),
                                    url::Url::parse(&uri).unwrap())
            .headers(request.headers().clone())
            .body(request.body().clone())
            .send();
        return resp
    }

    pub async fn execute_async(&self,request: http::Request<String>) -> Result<Response,Error>{
        let client = reqwest::Client::new();
        let uri = format!("{}", request.uri());
        let resp =   client.request(request.method().clone(),
                                    url::Url::parse(&uri).unwrap())
            .headers(request.headers().clone())
            .body(request.body().clone())
            .send().await?;
        Ok(resp)
    }
}
