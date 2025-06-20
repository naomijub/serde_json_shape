#![allow(missing_docs)]

use std::str::FromStr;

use insta::assert_snapshot;
use json_shape::{JsonShape, error::Error};

#[test]
fn parse_nil_shape() {
    let source = "null";
    let value = JsonShape::from_str(source).unwrap();

    assert_eq!(value, JsonShape::Null);
}

#[test]
fn parse_true_shape() {
    let source = "true";
    let value = JsonShape::from_str(source).unwrap();

    assert_eq!(value, JsonShape::Bool { optional: false });
}

#[test]
fn parse_false_shape() {
    let source = "false";
    let value = JsonShape::from_str(source).unwrap();

    assert_eq!(value, JsonShape::Bool { optional: false });
}

#[test]
fn parse_number_shape() {
    let source = "123.456";
    let value = JsonShape::from_str(source).unwrap();

    assert_eq!(value, JsonShape::Number { optional: false });
}

#[test]
fn parse_string_shape() {
    let source = "\"this is a string\"";
    let value = JsonShape::from_str(source).unwrap();

    assert_eq!(value, JsonShape::String { optional: false });
}

#[test]
fn parse_something_else_shape_error() {
    let source = "random";
    let err = JsonShape::from_str(source).unwrap_err();

    assert_eq!(err, Error::InvalidJson("random".to_string(), 0..6));
}

#[test]
fn parse_array_shape() {
    let source = "[\"string\", 123.456, 234, true, false, null]";
    let value = JsonShape::from_str(source).unwrap();

    assert_eq!(
        value,
        JsonShape::Tuple {
            elements: vec![
                JsonShape::String { optional: false },
                JsonShape::Number { optional: false },
                JsonShape::Number { optional: false },
                JsonShape::Bool { optional: false },
                JsonShape::Bool { optional: false },
                JsonShape::Null,
            ],
            optional: false
        }
    );
}

#[test]
fn parse_map_shape() {
    let source = "{
    \"a\" : 1234,
    \"b\": true
    }";
    let value = JsonShape::from_str(source).unwrap();
    let content = [
        (
            String::from_str("a").unwrap(),
            JsonShape::Number { optional: false },
        ),
        (
            String::from_str("b").unwrap(),
            JsonShape::Bool { optional: false },
        ),
    ]
    .into();
    assert_eq!(
        value,
        JsonShape::Object {
            content,
            optional: false
        }
    );
}

#[test]
fn complex_json_shape() {
    let source = r#"{
        "str": "this is a string",
        "number": 123.456,
        "bool_true": true,
        "bool_false": false,
        "nil": null,
        "array": [1, 2, 3, 4],
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

    let value = JsonShape::from_str(source).unwrap();
    assert_eq!(
        value,
        JsonShape::Object {
            content: [
                (
                    "array".to_string(),
                    JsonShape::Array {
                        r#type: Box::new(JsonShape::Number { optional: false }),
                        optional: false
                    }
                ),
                (
                    "tuple".to_string(),
                    JsonShape::Tuple {
                        elements: vec![
                            JsonShape::Number { optional: false },
                            JsonShape::String { optional: false },
                            JsonShape::Bool { optional: false }
                        ],
                        optional: false
                    },
                ),
                (
                    "array of maps".to_string(),
                    JsonShape::Array {
                        r#type: Box::new(JsonShape::Object {
                            content: [
                                ("a".to_string(), JsonShape::String { optional: false }),
                                ("c".to_string(), JsonShape::Number { optional: true }),
                                ("b".to_string(), JsonShape::Bool { optional: true })
                            ]
                            .into(),
                            optional: false
                        }),
                        optional: false
                    }
                ),
                (
                    "bool_false".to_string(),
                    JsonShape::Bool { optional: false },
                ),
                ("bool_true".to_string(), JsonShape::Bool { optional: false },),
                (
                    "map".to_string(),
                    JsonShape::Object {
                        content: [
                            ("a".to_string(), JsonShape::String { optional: false }),
                            ("c".to_string(), JsonShape::Number { optional: false })
                        ]
                        .into(),
                        optional: false
                    }
                ),
                ("nil".to_string(), JsonShape::Null,),
                ("number".to_string(), JsonShape::Number { optional: false },),
                ("str".to_string(), JsonShape::String { optional: false })
            ]
            .into(),
            optional: false
        }
    );
    assert_snapshot!(value);
}

#[test]
fn parse_duplicated_keys_object_shape() {
    let source = "{\"a\": 123, \"a\": true}";
    let err = JsonShape::from_str(source).unwrap_err();

    assert_eq!(
        err.to_string(),
        "invalid type `Boolean`. Expected `Number`."
    );
}

#[test]
fn complex_json_shape_from_sources() {
    let source_1 = r#"{
        "str": "this is a string",
        "number": 123.456,
        "nil": null,
        "array": [123, "string", true],
        "map": {
          "a": "b",
          "c": 123,
          "d": 1
        },
        "array of maps": [
            {
                "a": "b",
                "c": 123
            }
        ]
    }"#;

    let source_2 = r#"{
        "str": "this is a string",
        "bool": true,
        "nil": null,
        "array": [123, "string"],
        "map": {
          "a": "b",
          "c": 123,
          "e": 2
        },
        "array of maps": [
            {
                "a": "b",
                "b": true
            }
        ]
    }"#;

    let shape = JsonShape::from_sources(&[source_1, source_2]).unwrap();

    assert_snapshot!(shape);
}
