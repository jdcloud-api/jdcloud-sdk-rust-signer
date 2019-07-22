# jdcloud_signer: jdcloud.com API signer

[![Build Status](https://travis-ci.org/jdcloud-api/jdcloud-sdk-rust-signer.svg?branch=master)](https://travis-ci.org/jdcloud-api/jdcloud-sdk-rust-signer)
[![MIT licensed](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0.html)
[![crates.io](https://meritbadge.herokuapp.com/jdcloud_signer)](https://crates.io/crates/jdcloud_signer)
[![Released API docs](https://docs.rs/jdcloud_signer/badge.svg)](https://docs.rs/jdcloud_signer)
[![codecov](https://codecov.io/gh/jdcloud-api/jdcloud-sdk-rust-signer/branch/master/graph/badge.svg)](https://codecov.io/gh/jdcloud-api/jdcloud-sdk-rust-signer)

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [Usage: 普通方式](#usage-%E6%99%AE%E9%80%9A%E6%96%B9%E5%BC%8F)
  - [Cargo.toml](#cargotoml)
  - [使用范例](#%E4%BD%BF%E7%94%A8%E8%8C%83%E4%BE%8B)
- [Usage: 只签名方式](#usage-%E5%8F%AA%E7%AD%BE%E5%90%8D%E6%96%B9%E5%BC%8F)
  - [Cargo.toml](#cargotoml-1)
  - [使用范例](#%E4%BD%BF%E7%94%A8%E8%8C%83%E4%BE%8B-1)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->


## Usage: 普通方式

### Cargo.toml

添加如下一段到你的 Cargo.toml

```
[dependencies]
jdcloud_signer = "0.1"
```

### 使用范例

详细范例参见 [client.rs](./examples/client.rs)

```sh
$ export JDCLOUD_AK="..."
$ export JDCLOUD_SK="..."
$ cargo run --example client
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/examples/client`
status: 200 OK
content-type: "application/json; charset=utf-8"
transfer-encoding: "chunked"
connection: "close"
date: "Mon, 22 Jul 2019 09:18:34 GMT"
x-jdcloud-request-id: "bkrovrdrv8ewru46782326noreauvdsf"
x-jdcloud-operationid: "describeInstances"
x-jdcloud-upstream-latency: "310"
x-jdcloud-proxy-latency: "30"
via: "jd-gateway/1.0.1"
requestId: "bkrovrdrv8ewru46782326noreauvdsf"
```

## Usage: 只签名方式

如果你不喜欢 `reqwest`, 准备使用自己的http库，那么可以选择只做签名。

签名时我们会添加如下几个 Header 字段

* `User-Agent`: 如果未指定，那么设为 "JdcloudSdkRust/0.1.0", 如果已指定，则不做改动。
* `X-Jdcloud-Date`: 当前时间。
* `X-Jdcloud-Nonce`: 随机数。
* `Authorization`: 签名。

### Cargo.toml

添加如下一段到你的 Cargo.toml

```
[dependencies]
jdcloud_signer = { version = "0.1", default-features = false }
```

### 使用范例

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
        .uri("https://vm.jdcloud-api.com/v1/regions/cn-north-1/instances")
        .body("".to_string()).unwrap();
    signer.sign_request(&mut req).unwrap();
    println!("{}", req);
}
```
