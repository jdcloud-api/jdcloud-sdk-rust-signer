extern crate http;

mod credential;

use crate::credential::Credential;
use http::{Request, Error};

pub struct JdcloudSigner {
    credential: Credential,
    service_name: String,
    region: String,
}

impl JdcloudSigner {
    pub fn new(credential: Credential, service_name: String, region: String) -> JdcloudSigner {
        JdcloudSigner {
            credential,
            service_name,
            region
        }
    }

    pub fn sign_request(&self, request: Request<()>) -> Result<Request<()>, Error> {
        Ok(Request::builder().body(()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::header::{CONTENT_TYPE, USER_AGENT};

    #[test]
    fn test_new() {
        let c = Credential::new("ak".to_string(), "sk".to_string());
        let s = JdcloudSigner::new(c, "service_name".to_string(), "cn-north-1".to_string());
        let mut req = Request::builder();
        let req = req.uri("http://www.jdcloud-api.com/v1/regions/cn-north-1/instances?pageNumber=2&pageSize=10")
            .method("GET")
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, "JdcloudSdkRust/0.0.1 vm/0.7.4")
            .body(())
            .unwrap();
        let req = s.sign_request(req);  
    }
}
