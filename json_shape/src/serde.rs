#![allow(clippy::fallible_impl_from)]
use crate::Value as JsonShape;

pub(crate) mod impls;

/// Visitor navigating a [`serde_json::Value`] based on the respective [`JsonShape`]
pub struct JsonVisitor<'json> {
    value: &'json serde_json::Value,
    shape: JsonShape,
}

impl From<serde_json::Value> for JsonShape {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => JsonShape::Null,
            serde_json::Value::Bool(_) => JsonShape::Bool { optional: false },
            serde_json::Value::Number(_) => JsonShape::Number { optional: false },
            serde_json::Value::String(_) => JsonShape::String { optional: false },
            serde_json::Value::Array(values) => {
                if values
                    .windows(2)
                    .map(|val| {
                        (
                            JsonShape::from(val.first().unwrap()),
                            JsonShape::from(val.get(1).unwrap()),
                        )
                    })
                    .all(|val| val.0 == val.1)
                {
                    JsonShape::Array {
                        r#type: Box::new(JsonShape::from(values.first().unwrap())),
                        optional: false,
                    }
                } else {
                    JsonShape::Tuple {
                        elements: values.into_iter().map(JsonShape::from).collect(),
                        optional: false,
                    }
                }
            }
            serde_json::Value::Object(map) => JsonShape::Object {
                content: map
                    .into_iter()
                    .map(|(k, v)| (k, JsonShape::from(v)))
                    .collect(),
                optional: false,
            },
        }
    }
}

impl From<&serde_json::Value> for JsonShape {
    fn from(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => JsonShape::Null,
            serde_json::Value::Bool(_) => JsonShape::Bool { optional: false },
            serde_json::Value::Number(_) => JsonShape::Number { optional: false },
            serde_json::Value::String(_) => JsonShape::String { optional: false },
            serde_json::Value::Array(values) => {
                if values
                    .windows(2)
                    .map(|val| {
                        (
                            JsonShape::from(val.first().unwrap()),
                            JsonShape::from(val.get(1).unwrap()),
                        )
                    })
                    .all(|val| val.0 == val.1)
                {
                    JsonShape::Array {
                        r#type: Box::new(JsonShape::from(values[0].clone())),
                        optional: false,
                    }
                } else {
                    JsonShape::Tuple {
                        elements: values.iter().map(JsonShape::from).collect(),
                        optional: false,
                    }
                }
            }
            serde_json::Value::Object(map) => JsonShape::Object {
                content: map
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), JsonShape::from(v)))
                    .collect(),
                optional: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use serde_json::json;

    #[test]
    fn test_from_json_null() {
        let json = json!(null);
        let ref_shape = JsonShape::from(&json);
        let shape = JsonShape::from(json);
        assert_eq!(shape, JsonShape::Null);
        assert_eq!(ref_shape, JsonShape::Null);
    }

    #[test]
    fn test_from_json_number() {
        let json = json!(123.456);
        let ref_shape = JsonShape::from(&json);
        let shape = JsonShape::from(json);
        assert_eq!(shape, JsonShape::Number { optional: false });
        assert_eq!(ref_shape, JsonShape::Number { optional: false });
    }

    #[test]
    fn test_from_json_string() {
        let json = json!("string");
        let ref_shape = JsonShape::from(&json);
        let shape = JsonShape::from(json);
        assert_eq!(shape, JsonShape::String { optional: false });
        assert_eq!(ref_shape, JsonShape::String { optional: false });
    }

    #[test]
    fn test_from_json_bool() {
        let json = json!(true);
        let ref_shape = JsonShape::from(&json);
        let shape = JsonShape::from(json);
        assert_eq!(shape, JsonShape::Bool { optional: false });
        assert_eq!(ref_shape, JsonShape::Bool { optional: false });
    }

    #[test]
    fn test_from_json_array() {
        let json = json!([1, 2, 3]);
        let ref_shape = JsonShape::from(&json);
        let shape = JsonShape::from(json);
        assert_eq!(
            shape,
            JsonShape::Array {
                r#type: Box::new(JsonShape::Number { optional: false }),
                optional: false
            }
        );
        assert_eq!(
            ref_shape,
            JsonShape::Array {
                r#type: Box::new(JsonShape::Number { optional: false }),
                optional: false
            }
        );
    }

    #[test]
    fn test_from_json_tuple() {
        let json = json!([1, "string", true, null]);
        let ref_shape = JsonShape::from(&json);
        let shape = JsonShape::from(json);
        assert_eq!(
            shape,
            JsonShape::Tuple {
                elements: vec![
                    JsonShape::Number { optional: false },
                    JsonShape::String { optional: false },
                    JsonShape::Bool { optional: false },
                    JsonShape::Null
                ],
                optional: false
            }
        );
        assert_eq!(
            ref_shape,
            JsonShape::Tuple {
                elements: vec![
                    JsonShape::Number { optional: false },
                    JsonShape::String { optional: false },
                    JsonShape::Bool { optional: false },
                    JsonShape::Null
                ],
                optional: false
            }
        );
    }

    #[test]
    fn test_from_json_object() {
        let json = json!({
            "a": 1,
            "b": "string",
            "c": [12, 34, 56],
            "d": null,
            "e": true,
            "f": [12, true, "string"],
            "objs": [
                {
                    "a": 1,
                    "b" : "string"
                },
                {
                    "a": 2,
                    "c" : true
                }
            ],
            "obj": {
                "d": 1,
                "e" : "string"
            }
        });
        let shape = JsonShape::from(json);
        assert_snapshot!(shape.to_string());
    }
}
