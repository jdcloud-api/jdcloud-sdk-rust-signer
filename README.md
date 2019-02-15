[![Build Status](https://travis-ci.org/jdcloud-api/jdcloud-sdk-rust-signer.svg?branch=master)](https://travis-ci.org/jdcloud-api/jdcloud-sdk-rust-signer)

# Usage: 普通方式

## Cargo.toml

添加如下一段到你的 Cargo.toml

```
[dependencies]
jdcloud_signer = "0.1"
```

## 使用范例

```rust
use jdcloud_signer::{Credential, Signer, Client};
use http::Request;
use serde_json::Value;

fn main() {
    let ak = "...";
    let sk = "...";
    let credential = Credential::new(ak, sk);
    let signer = Signer::new(credential, "vm".to_string(), "cn-north-1".to_string());

    let mut req = Request::builder();
    let mut req = req.method("GET")
        .uri("http://vm.jdcloud-api.com/v1/regions/cn-north-1/instances")
        .body("".to_string()).unwrap();
    signer.sign_request(&mut req).unwrap();

    let client = Client::new();
    let mut res = client.execute(req).unwrap();

    println!("status: {}", res.status());
    for header in res.headers().into_iter() {
        println!("{}: {:?}", header.0, header.1);
    }
    let text = res.text().unwrap();
    let json: Value = serde_json::from_str(&text).unwrap();
    println!("requestId: {}", json["requestId"]);
}
```

# Usage: 只签名方式

如果你不喜欢 `reqwest`, 准备使用自己的http库，那么可以选择只做签名。


## Cargo.toml

添加如下一段到你的 Cargo.toml

```
[dependencies]
jdcloud_signer = { version = "0.1", default-features = false }
```

## 使用范例

```rust
use jdcloud_signer::{Credential, Signer};
use http::Request;

fn main() {
    let ak = "...";
    let sk = "...";
    let credential = Credential::new(ak, sk);
    let signer = Signer::new(credential, "vm".to_string(), "cn-north-1".to_string());

    let mut req = Request::builder();
    let mut req = req.method("GET")
        .uri("http://vm.jdcloud-api.com/v1/regions/cn-north-1/instances")
        .body("".to_string()).unwrap();
    signer.sign_request(&mut req).unwrap();
    println!("{}", req);
}
```

