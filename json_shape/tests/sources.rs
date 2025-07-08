#![allow(missing_docs)]

use json_shape::JsonShape;

#[test]
fn from_sources_manages_option_in_tuples() {
    let tuple_1 = "[1, 2, \"string\", true]".to_string();
    let tuple_2 = "[1, 2, \"string\", null]".to_string();

    let shape = JsonShape::from_sources(&[tuple_1, tuple_2]).unwrap();

    assert_eq!(
        shape,
        JsonShape::Tuple {
            elements: vec![
                JsonShape::Number { optional: false },
                JsonShape::Number { optional: false },
                JsonShape::String { optional: false },
                JsonShape::Bool { optional: true },
            ],
            optional: false
        }
    );
}

#[test]
fn from_sources_manages_option_in_tuples_2() {
    let tuple_1 = "[1, 2, \"string\", true]".to_string();
    let tuple_2 = "[1, null, \"string\", false]".to_string();

    let shape = JsonShape::from_sources(&[tuple_1, tuple_2]).unwrap();

    assert_eq!(
        shape,
        JsonShape::Tuple {
            elements: vec![
                JsonShape::Number { optional: false },
                JsonShape::Number { optional: true },
                JsonShape::String { optional: false },
                JsonShape::Bool { optional: false },
            ],
            optional: false
        }
    );
}

#[test]
fn from_sources_tuples_become_array_when_differ() {
    let tuple_1 = "[1, 2, \"string\", true]".to_string();
    let tuple_2 = "[1, \"string\", 2, false]".to_string();

    let shape = JsonShape::from_sources(&[tuple_1, tuple_2]).unwrap();

    assert_eq!(
        shape,
        JsonShape::Array {
            r#type: Box::new(JsonShape::OneOf {
                variants: [
                    JsonShape::Bool { optional: false },
                    JsonShape::Number { optional: false },
                    JsonShape::String { optional: false }
                ]
                .into(),
                optional: false
            }),
            optional: false
        }
    );
}
