extern crate http;
extern crate crypto;

mod credential;

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use crate::credential::Credential;
use http::Request;

static EMPTY_STRING_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";


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

    pub fn sign_request(&self, request: Request<String>) -> Result<Request<String>, &str> {
        if !self.credential.is_valid() {
            panic!("invalid credential")
        }
        Ok(Request::builder().body("".to_string()).unwrap())
    }

    fn compute_payload_hash(&self, request: Request<String>) -> String {
        if request.body().is_empty() {
            EMPTY_STRING_SHA256.to_string()
        } else {
            let mut hasher = Sha256::new();
            hasher.input_str(request.body());
            hasher.result_str()
        }
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
            .body("".to_string())
            .unwrap();
        let req = s.sign_request(req);  
    }
}
