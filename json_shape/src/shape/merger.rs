use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
};

use crate::{IsSubset, error::Error, value::Value};

pub fn merge(values: &[Value]) -> Result<Value, Error> {
    let mut iter = values.iter();
    let first = iter.next().ok_or(Error::EmptyFile)?.to_owned();
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
        (Value::Null, Value::Tuple { elements, .. }) => Ok(Value::Tuple {
            elements,
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
        // Bool + Array = OneOf[Bool | Array]
        (
            Value::Bool { optional },
            Value::Tuple {
                elements,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::Tuple {
                            elements,
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
                        Value::Tuple {
                            elements,
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
            Value::Tuple {
                elements,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::Number { optional: false },
                        Value::Tuple {
                            elements,
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
                        Value::Tuple {
                            elements,
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
            Value::Tuple {
                elements,
                optional: other_opt,
            },
        ) => {
            if optional || other_opt {
                Ok(Value::OneOf {
                    variants: [
                        Value::String { optional: false },
                        Value::Tuple {
                            elements,
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
                        Value::Tuple {
                            elements,
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
            Value::Array {
                r#type,
                optional: opt,
            },
            Value::Tuple { elements, optional },
        ) => {
            let mut variants = BTreeSet::default();
            variants.insert(r#type.deref().clone());
            for element in elements {
                variants.insert(element.as_non_optional());
            }
            if opt || optional {
                Ok(Value::Array {
                    r#type: Box::new(Value::OneOf {
                        variants,
                        optional: false,
                    }),
                    optional: true,
                })
            } else {
                Ok(Value::Array {
                    r#type: Box::new(Value::OneOf {
                        variants,
                        optional: false,
                    }),
                    optional: false,
                })
            }
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
            Value::Tuple {
                elements,
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
                        Value::Tuple {
                            elements,
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
                        Value::Tuple {
                            elements,
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
            variants.insert(value.as_non_optional());
            Ok(Value::OneOf { variants, optional })
        }
        (Value::Tuple { elements, .. }, Value::Null) => Ok(Value::Tuple {
            elements,
            optional: true,
        }),
        (
            Value::Tuple { elements, optional },
            Value::Array {
                r#type,
                optional: opt,
            },
        ) => {
            let mut variants = BTreeSet::default();
            if elements.iter().any(Value::is_optional) || r#type.is_optional() {
                variants.insert(Value::Null);
            }
            variants.insert(r#type.deref().clone());
            for element in elements {
                variants.insert(element.as_non_optional());
            }

            if opt || optional {
                Ok(Value::Array {
                    r#type: Box::new(Value::OneOf {
                        variants,
                        optional: false,
                    }),
                    optional: true,
                })
            } else {
                Ok(Value::Array {
                    r#type: Box::new(Value::OneOf {
                        variants,
                        optional: false,
                    }),
                    optional: false,
                })
            }
        }
        (
            Value::Tuple { elements, optional },
            Value::Tuple {
                elements: other,
                optional: opt,
            },
        ) => {
            let folded = elements
                .iter()
                .zip(other.iter())
                .map(|(a, b)| {
                    if a.is_subset(b) {
                        Some(b.to_owned())
                    } else if b.is_subset(a) {
                        Some(a.to_owned())
                    } else if b.is_null() {
                        Some(a.clone().as_optional())
                    } else if a.is_null() {
                        Some(b.clone().as_optional())
                    } else {
                        None
                    }
                })
                .try_fold(Vec::new(), |mut acc, v| {
                    acc.push(v?);
                    Some(acc)
                });
            if let (true, Some(folded)) = (elements.len() == other.len(), folded) {
                Ok(Value::Tuple {
                    elements: folded,
                    optional: optional || opt,
                })
            } else {
                let mut variants = BTreeSet::default();
                if elements.iter().any(Value::is_optional) || other.iter().any(Value::is_optional) {
                    variants.insert(Value::Null);
                }
                for element in elements {
                    variants.insert(element.as_non_optional());
                }
                for element in other {
                    variants.insert(element.as_non_optional());
                }
                Ok(Value::Array {
                    r#type: Box::new(Value::OneOf {
                        variants,
                        optional: false,
                    }),
                    optional: optional || opt,
                })
            }
        }
        (Value::Tuple { elements, .. }, other) => {
            let mut variants: BTreeSet<Value> = BTreeSet::default();
            if elements.iter().any(Value::is_optional) || other.is_optional() {
                variants.insert(Value::Null);
            }
            variants.extend(elements.into_iter().map(Value::as_non_optional));
            variants.insert(other.as_non_optional());

            Ok(Value::OneOf {
                variants,
                optional: false,
            })
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

    // AI GENERATED
    #[test]
    fn merge_objects_with_different_keys() {
        let value_1 = Value::Object {
            content: [
                ("key1".to_string(), Value::String { optional: false }),
                ("key2".to_string(), Value::Number { optional: false }),
            ]
            .into(),
            optional: false,
        };
        let value_2 = Value::Object {
            content: [
                ("key3".to_string(), Value::Bool { optional: false }),
                (
                    "key4".to_string(),
                    Value::Array {
                        r#type: Box::new(Value::String { optional: false }),
                        optional: false,
                    },
                ),
            ]
            .into(),
            optional: false,
        };

        let expected = Value::Object {
            content: [
                ("key1".to_string(), Value::String { optional: true }),
                ("key2".to_string(), Value::Number { optional: true }),
                ("key3".to_string(), Value::Bool { optional: true }),
                (
                    "key4".to_string(),
                    Value::Array {
                        r#type: Box::new(Value::String { optional: false }),
                        optional: true,
                    },
                ),
            ]
            .into(),
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_arrays_with_different_types() {
        let value_1 = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };
        let value_2 = Value::Array {
            r#type: Box::new(Value::String { optional: false }),
            optional: false,
        };

        let expected = Value::Array {
            r#type: Box::new(Value::OneOf {
                variants: [
                    Value::Number { optional: false },
                    Value::String { optional: false },
                ]
                .into(),
                optional: false,
            }),
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_value_with_itself() {
        let value = Value::Number { optional: false };
        assert_eq!(merge(&[value.clone(), value.clone()]).unwrap(), value);
    }

    #[test]
    fn merge_empty_array_with_non_empty_array() {
        let value_1 = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };
        let value_2 = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };

        let expected = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_tuples_with_same_length() {
        let value_1 = Value::Tuple {
            elements: vec![
                Value::Number { optional: false },
                Value::String { optional: true },
            ],
            optional: false,
        };
        let value_2 = Value::Tuple {
            elements: vec![
                Value::Number { optional: true },
                Value::String { optional: false },
            ],
            optional: false,
        };

        let expected = Value::Tuple {
            elements: vec![
                Value::Number { optional: true },
                Value::String { optional: true },
            ],
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_tuples_with_same_length_optional_tuples() {
        let value_1 = Value::Tuple {
            elements: vec![
                Value::Number { optional: false },
                Value::String { optional: false },
            ],
            optional: false,
        };
        let value_2 = Value::Tuple {
            elements: vec![
                Value::Number { optional: false },
                Value::String { optional: false },
            ],
            optional: true,
        };

        let expected = Value::Tuple {
            elements: vec![
                Value::Number { optional: false },
                Value::String { optional: false },
            ],
            optional: true,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_tuples_with_different_lengths() {
        let value_1 = Value::Tuple {
            elements: vec![
                Value::Number { optional: false },
                Value::String { optional: false },
            ],
            optional: false,
        };
        let value_2 = Value::Tuple {
            elements: vec![
                Value::Bool { optional: false },
                Value::Number { optional: false },
                Value::String { optional: true },
            ],
            optional: false,
        };

        let expected = Value::Array {
            r#type: Box::new(Value::OneOf {
                variants: [
                    Value::Bool { optional: false },
                    Value::Number { optional: false },
                    Value::String { optional: false },
                    Value::Null,
                ]
                .into(),
                optional: false,
            }),
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_tuple_with_array() {
        let value_1 = Value::Tuple {
            elements: vec![
                Value::Number { optional: false },
                Value::String { optional: false },
            ],
            optional: false,
        };
        let value_2 = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };

        let expected = Value::Array {
            r#type: Box::new(Value::OneOf {
                variants: [
                    Value::Number { optional: false },
                    Value::String { optional: false },
                ]
                .into(),
                optional: false,
            }),
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_arrays_with_same_type() {
        let value_1 = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };
        let value_2 = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };

        let expected = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }

    #[test]
    fn merge_arrays_with_same_type_but_one_optional() {
        let value_1 = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };
        let value_2 = Value::Array {
            r#type: Box::new(Value::Number { optional: true }),
            optional: false,
        };

        let expected = Value::Array {
            r#type: Box::new(Value::Number { optional: true }),
            optional: false,
        };

        assert_eq!(merge(&[value_1, value_2]).unwrap(), expected);
    }
}
