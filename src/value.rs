#![allow(clippy::match_same_arms)]
use std::{
    collections::{BTreeMap, BTreeSet, btree_map::Keys},
    fmt::Display,
};

use serde::{Deserialize, Serialize};

pub mod subset;
pub mod subtypes;

/// Helper trait to identify when two `JsonShapes` are similar but not necessarily equal, meaning they only diverge in being optional.
pub trait Similar<Rhs: ?Sized = Self> {
    /// Tests for `self` and `other` values to be similar (equal ignoring the optional), returning the optional version
    #[must_use]
    fn similar(&self, other: &Rhs) -> Option<Value>;
}

/// Represents any valid JSON value shape.
///
/// See the [`serde_json_shape::value` module documentation](self) for usage examples.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Value {
    /// Represents a JSON null value.
    Null,

    /// Represents a JSON boolean.
    Bool {
        /// If type is optional
        optional: bool,
    },

    /// Represents a JSON number.
    Number {
        /// If type is optional
        optional: bool,
    },
    /// Represents a JSON string.
    String {
        /// If type is optional
        optional: bool,
    },
    /// Represents a JSON array.
    Array {
        /// Type contained in the Array
        r#type: Box<Value>,
        /// If type is optional
        optional: bool,
    },

    /// Represents a JSON object.
    Object {
        /// Object internal members map, with key as `String` and value as [`JsonShape`]
        content: BTreeMap<String, Value>,
        /// If type is optional
        optional: bool,
    },

    /// Represents a JSON Value that can assume one of the Values described.
    /// Similar to an enum containing diffenrent internal types in Rust.
    OneOf {
        /// All possible [`JsonShape`] values
        variants: BTreeSet<Value>,
        /// If type is optional
        optional: bool,
    },

    /// Represents a JSON Array that behaves like a tuple.
    /// Similar to a Rust tuple, types are always the same and in same order
    Tuple {
        /// [`JsonShape`] order
        elements: Vec<Value>,
        /// If type is optional
        optional: bool,
    },
}

impl Value {
    /// Is this [`JsonShape`] optional? eg, `Option<String>`
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        match self {
            Value::Null => true,
            Value::Bool { optional } => *optional,
            Value::Number { optional } => *optional,
            Value::String { optional } => *optional,
            Value::Array { optional, .. } => *optional,
            Value::Object { optional, .. } => *optional,
            Value::OneOf { optional, .. } => *optional,
            Value::Tuple { optional, .. } => *optional,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn as_optional(self) -> Self {
        match self {
            Value::Null => Value::Null,
            Value::Bool { .. } => Value::Bool { optional: true },
            Value::Number { .. } => Value::Number { optional: true },
            Value::String { .. } => Value::String { optional: true },
            Value::Array { r#type, .. } => Value::Array {
                optional: true,
                r#type,
            },
            Value::Object { content, .. } => Value::Object {
                optional: true,
                content,
            },
            Value::OneOf { variants, .. } => Value::OneOf {
                optional: true,
                variants,
            },
            Value::Tuple { elements, .. } => Value::Tuple {
                optional: true,
                elements,
            },
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn as_non_optional(self) -> Self {
        match self {
            Value::Null => Value::Null,
            Value::Bool { .. } => Value::Bool { optional: false },
            Value::Number { .. } => Value::Number { optional: false },
            Value::String { .. } => Value::String { optional: false },
            Value::Array { r#type, .. } => Value::Array {
                optional: false,
                r#type,
            },
            Value::Object { content, .. } => Value::Object {
                optional: false,
                content,
            },
            Value::OneOf { variants, .. } => Value::OneOf {
                optional: false,
                variants,
            },
            Value::Tuple { elements, .. } => Value::Tuple {
                optional: false,
                elements,
            },
        }
    }

    pub(crate) const fn to_optional_mut(&mut self) {
        match self {
            Value::Null => (),
            Value::Bool { optional } => {
                *optional = true;
            }
            Value::Number { optional } => {
                *optional = true;
            }
            Value::String { optional } => {
                *optional = true;
            }
            Value::Array { optional, .. } => {
                *optional = true;
            }
            Value::Object { optional, .. } => {
                *optional = true;
            }
            Value::OneOf { optional, .. } => {
                *optional = true;
            }
            Value::Tuple { optional, .. } => {
                *optional = true;
            }
        }
    }

    /// Return the keys contained in a [`JsonShape::Object`]
    #[must_use]
    pub fn keys(&self) -> Option<Keys<String, Value>> {
        if let Self::Object { content, .. } = self {
            Some(content.keys())
        } else {
            None
        }
    }

    /// Checks if Json Node is null
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Checks if Json Node is boolean
    #[must_use]
    pub const fn is_boolean(&self) -> bool {
        matches!(self, Self::Bool { .. })
    }

    /// Checks if Json Node is number
    #[must_use]
    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Number { .. })
    }

    /// Checks if Json Node is string
    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String { .. })
    }

    /// Checks if Json Node is array
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array { .. })
    }

    /// Checks if Json Node is tuple
    #[must_use]
    pub const fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple { .. })
    }

    /// Checks if Json Node is object
    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object { .. })
    }

    /// Checks if Json Node is one of
    #[must_use]
    pub const fn is_oneof(&self) -> bool {
        matches!(self, Self::OneOf { .. })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Bool { optional } => write!(
                f,
                "{}",
                if *optional {
                    "Option<Boolean>"
                } else {
                    "Boolean"
                }
            ),
            Value::Number { optional } => write!(
                f,
                "{}",
                if *optional {
                    "Option<Number>"
                } else {
                    "Number"
                }
            ),
            Value::String { optional } => write!(
                f,
                "{}",
                if *optional {
                    "Option<String>"
                } else {
                    "String"
                }
            ),
            Value::Array { r#type, optional } => {
                if *optional {
                    write!(f, "Option<Array<{type}>>")
                } else {
                    write!(f, "Array<{type}>")
                }
            }
            Value::Object { content, optional } => {
                if *optional {
                    write!(f, "Option<Object{{{}}}>", display_object_content(content))
                } else {
                    write!(f, "Object{{{}}}", display_object_content(content))
                }
            }
            Value::OneOf { variants, optional } => {
                let variants = variants
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" | ");
                if *optional {
                    write!(f, "Option<OneOf[{variants}]>",)
                } else {
                    write!(f, "OneOf[{variants}]",)
                }
            }
            Value::Tuple { elements, optional } => {
                let elements = elements
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                if *optional {
                    write!(f, "Option<Tuple({elements})>",)
                } else {
                    write!(f, "Tuple({elements})",)
                }
            }
        }
    }
}

impl Similar for Value {
    fn similar(&self, other: &Self) -> Option<Value> {
        match (self, other) {
            (Value::Null, Value::Null) => Some(Value::Null),
            (Value::Bool { optional }, Value::Bool { optional: opt }) => Some(Value::Bool {
                optional: *optional || *opt,
            }),
            (Value::Number { optional }, Value::Number { optional: opt }) => Some(Value::Number {
                optional: *optional || *opt,
            }),
            (Value::String { optional }, Value::String { optional: opt }) => Some(Value::String {
                optional: *optional || *opt,
            }),
            (
                Value::Array { r#type, optional },
                Value::Array {
                    r#type: ty,
                    optional: opt,
                },
            ) if ty == r#type => Some(Value::Array {
                r#type: ty.clone(),
                optional: *optional || *opt,
            }),
            (
                Value::Object { content, optional },
                Value::Object {
                    content: cont,
                    optional: opt,
                },
            ) if cont == content => Some(Value::Object {
                content: content.clone(),
                optional: *optional || *opt,
            }),
            (
                Value::OneOf { variants, optional },
                Value::OneOf {
                    variants: var,
                    optional: opt,
                },
            ) if var == variants => Some(Value::OneOf {
                variants: variants.clone(),
                optional: *optional || *opt,
            }),
            (
                Value::Tuple { elements, optional },
                Value::Tuple {
                    elements: ty,
                    optional: opt,
                },
            ) if ty == elements => Some(Value::Tuple {
                elements: ty.clone(),
                optional: *optional || *opt,
            }),
            _ => None,
        }
    }
}

fn display_object_content(content: &BTreeMap<String, Value>) -> String {
    content
        .iter()
        .map(|(key, value)| {
            if key
                .chars()
                .all(|char| char.is_alphanumeric() || char == '_' || char == '-')
            {
                format!("{key}: {value}")
            } else {
                format!("\"{key}\": {value}")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_optional_returns_true_when_values_are_optional() {
        assert!(Value::Null.is_optional());
        assert!(Value::Bool { optional: true }.is_optional());
        assert!(Value::Number { optional: true }.is_optional());
        assert!(Value::String { optional: true }.is_optional());
        assert!(
            Value::Array {
                optional: true,
                r#type: Box::new(Value::Null)
            }
            .is_optional()
        );
        assert!(
            Value::Tuple {
                optional: true,
                elements: vec![Value::Null]
            }
            .is_optional()
        );
        assert!(
            Value::Object {
                optional: true,
                content: BTreeMap::default()
            }
            .is_optional()
        );
        assert!(
            Value::OneOf {
                optional: true,
                variants: BTreeSet::default()
            }
            .is_optional()
        );
    }

    #[test]
    fn is_optional_returns_false_when_values_are_not_optional() {
        assert!(!Value::Bool { optional: false }.is_optional());
        assert!(!Value::Number { optional: false }.is_optional());
        assert!(!Value::String { optional: false }.is_optional());
        assert!(
            !Value::Array {
                optional: false,
                r#type: Box::new(Value::Null)
            }
            .is_optional()
        );
        assert!(
            !Value::Tuple {
                optional: false,
                elements: vec![Value::Null]
            }
            .is_optional()
        );
        assert!(
            !Value::Object {
                optional: false,
                content: BTreeMap::default()
            }
            .is_optional()
        );
        assert!(
            !Value::OneOf {
                optional: false,
                variants: BTreeSet::default()
            }
            .is_optional()
        );
    }

    #[test]
    fn as_optional_returns_optional_version_of_values() {
        assert!(Value::Bool { optional: false }.as_optional().is_optional());
        assert!(
            Value::Number { optional: false }
                .as_optional()
                .is_optional()
        );
        assert!(
            Value::String { optional: false }
                .as_optional()
                .is_optional()
        );
        assert!(
            Value::Array {
                optional: false,
                r#type: Box::new(Value::Null)
            }
            .as_optional()
            .is_optional()
        );
        assert!(
            Value::Object {
                optional: false,
                content: BTreeMap::default()
            }
            .as_optional()
            .is_optional()
        );
        assert!(
            Value::OneOf {
                optional: false,
                variants: BTreeSet::default()
            }
            .as_optional()
            .is_optional()
        );
        assert!(
            Value::Tuple {
                optional: false,
                elements: vec![Value::Null]
            }
            .as_optional()
            .is_optional()
        );
    }

    #[test]
    fn keys_returns_keys_only_for_object() {
        assert!(Value::Null.keys().is_none());
        assert!(Value::Bool { optional: true }.keys().is_none());
        assert!(Value::Number { optional: true }.keys().is_none());
        assert!(Value::String { optional: true }.keys().is_none());
        assert!(
            Value::Array {
                optional: true,
                r#type: Box::new(Value::Null)
            }
            .keys()
            .is_none()
        );
        assert!(
            Value::OneOf {
                optional: true,
                variants: BTreeSet::default()
            }
            .keys()
            .is_none()
        );
        assert!(
            Value::Tuple {
                optional: true,
                elements: Vec::default()
            }
            .keys()
            .is_none()
        );
        assert_eq!(
            Value::Object {
                optional: true,
                content: [
                    ("key_1".to_string(), Value::Null),
                    ("key_2".to_string(), Value::Null),
                ]
                .into()
            }
            .keys()
            .unwrap()
            .collect::<Vec<_>>(),
            vec!["key_1", "key_2"]
        );
    }

    #[test]
    fn to_string_for_optional_values() {
        assert_eq!(Value::Null.to_string(), "Null");
        assert_eq!(
            Value::Bool { optional: true }.to_string(),
            "Option<Boolean>"
        );
        assert_eq!(
            Value::Number { optional: true }.to_string(),
            "Option<Number>"
        );
        assert_eq!(
            Value::String { optional: true }.to_string(),
            "Option<String>"
        );
        assert_eq!(
            Value::Array {
                optional: true,
                r#type: Box::new(Value::Null)
            }
            .to_string(),
            "Option<Array<Null>>"
        );
        assert_eq!(
            Value::Object {
                optional: true,
                content: BTreeMap::default()
            }
            .to_string(),
            "Option<Object{}>"
        );
        assert_eq!(
            Value::Object {
                optional: true,
                content: [
                    ("key_1".to_string(), Value::Null),
                    ("key_2".to_string(), Value::Number { optional: true }),
                    ("key_3".to_string(), Value::Number { optional: false })
                ]
                .into()
            }
            .to_string(),
            "Option<Object{key_1: Null, key_2: Option<Number>, key_3: Number}>"
        );
        assert_eq!(
            Value::OneOf {
                optional: true,
                variants: [
                    Value::Null,
                    Value::Number { optional: true },
                    Value::Number { optional: false }
                ]
                .into()
            }
            .to_string(),
            "Option<OneOf[Null | Number | Option<Number>]>"
        );
        assert_eq!(
            Value::Tuple {
                optional: true,
                elements: [
                    Value::Null,
                    Value::Number { optional: true },
                    Value::Number { optional: false }
                ]
                .into()
            }
            .to_string(),
            "Option<Tuple(Null, Option<Number>, Number)>"
        );
    }

    #[test]
    fn to_string_for_non_optional_values() {
        assert_eq!(Value::Bool { optional: false }.to_string(), "Boolean");
        assert_eq!(Value::Number { optional: false }.to_string(), "Number");
        assert_eq!(Value::String { optional: false }.to_string(), "String");
        assert_eq!(
            Value::Array {
                optional: false,
                r#type: Box::new(Value::Null)
            }
            .to_string(),
            "Array<Null>"
        );
        assert_eq!(
            Value::Object {
                optional: false,
                content: BTreeMap::default()
            }
            .to_string(),
            "Object{}"
        );
        assert_eq!(
            Value::Object {
                optional: false,
                content: [
                    ("key_1".to_string(), Value::Null),
                    ("key_2".to_string(), Value::Number { optional: true }),
                    ("key_3".to_string(), Value::Number { optional: false })
                ]
                .into()
            }
            .to_string(),
            "Object{key_1: Null, key_2: Option<Number>, key_3: Number}"
        );
        assert_eq!(
            Value::OneOf {
                optional: false,
                variants: [
                    Value::Null,
                    Value::Number { optional: false },
                    Value::Number { optional: true }
                ]
                .into()
            }
            .to_string(),
            "OneOf[Null | Number | Option<Number>]"
        );
        assert_eq!(
            Value::Tuple {
                optional: false,
                elements: [
                    Value::Null,
                    Value::Number { optional: true },
                    Value::Number { optional: false }
                ]
                .into()
            }
            .to_string(),
            "Tuple(Null, Option<Number>, Number)"
        );
    }

    #[test]
    fn to_optional_mut_transforms_value_inline_as_ref_mut() {
        let mut v = Value::Bool { optional: false };
        assert!(!v.is_optional());
        v.to_optional_mut();
        assert!(v.is_optional());
        let mut v = Value::Number { optional: false };
        assert!(!v.is_optional());
        v.to_optional_mut();
        assert!(v.is_optional());
        let mut v = Value::String { optional: false };
        assert!(!v.is_optional());
        v.to_optional_mut();
        assert!(v.is_optional());
    }

    #[test]
    fn parse_multiple_keys() {
        let map = [
            ("key_value_1".to_string(), Value::Null),
            ("key-value-1".to_string(), Value::Null),
            ("KeyValue1".to_string(), Value::Null),
            ("key value 1".to_string(), Value::Null),
            ("key_value?".to_string(), Value::Null),
            ("key_value!".to_string(), Value::Null),
        ]
        .into();

        let s = display_object_content(&map);

        assert_eq!(
            s,
            "KeyValue1: Null, \"key value 1\": Null, key-value-1: Null, \"key_value!\": Null, \"key_value?\": Null, key_value_1: Null"
        );
    }
}
