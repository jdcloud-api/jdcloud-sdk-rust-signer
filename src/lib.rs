extern crate http;
extern crate crypto;
extern crate chrono;
extern crate uuid;
extern crate url;

mod credential;

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crate::credential::Credential;
use http::Request;
use http::header::HeaderValue;
use chrono::prelude::*;
use uuid::Uuid;
use url::percent_encoding::{utf8_percent_encode, EncodeSet};

static EMPTY_STRING_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
static SHORT_DATE_FORMAT_STR: &str = "%Y%m%d";
static LONG_DATE_FORMAT_STR: &str = "%Y%m%dT%H%M%SZ";
static DATE_HEADER: &str = "x-jdcloud-date";
static NONCE_HEADER: &str = "x-jdcloud-nonce";
static HMAC_SHA256: &str = "JDCLOUD2-HMAC-SHA256";
static JDCLOUD_REQUEST: &str = "jdcloud2_request";
static SIGNING_KEY: &str = "JDCLOUD2";

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

    pub fn sign_request(&self, request: &mut Request<String>) -> bool {
        if !self.credential.is_valid() {
            panic!("invalid credential");
        }

        let now: DateTime<Utc> = Utc::now();
        let uuid = Uuid::new_v4().to_hyphenated().to_string();
        self.sign_request_2(request, &now, &uuid)
    }

    fn sign_request_2(&self, request: &mut Request<String>, now: &DateTime<Utc>, uuid: &str) -> bool {
        self.fill_request_with_uuid(request, now, uuid);
        let authorization = self.make_authorization(&request, now);
        request.headers_mut()
            .insert("Authorization", HeaderValue::from_str(&authorization).unwrap());
        true
    }

    fn make_authorization(&self, request: &Request<String>, now: &DateTime<Utc>) -> String {
        let (string_to_sign, signed_headers) = self.make_string_to_sign(request, &now);
        let signing_key = self.make_signing_key(&now);
        let signature = hmac_sha256(&signing_key, &string_to_sign);
        let signature = base16(&signature);
        let request_date = now.format(SHORT_DATE_FORMAT_STR).to_string();
        format!("{} Credential={}/{}, SignedHeaders={}, Signature={}",
            HMAC_SHA256,
            self.credential.ak(),
            self.make_credential_scope(&request_date),
            signed_headers,
            signature
        )
    }

    fn fill_request_with_uuid(&self, request: &mut Request<String>, now: &DateTime<Utc>, uuid: &str) {
        let request_date_time = now.format(LONG_DATE_FORMAT_STR).to_string();
        let headers = request.headers_mut();
        headers.insert(DATE_HEADER, HeaderValue::from_str(&request_date_time).unwrap());
        headers.insert(NONCE_HEADER, HeaderValue::from_str(uuid).unwrap());
    }

    fn make_signing_key(&self, now: &DateTime<Utc>) -> Vec<u8> {
        let request_date = now.format(SHORT_DATE_FORMAT_STR).to_string();
        let k_secret = self.credential.sk();
        let mac = hmac_sha256([SIGNING_KEY, k_secret].concat().as_bytes(), &request_date);
        let mac = hmac_sha256(&mac, &self.region);
        let mac = hmac_sha256(&mac, &self.service_name);
        hmac_sha256(&mac, JDCLOUD_REQUEST)
    }

    fn make_credential_scope(&self, request_date: &str) -> String {
        format!("{}/{}/{}/{}", request_date, self.region, self.service_name, JDCLOUD_REQUEST)
    }

    fn make_string_to_sign(&self, request: &Request<String>, now: &DateTime<Utc>) -> (String, String) {
        let request_date_time = now.format(LONG_DATE_FORMAT_STR).to_string();
        let request_date = now.format(SHORT_DATE_FORMAT_STR).to_string();

        let mut string_to_sign = "".to_owned();
        string_to_sign.push_str(HMAC_SHA256);
        string_to_sign.push('\n');
        string_to_sign.push_str(&request_date_time);
        string_to_sign.push('\n');
        let credential_scope = self.make_credential_scope(&request_date);
        string_to_sign.push_str(&credential_scope);
        string_to_sign.push('\n');

        let (cananical_request, signed_headers) = make_cananical_request_str(request);
        let mut hasher = Sha256::new();
        hasher.input_str(&cananical_request);
        let cananical_request = hasher.result_str();
        string_to_sign.push_str(&cananical_request);
        (string_to_sign, signed_headers)
    }
}

fn should_sign_header(header: &str) -> bool {
    return !(header.eq_ignore_ascii_case("user-agent") || header.eq_ignore_ascii_case("authorization"))
}

fn make_cananical_request_str(request: &Request<String>) -> (String, String) {
    let mut res: String = "".to_owned();
    res.push_str(request.method().as_str());
    res.push('\n');
    res.push_str(request.uri().path());
    res.push('\n');
    res.push_str(&make_cananical_query_str(request));
    res.push('\n');
    let (headers, signed_headers) = make_cananical_header_str_and_signed_headers(request);
    res.push_str(&headers);
    res.push('\n');
    res.push_str(&signed_headers);
    res.push('\n');
    res.push_str(&compute_payload_hash(request));
    (res, signed_headers)
}

fn compute_payload_hash(request: &Request<String>) -> String {
    if request.body().is_empty() {
        EMPTY_STRING_SHA256.to_string()
    } else {
        let mut hasher = Sha256::new();
        hasher.input_str(request.body());
        hasher.result_str()
    }
}


fn make_cananical_header_str_and_signed_headers(request: &Request<String>) -> (String, String) {
    let mut header_names = Vec::new();
    for header_name in request.headers().into_iter() {
        header_names.push(header_name);
    }
    header_names.sort_by(|a, b|{
        a.0.as_str().partial_cmp(b.0.as_str()).unwrap()
    });
    let mut res: String = "".to_owned();
    let mut signed_headers = "".to_owned();
    let mut first = true;
    for x in header_names {
        res.push_str(x.0.as_str());
        res.push(':');
        res.push_str(&trim_all(x.1.to_str().unwrap()));
        res.push('\n');
        if !first {
            signed_headers.push(';');
        }
        first = false;
        signed_headers.push_str(x.0.as_str());
    }
    (res, signed_headers)
}

fn trim_all(s: &str) -> String {
    let mut res: String = "".to_owned();
    let mut last_one_is_space = true;
    for c in s.trim_matches(' ').chars() {
        if c == ' ' {
            if !last_one_is_space {
                res.push(c);
                last_one_is_space = true;
            }
        } else {
            res.push(c);
            last_one_is_space = false;
        }
    }
    res
}

fn make_cananical_query_str(request: &Request<String>) -> String {
    let query = request.uri().query();
    let query = match query {
        None => "",
        Some(q) => q
    };
    let query = url::form_urlencoded::parse(query.as_bytes());
    let mut vec = Vec::new();
    for q in query {
        vec.push((q.0.to_string(), q.1.to_string()));
    }
    vec.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.partial_cmp(&b.1).unwrap()
        } else {
            a.0.partial_cmp(&b.0).unwrap()
        }
    });
    let mut res: String = "".to_owned();
    let mut first = true;
    for x in vec {
        if !first {
            res.push('&');
        }
        first = false;
        res.push_str(&utf8_percent_encode(&x.0, Aws4QueryItemEncodeSet).to_string());
        res.push('=');
        res.push_str(&utf8_percent_encode(&x.1, Aws4QueryItemEncodeSet).to_string());
    }
    res
}

fn hmac_sha256(key: &[u8], data: &str) -> Vec<u8> {
    let mut hmac = Hmac::new(Sha256::new(), key);
    hmac.input(data.as_bytes());
    let result = hmac.result();
    let code = result.code();
    code.to_vec()
}

#[derive(Copy, Clone, Debug)]
struct Aws4QueryItemEncodeSet;

impl EncodeSet for Aws4QueryItemEncodeSet {
    #[inline]
    fn contains(&self, c: u8) -> bool {
        !(('A' as u8 <= c && c <= 'Z' as u8)
            || ('a' as u8 <= c && c <= 'z' as u8)
            || ('0' as u8 <= c && c <= '9' as u8)
            || c == '-' as u8
            || c == '_' as u8
            || c == '.' as u8
            || c == '~' as u8)
    }
}

fn base16(data: &[u8]) -> String{
    let mut res = "".to_owned();
    let a = "0123456789abcdef".as_bytes();
    for c in data {
        let b1 = c/16;
        let b2 = c%16;
        res.push(a[b1 as usize] as char);
        res.push(a[b2 as usize] as char);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::header::{CONTENT_TYPE, USER_AGENT};

    #[test]
    fn test_sign_request() {
        let c = Credential::new("ak".to_string(), "sk".to_string());
        let s = JdcloudSigner::new(c, "service_name".to_string(), "cn-north-1".to_string());
        let mut req = make_test_request();
        let res = s.sign_request(&mut req);
        assert!(res);
        assert_eq!(get_headers_from_request(&req),
            ["authorization", "content-type", "user-agent", "x-jdcloud-date", "x-jdcloud-nonce"]);
    }

    #[test]
    fn test_sign_request_2() {
        let c = Credential::new("ak".to_string(), "sk".to_string());
        let s = JdcloudSigner::new(c, "service_name".to_string(), "cn-north-1".to_string());
        let mut req = make_test_request();
        let now = chrono::Utc.ymd(2018, 4, 5).and_hms(01, 02, 03);
        let uuid = "55f3919e-3a7d-4174-b117-f150ff25e274";
        let res = s.sign_request_2(&mut req, &now, &uuid);
        assert!(res);
        assert_eq!(get_headers_from_request(&req),
            ["authorization", "content-type", "user-agent", "x-jdcloud-date", "x-jdcloud-nonce"]);
        assert_eq!(req.headers().get("authorization").unwrap(),
            "JDCLOUD2-HMAC-SHA256 Credential=ak/20180405/cn-north-1/service_name/jdcloud2_request, SignedHeaders=content-type;user-agent;x-jdcloud-date;x-jdcloud-nonce, Signature=b814d29cc86f397d5772e104e67ce125ea621a96d2e55f55171fa4719937a15f");
    }

    #[test]
    fn test_make_signing_key() {
        let c = Credential::new("ak".to_string(), "sk".to_string());
        let s = JdcloudSigner::new(c, "service_name".to_string(), "cn-north-1".to_string());
        let now = chrono::Utc.ymd(2018, 4, 5).and_hms(01, 02, 03);
        assert_eq!(base16(&s.make_signing_key(&now)), "b302aa05734bcaf60be65a4be7c971669ac55444769681c19113d80460e31a33");
    }


    #[test]
    fn test_hmac_sha1() {
        let a = hmac_sha256("AWS4wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY".as_bytes(), "20150830");
        let a = hmac_sha256(&a, "us-east-1");
        let a = hmac_sha256(&a, "iam");
        let a = hmac_sha256(&a, "aws4_request");
        assert_eq!(base16(&a), "c4afb1cc5771d871763a393e44b703571b55cc28424d1a5e86da6ed3c154a4b9");
    }

    fn make_test_request() -> Request<String> {
        let mut req = Request::builder();
        req.uri("http://www.jdcloud-api.com/v1/regions/cn-north-1/instances?pageNumber=2&pageSize=10")
            .method("GET")
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, "JdcloudSdkRust/0.0.1 vm/0.7.4")
            .body("".to_string())
            .unwrap()
    }

    #[test]
    fn test_make_credential_scope() {
        let c = Credential::new("ak".to_string(), "sk".to_string());
        let s = JdcloudSigner::new(c, "service_name".to_string(), "cn-north-1".to_string());
        assert_eq!(s.make_credential_scope("20180101"), "20180101/cn-north-1/service_name/jdcloud2_request");
    }

    #[test]
    fn test_make_string_to_sign() {
        let c = Credential::new("ak".to_string(), "sk".to_string());
        let s = JdcloudSigner::new(c, "service_name".to_string(), "cn-north-1".to_string());
        let req = make_test_request();
        let now = chrono::Utc.ymd(2018, 4, 5).and_hms(01, 02, 03);
        assert_eq!(s.make_string_to_sign(&req, &now).0,
            "JDCLOUD2-HMAC-SHA256\n20180405T010203Z\n20180405/cn-north-1/service_name/jdcloud2_request\ne6e831a1bb6514d638df6d183d74e830f048843396ec512150f862654e6ffc33");
    }

    fn get_headers_from_request(req: &Request<String>) -> Vec<String> {
        let mut res = Vec::new();
        for header_name in req.headers().into_iter() {
            res.push(header_name.0.to_string());
        }
        res.sort();
        res
    }


    #[test]
    fn test_should_sign_header() {
        let should_not_sign_headers = ["user-agent", "User-Agent", "Authorization", "authorization"];
        let should_sign_headers = ["X-hello", "Content-Length", "User"];
        for header in should_not_sign_headers.iter() {
            assert!(!should_sign_header(header))
        }
        for header in should_sign_headers.iter() {
            assert!(should_sign_header(header))
        }
    }

    #[test]
    fn test_make_cananical_request_str() {
        let req = Request::builder().method("GET").body("".to_string()).unwrap();
        assert_eq!(make_cananical_request_str(&req).0, ["GET\n/\n\n\n\n",EMPTY_STRING_SHA256].concat());
        let req = Request::builder().method("POST").body("".to_string()).unwrap();
        assert_eq!(make_cananical_request_str(&req).0, ["POST\n/\n\n\n\n",EMPTY_STRING_SHA256].concat());
        let req = Request::builder().method("GET").uri("/helloworld").body("".to_string()).unwrap();
        assert_eq!(make_cananical_request_str(&req).0, ["GET\n/helloworld\n\n\n\n",EMPTY_STRING_SHA256].concat());
        let req = Request::builder().method("GET").uri("/hello%20world").body("".to_string()).unwrap();
        assert_eq!(make_cananical_request_str(&req).0, ["GET\n/hello%20world\n\n\n\n",EMPTY_STRING_SHA256].concat());
        let req = Request::builder().method("GET").uri("/Hello%20world").body("".to_string()).unwrap();
        assert_eq!(make_cananical_request_str(&req).0, ["GET\n/Hello%20world\n\n\n\n",EMPTY_STRING_SHA256].concat());
        let req = Request::builder().method("GET").uri("/Hello%20world?").body("".to_string()).unwrap();
        assert_eq!(make_cananical_request_str(&req).0, ["GET\n/Hello%20world\n\n\n\n",EMPTY_STRING_SHA256].concat());
        let req = Request::builder().method("GET").uri("/Hello%20world?a=1").body("".to_string()).unwrap();
        assert_eq!(make_cananical_request_str(&req).0, ["GET\n/Hello%20world\na=1\n\n\n",EMPTY_STRING_SHA256].concat());
        let req = Request::builder().method("GET").uri("/Hello%20world?a=1").header("A", "B").body("".to_string()).unwrap();
        assert_eq!(make_cananical_request_str(&req).0, ["GET\n/Hello%20world\na=1\na:B\n\na\n",EMPTY_STRING_SHA256].concat());
        let req = Request::builder().method("GET").uri("/Hello%20world?a=1").header("A", "B").body("a".to_string()).unwrap();
        assert_eq!(make_cananical_request_str(&req).0,
            ["GET\n/Hello%20world\na=1\na:B\n\na\n","ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb"].concat());
    }

    fn make_cananical_header_str(request: &Request<String>) -> String {
       make_cananical_header_str_and_signed_headers(&request).0
    }

    #[test]
    fn test_make_cananical_header_str() {
        let req = Request::builder().method("GET").body("".to_string()).unwrap();
        assert_eq!(make_cananical_header_str(&req), "");

        let single_header_tcs = vec![
            ("Hello", "World", "hello:World\n"),
            ("Hello", "Wor ld", "hello:Wor ld\n"),
            ("Hello", "Wor  ld", "hello:Wor ld\n"),
            ("Hello", "\"World\"", "hello:\"World\"\n"),
            ("Hello", " World ", "hello:World\n"),
            ("Hello", "  World  ", "hello:World\n"),
            ("Hello", "  World", "hello:World\n"),
            ("Hello", "World  ", "hello:World\n"),
            ("Hello", "", "hello:\n"),
            ("Hello", "  ", "hello:\n"),
            ("Hello", "  \t", "hello:\t\n"),
        ];

        for tc in single_header_tcs {
            let req = Request::builder().method("GET")
                .header(tc.0, tc.1)
                .body("".to_string()).unwrap();
            assert_eq!(make_cananical_header_str(&req), tc.2);
        }

        let multi_header_cases = vec![
            (vec![("Hello", "World")], "hello:World\n"),
            (vec![("Hello", "World"), ("A", "B")], "a:B\nhello:World\n"),
            (vec![("A", "A"), ("B", "B")], "a:A\nb:B\n"),
            (vec![("B", "B"), ("A", "A")], "a:A\nb:B\n"),
        ];
        for tc in multi_header_cases {
            let mut req = Request::builder();
            req.method("GET");
            for x in tc.0 {
                req.header(x.0, x.1);
            }
            let req = req.body("".to_string()).unwrap();
            assert_eq!(make_cananical_header_str(&req), tc.1);
        }
    }

    fn make_cananical_signed_headers(request: &Request<String>) -> String {
       make_cananical_header_str_and_signed_headers(&request).1
    }

    #[test]
    fn test_make_cananical_signed_headers() {
        let req = Request::builder().method("GET").body("".to_string()).unwrap();
        assert_eq!(make_cananical_signed_headers(&req), "");

        let testcases = vec![
            (vec!["a"], "a"),
            (vec![], ""),
            (vec!["a", "b"], "a;b"),
            (vec!["b", "a"], "a;b"),
            (vec!["A", "a"], "a;a"),
        ];
        for tc in testcases {
            let mut req = Request::builder();
            req.method("GET");
            for x in tc.0 {
                req.header(x, "a");
            }
            let req = req.body("".to_string()).unwrap();
            assert_eq!(make_cananical_signed_headers(&req), tc.1);
        }
    }

    #[test]
    fn test_make_cananical_query_str() {
        let req = Request::builder().method("GET").body("".to_string()).unwrap();
        assert_eq!(make_cananical_query_str(&req), "");
        let testcases = vec![
            ("/", ""),
            ("/?", ""),
            ("/?a=1", "a=1"),
            ("/?a=1#bcd", "a=1"),
            ("/?a=1&b=1", "a=1&b=1"),
            ("/?a&b", "a=&b="),
            ("/?a=&b", "a=&b="),
            ("/?a&b=", "a=&b="),
            ("/?a=&b=", "a=&b="),
            ("/?b&a", "a=&b="),
            ("/?b&a&B&A", "A=&B=&a=&b="),
            ("/?a=-_.~", "a=-_.~"),
            ("/?a=/", "a=%2F"),
            ("/?a=%", "a=%25"),
            ("/?a=%2", "a=%252"),
            ("/?a=%00", "a=%00"),
            ("/?a=%ff", "a=%EF%BF%BD"),
            ("/?a=%0g", "a=%250g"),
            ("/?a=%fg", "a=%25fg"),
            ("/?b&a=%e4%b8%ad", "a=%E4%B8%AD&b="),
            ("/?b&a=%e4%b8", "a=%EF%BF%BD&b="),
            ("/?b&a=%2f%25%20", "a=%2F%25%20&b="),
            ("/?b&a=%2f%25%20", "a=%2F%25%20&b="),
            ("/?b&a=+++", "a=%20%20%20&b="),
            ("/?a=2&a=1", "a=1&a=2"),
            ("/?a=1&a=1", "a=1&a=1"),
        ];
        for tc in testcases {
            let req = Request::builder().method("GET").uri(tc.0).body("".to_string()).unwrap();
            assert_eq!(make_cananical_query_str(&req), tc.1);
        }
    }
}
