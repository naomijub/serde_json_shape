use std::collections::BTreeMap;

use crate::{error::Error, value::Value};

pub fn merge(values: &[Value]) -> Result<Value, Error> {
    let mut iter = values.iter();
    let first = iter.next().ok_or(Error::Unknown)?.to_owned();
    iter.try_fold(first, |acc, v| merger(acc, v.to_owned()))
}

#[expect(clippy::match_same_arms)]
#[expect(clippy::too_many_lines)]
#[expect(clippy::cognitive_complexity)]
pub fn merger(rhs: Value, lhs: Value) -> Result<Value, Error> {
    match (rhs, lhs) {
        // Null + Null = Null
        (Value::Null, Value::Null) => Ok(Value::Null),
        // Null + T = Option<T>
        (Value::Null, Value::Bool { .. }) => Ok(Value::Bool { optional: true }),
        (Value::Null, Value::Number { .. }) => Ok(Value::Number { optional: true }),
        (Value::Null, Value::String { .. }) => Ok(Value::String { optional: true }),
        (Value::Null, Value::Array { r#type, .. }) => Ok(Value::Array {
            r#type,
            optional: true,
        }),
        (Value::Null, Value::Object { content, .. }) => Ok(Value::Object {
            content,
            optional: true,
        }),
        (Value::Null, Value::OneOf { variants, .. }) => Ok(Value::OneOf {
            variants,
            optional: true,
        }),
        (Value::Bool { .. }, Value::Null) => Ok(Value::Bool { optional: true }),
        // Bool + Option<Bool> = Option<Bool>
        (
            Value::Bool { optional },
            Value::Bool {
                optional: other_opt,
            },
        ) => Ok(Value::Bool {
            optional: (optional || other_opt),
        }),
        // Bool + Number = OneOf[Bool | Number]
        (
            Value::Bool { optional },
            Value::Number {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::Number { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::Number { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        // Bool + String = OneOf[Bool | String]
        (
            Value::Bool { optional },
            Value::String {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::String { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::String { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        // Bool + Array = OneOf[Bool | Array]
        (
            Value::Bool { optional },
            Value::Array {
                r#type,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        // Bool + Object = OneOf[Bool | Object]
        (
            Value::Bool { optional },
            Value::Object {
                content,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::Object {
                            content,
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        // Bool + Oneof[T | U] = OneOf[Bool | T | U]
        // Bool + Oneof[Bool | T | U] = OneOf[Bool | T | U]
        // Option<Bool> + Oneof[Bool | T | U] = OneOf[Bool | T | U | Null]
        (
            Value::Bool { optional },
            Value::OneOf {
                mut variants,
                optional: other_opt,
            },
        ) => {
            if optional && !variants.contains(&Value::Null) {
                variants.insert(Value::Null);
            }
            variants.insert(Value::Bool { optional: false });
            Ok(Value::OneOf {
                variants,
                optional: other_opt,
            })
        }
        (Value::Number { .. }, Value::Null) => Ok(Value::Number { optional: true }),
        (
            Value::Number { optional },
            Value::Bool {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::Number { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::Number { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Number { optional },
            Value::Number {
                optional: other_opt,
            },
        ) => Ok(Value::Number {
            optional: (optional || other_opt),
        }),
        (
            Value::Number { optional },
            Value::String {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::String { optional: false },
                        Value::Number { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::String { optional: false },
                        Value::Number { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Number { optional },
            Value::Array {
                r#type,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Number { optional: false },
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Number { optional: false },
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Number { optional },
            Value::Object {
                content,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Number { optional: false },
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Number { optional: false },
                        Value::Object {
                            content,
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Number { optional },
            Value::OneOf {
                mut variants,
                optional: other_opt,
            },
        ) => {
            if optional && !variants.contains(&Value::Null) {
                variants.insert(Value::Null);
            }
            variants.insert(Value::Number { optional: false });
            Ok(Value::OneOf {
                variants,
                optional: other_opt,
            })
        }
        (Value::String { .. }, Value::Null) => Ok(Value::String { optional: true }),
        (
            Value::String { optional },
            Value::Bool {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::String { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::String { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::String { optional },
            Value::Number {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Number { optional: false },
                        Value::String { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Number { optional: false },
                        Value::String { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::String { optional },
            Value::String {
                optional: other_opt,
            },
        ) => Ok(Value::String {
            optional: (optional || other_opt),
        }),
        (
            Value::String { optional },
            Value::Array {
                r#type,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::String { optional: false },
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::String { optional: false },
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::String { optional },
            Value::Object {
                content,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::String { optional: false },
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::String { optional: false },
                        Value::Object {
                            content,
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::String { optional },
            Value::OneOf {
                mut variants,
                optional: other_opt,
            },
        ) => {
            if optional && !variants.contains(&Value::Null) {
                variants.insert(Value::Null);
            }
            variants.insert(Value::String { optional: false });
            Ok(Value::OneOf {
                variants,
                optional: other_opt,
            })
        }
        (Value::Array { r#type, .. }, Value::Null) => Ok(Value::Array {
            r#type,
            optional: true,
        }),
        (
            Value::Array { r#type, optional },
            Value::Bool {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Bool { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Bool { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Array { r#type, optional },
            Value::Number {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Number { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Number { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Array { r#type, optional },
            Value::String {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::String { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::String { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Array { r#type, optional },
            Value::Array {
                r#type: other_ty,
                optional: other_opt,
            },
        ) => {
            let ty = merger(*r#type, *other_ty)?;
            Ok(Value::Array {
                r#type: Box::new(ty),
                optional: optional || other_opt,
            })
        }
        (
            Value::Array { r#type, optional },
            Value::Object {
                content,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Object {
                            content,
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Array { r#type, optional },
            Value::OneOf {
                mut variants,
                optional: other_opt,
            },
        ) => {
            if optional && !variants.contains(&Value::Null) {
                variants.insert(Value::Null);
            }
            variants.insert(Value::Array {
                r#type,
                optional: false,
            });
            Ok(Value::OneOf {
                variants,
                optional: other_opt,
            })
        }
        (Value::Object { content, .. }, Value::Null) => Ok(Value::Object {
            content,
            optional: true,
        }),
        (
            Value::Object { content, optional },
            Value::Bool {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Bool { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Bool { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Object { content, optional },
            Value::Number {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Number { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Number { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Object { content, optional },
            Value::String {
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::String { optional: false },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::String { optional: false },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Object { content, optional },
            Value::Array {
                r#type,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                        Value::Null,
                    ]
                    .into(),
                    optional: false,
                })
            } else {
                Ok(Value::OneOf {
                    variants: [
                        Value::Object {
                            content,
                            optional: false,
                        },
                        Value::Array {
                            r#type,
                            optional: false,
                        },
                    ]
                    .into(),
                    optional: false,
                })
            }
        }
        (
            Value::Object { content, optional },
            Value::Object {
                content: mut other_content,
                optional: other_opt,
            },
        ) => {
            let mut map = BTreeMap::default();

            for (key, value) in content {
                if let Some(other_value) = other_content.remove(&key) {
                    let v = merger(value, other_value)?;
                    map.insert(key, v);
                } else {
                    map.insert(key, value.as_optional());
                }
            }

            for (key, value) in other_content {
                map.insert(key, value.as_optional());
            }

            Ok(Value::Object {
                content: map,
                optional: optional || other_opt,
            })
        }
        (
            Value::Object { content, optional },
            Value::OneOf {
                mut variants,
                optional: other_opt,
            },
        ) => {
            if optional && !variants.contains(&Value::Null) {
                variants.insert(Value::Null);
            }
            variants.insert(Value::Object {
                content,
                optional: false,
            });
            Ok(Value::OneOf {
                variants,
                optional: other_opt,
            })
        }
        (Value::OneOf { variants, .. }, Value::Null) => Ok(Value::OneOf {
            variants,
            optional: true,
        }),
        (
            Value::OneOf {
                mut variants,
                optional,
            },
            Value::OneOf {
                variants: other_var,
                optional: other_opt,
            },
        ) => {
            variants.extend(other_var);
            Ok(Value::OneOf {
                variants,
                optional: optional || other_opt,
            })
        }
        (
            Value::OneOf {
                mut variants,
                optional,
            },
            value,
        ) => {
            if value.is_optional() && !variants.contains(&Value::Null) {
                variants.insert(Value::Null);
            }
            println!("Value: {value}");
            variants.insert(value.as_non_optional());
            Ok(Value::OneOf { variants, optional })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn merge_simple_json_objects_as_one_of() {
        let value_1 = Value::Null;
        let value_2 = Value::Bool { optional: false };
        let value_3 = Value::Number { optional: false };
        let value_4 = Value::String { optional: false };

        let result = merge(&[value_1, value_2, value_3, value_4]).unwrap();

        assert_eq!(
            result,
            Value::OneOf {
                variants: BTreeSet::from_iter([
                    Value::Null,
                    Value::Bool { optional: false },
                    Value::Number { optional: false },
                    Value::String { optional: false }
                ]),
                optional: false
            }
        );
    }

    #[test]
    fn merge_simple_json_objects_with_optional_as_one_of() {
        let value_2 = Value::Bool { optional: false };
        let value_3 = Value::Number { optional: false };
        let value_4 = Value::String { optional: false };
        let value_5 = Value::Number { optional: true };

        let result = merge(&[value_2, value_3, value_4, value_5]).unwrap();

        assert_eq!(
            result,
            Value::OneOf {
                variants: BTreeSet::from_iter([
                    Value::Null,
                    Value::Bool { optional: false },
                    Value::Number { optional: false },
                    Value::String { optional: false }
                ]),
                optional: false
            }
        );
    }

    #[test]
    fn merge_one_ofs() {
        // Option<[Number | Bool]> + [Number | String] => Option<[Number | Bool | String]>
        let value_1 = Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::Bool { optional: false },
            ]
            .into(),
            optional: true,
        };
        let value_2 = Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::String { optional: false },
            ]
            .into(),
            optional: false,
        };

        let expected = Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::Bool { optional: false },
                Value::String { optional: false },
            ]
            .into(),
            optional: true,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_objects() {
        let value_1 = Value::Object {
            content: [
                ("same_kv".to_string(), Value::Bool { optional: false }),
                (
                    "same_k_diff_v".to_string(),
                    Value::Number { optional: false },
                ),
                ("key".to_string(), Value::String { optional: false }),
            ]
            .into(),
            optional: false,
        };
        let value_2 = Value::Object {
            content: [
                ("same_kv".to_string(), Value::Bool { optional: false }),
                (
                    "same_k_diff_v".to_string(),
                    Value::String { optional: false },
                ),
                ("other_key".to_string(), Value::Bool { optional: false }),
            ]
            .into(),
            optional: false,
        };

        let expected = Value::Object {
            content: [
                ("same_kv".to_string(), Value::Bool { optional: false }),
                (
                    "same_k_diff_v".to_string(),
                    Value::OneOf {
                        variants: [
                            Value::String { optional: false },
                            Value::Number { optional: false },
                        ]
                        .into(),
                        optional: false,
                    },
                ),
                ("other_key".to_string(), Value::Bool { optional: true }),
                ("key".to_string(), Value::String { optional: true }),
            ]
            .into(),
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_diff_arrays() {
        let value_1 = Value::Array {
            r#type: Box::new(Value::String { optional: true }),
            optional: true,
        };
        let value_2 = Value::Array {
            r#type: Box::new(Value::Bool { optional: false }),
            optional: false,
        };

        let expected = Value::Array {
            r#type: Box::new(Value::OneOf {
                variants: [
                    Value::String { optional: false },
                    Value::Bool { optional: false },
                    Value::Null,
                ]
                .into(),
                optional: false,
            }),
            optional: true,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_same_arrays() {
        let value_1 = Value::Array {
            r#type: Box::new(Value::String { optional: true }),
            optional: false,
        };
        let value_2 = Value::Array {
            r#type: Box::new(Value::String { optional: false }),
            optional: false,
        };

        let expected = Value::Array {
            r#type: Box::new(Value::String { optional: true }),
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }
}
