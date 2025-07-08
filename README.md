
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
    "array": [1, 2, 3, 4],
    "bool_true": true,
    "bool_false": false,
    "nil": null,
    "tuple": [123, "string", true],
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
    array: Array<Number>,
    "array of maps": Array<Object{
        a: String, 
        b: Option<Boolean>, 
        c: Option<Number>
    }>, 
    bool_false: Boolean, 
    bool_true: Boolean, 
    map: Object{
        a: String, 
        c: Number
    }, 
    nil: Null, 
    number: Number, 
    str: String,
    tuple: Tuple(Boolean, Number, String)
}
```

### General rules when merging two [`JsonShape`]:
- `T + Null = Option<T>`
- `T + U = OneOf[T | U]`
- `T + Option<U> = OneOf[T | U | Null]`
- `Tuple(U, T, V) + Tuple(U, T, Null) = Tuple(U, T, Option<V>)`
- `Array<T> + Array<U> => Array<OneOf[T | U]>`
- `Tuple(U, T, V) + Array<U> = Array<OneOf[T | U | V]>`
- `Object{key: Number, "key space": Bool} +  Object{key: String, "key_special_char?": String} => Object{key: OneOf[Number | String], "key space": Option<Bool>, "key_special_char?": Option<String> }`
- `OneOf[T | U] + OneOf[V | X] = OneOf[T | U | V | X]`
- `OneOf[T | U] + Option<U> = OneOf[T | U | Null]`

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

### From `String`
```rust
use json_shape::JsonShape;
use std::str::FromStr;

let source = r#"{
        "str": "this is a string",
        "number": 123.456,
        "bool_true": true,
        "bool_false": false,
        "nil": null,
        "tuple": [123, "string", true],
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

* If multiple `JSON` sources are available, you may use [`JsonShape::from_sources`](https://docs.rs/json_shape/latest/json_shape/enum.JsonShape.html#method.from_sources), which expects a list of Json strings.

### From `serde_json::Value`

```rust
use std::{fs::read_to_string, str::FromStr};

use json_shape::JsonShape;
use serde_json::Value;

let json_str = read_to_string("./testdata/rfc-9535-example-1.json").unwrap();
let json: Value = serde_json::from_str(&json_str).unwrap();
let shape_from_value = JsonShape::from(&json);
```