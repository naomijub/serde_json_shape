#![allow(clippy::match_same_arms)]
use std::{
    collections::{BTreeMap, BTreeSet, btree_map::Keys},
    fmt::Display,
};

use serde::{Deserialize, Serialize};

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

    /// Represents a JSON Value that can assume one of the Values described
    OneOf {
        /// All possible [`JsonShape`] values
        variants: BTreeSet<Value>,
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
                if *optional {
                    write!(
                        f,
                        "Option<OneOf[{}]>",
                        variants
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(" | ")
                    )
                } else {
                    write!(
                        f,
                        "OneOf[{}]",
                        variants
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(" | ")
                    )
                }
            }
        }
    }
}

fn display_object_content(content: &BTreeMap<String, Value>) -> String {
    content
        .iter()
        .map(|(key, value)| format!("\"{key}\": {value}"))
        .collect::<Vec<_>>()
        .join(", ")
}
