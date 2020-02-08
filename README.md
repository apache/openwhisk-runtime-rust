<!--
#
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.
# The ASF licenses this file to You under the Apache License, Version 2.0
# (the "License"); you may not use this file except in compliance with
# the License.  You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
-->
# Apache OpenWhisk Runtime for Rust

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![Build Status](https://travis-ci.org/apache/openwhisk-runtime-rust.svg?branch=master)](https://travis-ci.org/apache/openwhisk-runtime-rust)

### Give it a try today
To use as a Docker action:

```
wsk action update myAction my_action.rs --docker openwhisk/action-rust-v1.34
```

The file `my_action.rs` looks like:

```
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    #[serde(default = "stranger")]
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Output {
    body: String,
}

fn stranger() -> String {
    "stranger".to_string()
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input: Input = serde_json::from_value(args)?;
    let output = Output {
        body: format!("Hello, {}", input.name),
    };
    serde_json::to_value(output)
}
```

The action is mainly composed by a `main` function that accepts a JSON `serdes Value` as input and returns a `Result` including a JSON `serde Value`.

### Managing dependencies

If your action needs external dependencies, you need to provide a zip file including your source and your cargo file with all your dependencies. The folder structure is the following:
```
|- Cargo.toml
|- src
    |- lib.rs
```
Here is an example of a `Cargo.toml` file
```
[package]
name = "actions"
version = "0.1.0"
authors = ["John Doe <john@doe.domain>"]
edition = "2018"

[dependencies]
serde_json = "1.0"
serde = "1.0"
serde_derive = "1.0"
```
Once you have all your code zipped in a file with the showed folder structure you can generate your action with the following command:
```
wsk action create yourAction /full_path_to/yourCode.zip --docker openwhisk/action-rust-v1.34
```
