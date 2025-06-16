
[![Latest Version](https://img.shields.io/crates/v/json_shape)](https://crates.io/crates/json_shape)
[![License:Apache](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://github.com/naomijub/serde_json_shape/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/naomijub/serde_json_shape/actions/workflows/rust.yml)
[![Coverage Status](https://coveralls.io/repos/github/naomijub/serde_json_shape/badge.svg)](https://coveralls.io/github/naomijub/serde_json_shape)

# JSON_Shape

This libraries is not intended to serialize a JSON into a value representation like [`serde_json`](https://crates.io/crates/serde_json) does but to represent that types of data that a json or multiple jsons have:

```json
{
    "str": "this is a string",
    "number": 123.456,
    "bool_true": true,
    "bool_false": false,
    "nil": null,
    "array": [123, "string", true],
    "map": {
        "a": "b",
        "c": 123
    },
    "array of maps": [
        {
            "a": "b",
            "c": 123
        },
        {
            "a": "b",
            "b": true
        }
    ]
}
```

Will be parsed as:

```ru
Object{
    "array": Array<OneOf[Boolean | Number | String]>,
    "array of maps": Array<Object{
        "a": String, 
        "b": Option<Boolean>, 
        "c": Option<Number>
    }>, 
    "bool_false": Boolean, 
    "bool_true": Boolean, 
    "map": Object{
        "a": String, 
        "c": Number
    }, 
    "nil": Null, 
    "number": Number, 
    "str": String
}
```

> ### Usage Warning
>
> This library does not conform to Swagger or JsonSchema specifications, as they are signiticantly more complex than the intended usage for this library.


## Installation
Run the following Cargo command in your project directory:

```shell
$ cargo add json_shape
```

Or add the following line to your Cargo.toml:

```toml
[dependencies]
json_shape = "0.1"
```

## Usage 

```rust
use json_shape::JsonShape;
use std::str::FromStr;

let source = r#"{
        "str": "this is a string",
        "number": 123.456,
        "bool_true": true,
        "bool_false": false,
        "nil": null,
        "array": [123, "string", true],
        "map": {
          "a": "b",
          "c": 123
        },
        "array of maps": [
            {
                "a": "b",
                "c": 123
            },
            {
                "a": "b",
                "b": true
            }
        ]
    }"#;

let json_shape = JsonShape::from_str(source).unwrap();
```
