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
        Self::from(&value)
    }
}

impl From<&serde_json::Value> for JsonShape {
    fn from(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Self::Null,
            serde_json::Value::Bool(_) => Self::Bool { optional: false },
            serde_json::Value::Number(_) => Self::Number { optional: false },
            serde_json::Value::String(_) => Self::String { optional: false },
            serde_json::Value::Array(values) => {
                if values.len() > 1
                    && values
                        .iter()
                        .map(Self::from)
                        .all(|value| matches!(value, Self::Object { .. }))
                {
                    let mut iter = values.iter().map(Self::from);
                    let Some(Self::Object { content, .. }) = iter.next() else {
                        unreachable!("Guaranteed to be Object by all");
                    };
                    let content = iter
                        .clone()
                        .filter(|shape| matches!(shape, Self::Object { .. }))
                        .filter_map(|content| {
                            content.keys().map(|keys| keys.cloned().collect::<Vec<_>>())
                        })
                        .fold(content, |mut acc, keys| {
                            for (key, value) in &mut acc {
                                if !keys.iter().any(|k| k == key) {
                                    value.to_optional_mut();
                                }
                            }
                            acc
                        });
                    let object = iter.fold(content, |mut acc, content| {
                        let Self::Object { content, .. } = content else {
                            return acc;
                        };
                        for (key, value) in content {
                            let old_value = acc
                                .entry(key.clone())
                                .or_insert_with(|| value.clone().as_optional());
                            if let Self::OneOf { variants, .. } = old_value {
                                variants.insert(value.clone());
                            }
                        }
                        acc
                    });

                    Self::Array {
                        r#type: Box::new(Self::Object {
                            content: object,
                            optional: false,
                        }),
                        optional: false,
                    }
                } else if values.len() == 1
                    || values
                        .windows(2)
                        .map(|val| {
                            (
                                Self::from(val.first().unwrap()),
                                Self::from(val.get(1).unwrap()),
                            )
                        })
                        .all(|val| val.0 == val.1)
                {
                    Self::Array {
                        r#type: Box::new(Self::from(values[0].clone())),
                        optional: false,
                    }
                } else if values.len() > 1 {
                    Self::Tuple {
                        elements: values.iter().map(Self::from).collect(),
                        optional: false,
                    }
                } else {
                    Self::Array {
                        r#type: Box::new(Self::Null),
                        optional: true,
                    }
                }
            }
            serde_json::Value::Object(map) => Self::Object {
                content: map
                    .into_iter()
                    .map(|(k, v)| (k.clone(), Self::from(v)))
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
