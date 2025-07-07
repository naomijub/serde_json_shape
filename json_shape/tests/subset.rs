#![allow(missing_docs)]

use std::str::FromStr;

use json_shape::{IsSubset, JsonShape, error::Error};

#[test]
fn json_is_subset_of_json_shape() {
    let shape = JsonShape::Object {
        content: [
            ("name".to_string(), JsonShape::String { optional: false }),
            ("surname".to_string(), JsonShape::String { optional: false }),
            (
                "middle name".to_string(),
                JsonShape::String { optional: true },
            ),
            ("age".to_string(), JsonShape::Number { optional: false }),
            (
                "id".to_string(),
                JsonShape::OneOf {
                    variants: [
                        JsonShape::Object {
                            content: [
                                ("number".to_string(), JsonShape::Number { optional: false }),
                                ("state".to_string(), JsonShape::String { optional: false }),
                            ]
                            .into(),
                            optional: false,
                        },
                        JsonShape::Array {
                            r#type: Box::new(JsonShape::Number { optional: false }),
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                },
            ),
        ]
        .into(),
        optional: false,
    };

    let json_1 = r#"{
    "name": "lorem",
    "surname": "ipsum",
    "age": 30,
    "id": {
        "number": 123456,
        "state": "st"
    }
}"#;

    let shape_1 = JsonShape::from_str(json_1).unwrap();

    assert!(shape_1.is_subset(&shape));
    assert!(shape.is_superset(json_1));
}

#[test]
fn json_is_subset_of_json_shape_checked() {
    let shape = JsonShape::Object {
        content: [
            ("name".to_string(), JsonShape::String { optional: false }),
            ("surname".to_string(), JsonShape::String { optional: false }),
            (
                "middle name".to_string(),
                JsonShape::String { optional: true },
            ),
            ("age".to_string(), JsonShape::Number { optional: false }),
            (
                "id".to_string(),
                JsonShape::OneOf {
                    variants: [
                        JsonShape::Object {
                            content: [
                                ("number".to_string(), JsonShape::Number { optional: false }),
                                ("state".to_string(), JsonShape::String { optional: false }),
                            ]
                            .into(),
                            optional: false,
                        },
                        JsonShape::Array {
                            r#type: Box::new(JsonShape::Number { optional: false }),
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                },
            ),
        ]
        .into(),
        optional: false,
    };

    let json_1 = r#"{
    "name": "lorem",
    "surname": "ipsum",
    "age": 30,
    "id": {
        "number": 123456,
        "state": "st"
    }
}"#;

    assert!(shape.is_superset_checked(json_1).unwrap());
}

#[test]
fn json_is_subset_of_json_shape_checked_json_error() {
    let shape = JsonShape::Object {
        content: [
            ("name".to_string(), JsonShape::String { optional: false }),
            ("surname".to_string(), JsonShape::String { optional: false }),
            (
                "middle name".to_string(),
                JsonShape::String { optional: true },
            ),
            ("age".to_string(), JsonShape::Number { optional: false }),
            (
                "id".to_string(),
                JsonShape::OneOf {
                    variants: [
                        JsonShape::Object {
                            content: [
                                ("number".to_string(), JsonShape::Number { optional: false }),
                                ("state".to_string(), JsonShape::String { optional: false }),
                            ]
                            .into(),
                            optional: false,
                        },
                        JsonShape::Array {
                            r#type: Box::new(JsonShape::Number { optional: false }),
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                },
            ),
        ]
        .into(),
        optional: false,
    };

    let json_1 = r#"{
    "name": "lorem",
    "surname": "ipsum",
    "age": 30,
    "id": {
        "number": 123456,
        "state": "st
    }
}"#;

    assert_eq!(
        shape.is_superset_checked(json_1).unwrap_err(),
        Error::InvalidJson {
            value: "\"st\n    }\n}".to_string(),
            span: 117..128
        }
    );
}
