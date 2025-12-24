use crate::value::subtypes::IsOneOf;
use crate::{
    IsSubset,
    value::{
        Value,
        subtypes::{Boolean, Number, Optional, String as Str},
    },
};

/// - `JsonShape::Number` is subset of `JsonShape::Option<Number>`
/// - `JsonShape::Null` is subset of `JsonShape::Option<Number>` and  `JsonShape::Null`
/// - `JsonShape::Number` is subset of `JsonShape::OneOf[Number | String]`
/// - `JsonShape::Number` is *NOT* subset of `JsonShape::Array<Number>` => `1.23 != [1.23]`
/// - `JsonShape::Array<Number>` is subset of `JsonShape::Array<OnOf<[Number | Boolean]>>`
/// - `JsonShape::Object{"key_a": JsonShape::Number}` is *NOT* subset of `JsonShape::Object{"key_b": JsonShape::Number}` => `key_a != key_b`
/// - `JsonShape::Object{"key_a": JsonShape::Number}` is subset of `JsonShape::Object{"key_a": JsonShape::Option<Number>}`
/// - `JsonShape::Object{"key_a": JsonShape::Number}` is subset of `JsonShape::Object{"key_a": JsonShape::OneOf[Number | Boolean]}`
impl IsSubset for Value {
    #[allow(clippy::too_many_lines)]
    /// Checks if [`JsonShape`] is subset of `other` [`JsonShape`]
    fn is_subset(&self, other: &Self) -> bool {
        match self {
            Self::Null => other.is_optional() || other.is_null(),
            // Optionals
            Self::Bool { optional: true } => {
                other.is_boolean() && other.is_optional()
                    || IsOneOf::<Optional<Boolean>>::is_one_of(other)
            }
            Self::Number { optional: true } => {
                other.is_number() && other.is_optional()
                    || IsOneOf::<Optional<Number>>::is_one_of(other)
            }
            Self::String { optional: true } => {
                other.is_string() && other.is_optional()
                    || IsOneOf::<Optional<Str>>::is_one_of(other)
            }
            Self::Array {
                r#type,
                optional: true,
            } => match other {
                Self::Array {
                    r#type: ty,
                    optional: true,
                } => r#type.is_subset(ty),
                Self::OneOf { variants, .. } => variants.contains(&Self::Array {
                    r#type: r#type.clone(),
                    optional: true,
                }),
                _ => false,
            },
            Self::Tuple {
                elements,
                optional: true,
            } => match other {
                Self::Tuple {
                    elements: other,
                    optional: true,
                } => {
                    elements.iter().zip(other).all(|(a, b)| a.is_subset(b))
                        && elements.len() == other.len()
                }
                Self::OneOf { variants, .. } => variants.contains(&Self::Tuple {
                    elements: elements.clone(),
                    optional: true,
                }),
                Self::Array { r#type, .. } => {
                    let Self::OneOf { variants, .. } = &&**r#type else {
                        return false;
                    };
                    elements.iter().all(|element| variants.contains(element))
                }
                _ => false,
            },
            Self::Object {
                content,
                optional: true,
            } => match other {
                Self::Object {
                    content: other,
                    optional: true,
                } => {
                    for (k, v) in other {
                        if !content.contains_key(k) && !v.is_optional() {
                            return false;
                        }
                    }
                    content.iter().all(|(key, value)| {
                        other
                            .get(key)
                            .is_some_and(|other_val| value.is_subset(other_val))
                    })
                }
                Self::OneOf { variants, .. } => variants
                    .iter()
                    .filter(|var| matches!(var, Self::Object { .. }))
                    .any(|var| self.is_subset(var)),
                _ => false,
            },
            Self::OneOf {
                variants,
                optional: true,
            } => match other {
                Self::OneOf {
                    variants: var,
                    optional: true,
                } => {
                    variants.is_subset(var)
                        || variants
                            .iter()
                            .all(|variant| var.iter().any(|v| variant.is_subset(v)))
                }
                _ => false,
            },

            // Non-optionals
            Self::Bool { optional: false } => {
                other.is_boolean()
                    || IsOneOf::<Boolean>::is_one_of(other)
                    || IsOneOf::<Optional<Boolean>>::is_one_of(other)
            }
            Self::Number { optional: false } => {
                other.is_number()
                    || IsOneOf::<Number>::is_one_of(other)
                    || IsOneOf::<Optional<Number>>::is_one_of(other)
            }
            Self::String { optional: false } => {
                other.is_string()
                    || IsOneOf::<Str>::is_one_of(other)
                    || IsOneOf::<Optional<Str>>::is_one_of(other)
            }
            Self::Array {
                r#type,
                optional: false,
            } => match other {
                Self::Array { r#type: ty, .. } => r#type.is_subset(ty),
                Self::OneOf { variants, .. } => {
                    variants.contains(&Self::Array {
                        r#type: r#type.clone(),
                        optional: false,
                    }) || variants.contains(&Self::Array {
                        r#type: r#type.clone(),
                        optional: true,
                    })
                }
                _ => false,
            },
            Self::Tuple {
                elements,
                optional: false,
            } => match other {
                Self::Tuple {
                    elements: other, ..
                } => {
                    elements.iter().zip(other).all(|(a, b)| a.is_subset(b))
                        && elements.len() == other.len()
                }
                Self::OneOf { variants, .. } => {
                    variants.contains(&Self::Tuple {
                        elements: elements.clone(),
                        optional: false,
                    }) || variants.contains(&Self::Tuple {
                        elements: elements.clone(),
                        optional: true,
                    })
                }
                Self::Array { r#type, .. } => {
                    let Self::OneOf { variants, .. } = &&**r#type else {
                        return false;
                    };
                    elements.iter().all(|element| variants.contains(element))
                }
                _ => false,
            },
            Self::Object {
                content,
                optional: false,
            } => match other {
                Self::Object { content: other, .. } => {
                    for (k, v) in other {
                        if !content.contains_key(k) && !v.is_optional() {
                            return false;
                        }
                    }
                    content.iter().all(|(key, value)| {
                        other
                            .get(key)
                            .is_some_and(|other_val| value.is_subset(other_val))
                    })
                }
                Self::OneOf { variants, .. } => variants
                    .iter()
                    .filter(|var| matches!(var, Self::Object { .. }))
                    .any(|var| self.is_subset(var)),
                _ => false,
            },
            Self::OneOf {
                variants,
                optional: false,
            } => match other {
                Self::OneOf { variants: var, .. } => {
                    variants.is_subset(var)
                        || variants
                            .iter()
                            .all(|variant| var.iter().any(|v| variant.is_subset(v)))
                }
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod null {
        use super::*;

        #[test]
        fn when_null_is_subset_of_null() {
            assert!(Value::Null.is_subset(&Value::Null));
        }

        #[test]
        fn when_null_is_subset_of_optional() {
            assert!(Value::Null.is_subset(&Value::Number { optional: true }));
        }

        #[test]
        fn when_null_is_not_subset_of_number() {
            assert!(!Value::Null.is_subset(&Value::Number { optional: false }));
        }
    }

    mod number {
        use super::*;

        #[test]
        fn when_number_is_subset_of_number() {
            assert!(
                Value::Number { optional: false }.is_subset(&Value::Number { optional: false })
            );
        }

        #[test]
        fn when_number_is_subset_of_optional_number() {
            assert!(Value::Number { optional: false }.is_subset(&Value::Number { optional: true }));
        }

        #[test]
        fn when_optional_number_is_subset_of_optional_number() {
            assert!(Value::Number { optional: true }.is_subset(&Value::Number { optional: true }));
        }

        #[test]
        fn when_optional_number_is_not_subset_of_number() {
            assert!(
                !Value::Number { optional: true }.is_subset(&Value::Number { optional: false })
            );
        }

        #[test]
        fn when_number_is_not_subset_of_string() {
            assert!(
                !Value::Number { optional: false }.is_subset(&Value::String { optional: false })
            );
        }

        #[test]
        fn when_number_is_subset_of_oneof_with_number_variant() {
            assert!(Value::Number { optional: false }.is_subset(&Value::OneOf {
                variants: [Value::Number { optional: false }, Value::Null].into(),
                optional: false
            }));
        }

        #[test]
        fn when_number_is_subset_of_oneof_with_optional_number_variant() {
            assert!(Value::Number { optional: false }.is_subset(&Value::OneOf {
                variants: [Value::Number { optional: true }, Value::Null].into(),
                optional: false
            }));
        }
    }

    mod string {
        use super::*;

        #[test]
        fn when_string_is_subset_of_string() {
            assert!(
                Value::String { optional: false }.is_subset(&Value::String { optional: false })
            );
        }

        #[test]
        fn when_string_is_subset_of_optional_string() {
            assert!(Value::String { optional: false }.is_subset(&Value::String { optional: true }));
        }

        #[test]
        fn when_optional_string_is_subset_of_optional_string() {
            assert!(Value::String { optional: true }.is_subset(&Value::String { optional: true }));
        }

        #[test]
        fn when_optional_string_is_not_subset_of_string() {
            assert!(
                !Value::String { optional: true }.is_subset(&Value::String { optional: false })
            );
        }

        #[test]
        fn when_string_is_not_subset_of_string() {
            assert!(
                !Value::String { optional: false }.is_subset(&Value::Number { optional: false })
            );
        }

        #[test]
        fn when_string_is_subset_of_oneof_with_string_variant() {
            assert!(Value::String { optional: false }.is_subset(&Value::OneOf {
                variants: [Value::String { optional: false }, Value::Null].into(),
                optional: false
            }));
        }

        #[test]
        fn when_string_is_subset_of_oneof_with_optional_string_variant() {
            assert!(Value::String { optional: false }.is_subset(&Value::OneOf {
                variants: [Value::String { optional: true }, Value::Null].into(),
                optional: false
            }));
        }
    }

    mod bool {
        use super::*;

        #[test]
        fn when_bool_is_subset_of_bool() {
            assert!(Value::Bool { optional: false }.is_subset(&Value::Bool { optional: false }));
        }

        #[test]
        fn when_bool_is_subset_of_optional_bool() {
            assert!(Value::Bool { optional: false }.is_subset(&Value::Bool { optional: true }));
        }

        #[test]
        fn when_optional_bool_is_subset_of_optional_bool() {
            assert!(Value::Bool { optional: true }.is_subset(&Value::Bool { optional: true }));
        }

        #[test]
        fn when_optional_bool_is_not_subset_of_bool() {
            assert!(!Value::Bool { optional: true }.is_subset(&Value::Bool { optional: false }));
        }

        #[test]
        fn when_bool_is_not_subset_of_string() {
            assert!(!Value::Bool { optional: false }.is_subset(&Value::String { optional: false }));
        }

        #[test]
        fn when_bool_is_subset_of_oneof_with_bool_variant() {
            assert!(Value::Bool { optional: false }.is_subset(&Value::OneOf {
                variants: [Value::Bool { optional: false }, Value::Null].into(),
                optional: false
            }));
        }

        #[test]
        fn when_bool_is_subset_of_oneof_with_optional_bool_variant() {
            assert!(Value::Bool { optional: false }.is_subset(&Value::OneOf {
                variants: [Value::Bool { optional: true }, Value::Null].into(),
                optional: false
            }));
        }
    }

    mod array {
        use super::*;

        #[test]
        fn when_array_number_is_subset_of_array_number() {
            assert!(
                Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: false
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: false
                })
            );
        }

        #[test]
        fn when_array_number_is_subset_of_optional_array_number() {
            assert!(
                Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: false
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: true
                })
            );
        }

        #[test]
        fn when_array_number_is_subset_of_array_optional_number() {
            assert!(
                Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: false
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: false
                })
            );
        }

        #[test]
        fn when_array_number_is_subset_of_optional_array_optional_number() {
            assert!(
                Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: false
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: true
                })
            );
        }

        #[test]
        fn when_array_optional_number_is_subset_of_array_number() {
            assert!(
                !Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: false
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: false
                })
            );
        }

        #[test]
        fn when_array_optional_number_is_subset_of_optional_array_number() {
            assert!(
                !Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: false
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: true
                })
            );
        }

        #[test]
        fn when_array_optional_number_is_subset_of_array_optional_number() {
            assert!(
                Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: false
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: false
                })
            );
        }

        #[test]
        fn when_array_optional_number_is_subset_of_optional_array_optional_number() {
            assert!(
                Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: false
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: true
                })
            );
        }

        #[test]
        fn when_optional_array_number_is_subset_of_array_number() {
            assert!(
                !Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: true
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: false
                })
            );
        }

        #[test]
        fn when_optional_array_number_is_subset_of_optional_array_number() {
            assert!(
                Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: true
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: true
                })
            );
        }

        #[test]
        fn when_optional_array_number_is_subset_of_array_optional_number() {
            assert!(
                !Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: true
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: false
                })
            );
        }

        #[test]
        fn when_optional_array_number_is_subset_of_optional_array_optional_number() {
            assert!(
                Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: true
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: true
                })
            );
        }

        #[test]
        fn when_optional_array_optional_number_is_subset_of_array_number() {
            assert!(
                !Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: true
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: false
                })
            );
        }

        #[test]
        fn when_optional_array_optional_number_is_subset_of_optional_array_number() {
            assert!(
                !Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: true
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: true
                })
            );
        }

        #[test]
        fn when_optional_array_optional_number_is_subset_of_array_optional_number() {
            assert!(
                !Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: true
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: false
                })
            );
        }

        #[test]
        fn when_optional_array_optional_number_is_subset_of_optional_array_optional_number() {
            assert!(
                Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: true
                }
                .is_subset(&Value::Array {
                    r#type: Box::new(Value::Number { optional: true }),
                    optional: true
                })
            );
        }
    }

    mod oneof {
        use super::*;

        #[test]
        fn when_oneof_is_subset_of_equal_oneof() {
            assert!(
                Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: false
                }
                .is_subset(&Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_oneof_is_subset_of_equal_optional_oneof() {
            assert!(
                Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: false
                }
                .is_subset(&Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: true
                })
            );
        }

        #[test]
        fn when_oneof_is_subset_of_larger_oneof() {
            assert!(
                Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: false
                }
                .is_subset(&Value::OneOf {
                    variants: [
                        Value::Number { optional: false },
                        Value::String { optional: false }
                    ]
                    .into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_oneof_is_subset_of_larger_oneof_with_optional_superset() {
            assert!(
                Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: false
                }
                .is_subset(&Value::OneOf {
                    variants: [
                        Value::Number { optional: true },
                        Value::String { optional: false }
                    ]
                    .into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_oneof_is_subset_of_diff_oneof() {
            assert!(
                !Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: false
                }
                .is_subset(&Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::String { optional: false }
                    ]
                    .into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_optional_oneof_is_subset_of_equal_oneof() {
            assert!(
                Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: true
                }
                .is_subset(&Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: true
                })
            );
        }

        #[test]
        fn when_optional_oneof_is_not_subset_of_non_optional_oneof() {
            assert!(
                !Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: true
                }
                .is_subset(&Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_optional_oneof_is_subset_of_equal_optional_oneof() {
            assert!(
                Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: true
                }
                .is_subset(&Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: true
                })
            );
        }

        #[test]
        fn when_optional_oneof_is_subset_of_larger_oneof() {
            assert!(
                Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: true
                }
                .is_subset(&Value::OneOf {
                    variants: [
                        Value::Number { optional: false },
                        Value::String { optional: false }
                    ]
                    .into(),
                    optional: true
                })
            );
        }

        #[test]
        fn when_optional_oneof_is_subset_of_larger_oneof_with_optional_superset() {
            assert!(
                Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: true
                }
                .is_subset(&Value::OneOf {
                    variants: [
                        Value::Number { optional: true },
                        Value::String { optional: false }
                    ]
                    .into(),
                    optional: true
                })
            );
        }

        #[test]
        fn when_optional_oneof_is_not_subset_of_diff_oneof() {
            assert!(
                !Value::OneOf {
                    variants: [Value::Number { optional: false }].into(),
                    optional: true
                }
                .is_subset(&Value::OneOf {
                    variants: [
                        Value::Bool { optional: false },
                        Value::String { optional: false }
                    ]
                    .into(),
                    optional: false
                })
            );
        }
    }

    mod object {
        use super::*;

        #[test]
        fn when_empty_obj_is_subset_of_empty_obj() {
            assert!(
                Value::Object {
                    content: [].into(),
                    optional: false
                }
                .is_subset(&Value::Object {
                    content: [].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_optional_empty_obj_is_subset_of_empty_obj() {
            assert!(
                !Value::Object {
                    content: [].into(),
                    optional: true
                }
                .is_subset(&Value::Object {
                    content: [].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_empty_obj_is_subset_of_obj_with_null() {
            assert!(
                Value::Object {
                    content: [].into(),
                    optional: false
                }
                .is_subset(&Value::Object {
                    content: [("key".to_string(), Value::Null)].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_empty_obj_is_subset_of_obj_with_optional() {
            assert!(
                Value::Object {
                    content: [].into(),
                    optional: false
                }
                .is_subset(&Value::Object {
                    content: [("key".to_string(), Value::Number { optional: true })].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_empty_obj_is_not_subset_of_obj_with_value() {
            assert!(
                !Value::Object {
                    content: [].into(),
                    optional: false
                }
                .is_subset(&Value::Object {
                    content: [("key".to_string(), Value::Number { optional: false })].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_obj_is_subset_of_obj_with_same_optional() {
            assert!(
                Value::Object {
                    content: [("key".to_string(), Value::Number { optional: false })].into(),
                    optional: false
                }
                .is_subset(&Value::Object {
                    content: [("key".to_string(), Value::Number { optional: true })].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_obj_of_optionalis_subset_of_obj_with_same() {
            assert!(
                Value::Object {
                    content: [("key".to_string(), Value::Number { optional: true })].into(),
                    optional: false
                }
                .is_subset(&Value::Object {
                    content: [("key".to_string(), Value::Number { optional: true })].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_obj_is_not_subset_of_obj_with_different_same_key() {
            assert!(
                !Value::Object {
                    content: [("key".to_string(), Value::Number { optional: false })].into(),
                    optional: false
                }
                .is_subset(&Value::Object {
                    content: [("key".to_string(), Value::Bool { optional: true })].into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_obj_is_subset_of_obj_with_same_key_superset() {
            assert!(
                Value::Object {
                    content: [("key".to_string(), Value::Number { optional: false })].into(),
                    optional: false
                }
                .is_subset(&Value::Object {
                    content: [(
                        "key".to_string(),
                        Value::OneOf {
                            variants: [
                                Value::Number { optional: false },
                                Value::Bool { optional: false },
                                Value::Null,
                            ]
                            .into(),
                            optional: false
                        }
                    )]
                    .into(),
                    optional: false
                })
            );
        }

        #[test]
        fn when_large_obj_is_subset_of_larger_obj() {
            assert!(
                Value::Object {
                    content: [
                        ("key".to_string(), Value::Number { optional: false }),
                        ("b".to_string(), Value::Bool { optional: false }),
                        ("s".to_string(), Value::String { optional: true }),
                    ]
                    .into(),
                    optional: true
                }
                .is_subset(&Value::Object {
                    content: [
                        ("key".to_string(), Value::Number { optional: true }),
                        ("b".to_string(), Value::Bool { optional: false }),
                        ("s".to_string(), Value::String { optional: true }),
                        ("z".to_string(), Value::String { optional: true }),
                        ("n".to_string(), Value::Number { optional: true }),
                    ]
                    .into(),
                    optional: true
                })
            );
        }
    }

    #[test]
    fn object_as_subset_of_oneof() {
        let shape = Value::OneOf {
            variants: [
                Value::Object {
                    content: [
                        ("number".to_string(), Value::Number { optional: false }),
                        ("state".to_string(), Value::String { optional: false }),
                    ]
                    .into(),
                    optional: false,
                },
                Value::Array {
                    r#type: Box::new(Value::Number { optional: false }),
                    optional: false,
                },
            ]
            .into(),
            optional: false,
        };

        let value = Value::Object {
            content: [
                ("number".to_string(), Value::Number { optional: false }),
                ("state".to_string(), Value::String { optional: false }),
            ]
            .into(),
            optional: false,
        };

        assert!(value.is_subset(&shape));
    }
}

#[cfg(test)]
mod ai_tests {
    use super::*;
    #[test]
    fn null_is_subset_of_null() {
        assert!(Value::Null.is_subset(&Value::Null));
    }

    #[test]
    fn null_is_not_subset_of_number() {
        assert!(!Value::Null.is_subset(&Value::Number { optional: false }));
    }

    #[test]
    fn null_is_not_subset_of_string() {
        assert!(!Value::Null.is_subset(&Value::String { optional: false }));
    }

    #[test]
    fn number_is_subset_of_number() {
        assert!(Value::Number { optional: false }.is_subset(&Value::Number { optional: false }));
    }

    #[test]
    fn number_is_not_subset_of_string() {
        assert!(!Value::Number { optional: false }.is_subset(&Value::String { optional: false }));
    }

    #[test]
    fn number_is_not_subset_of_null() {
        assert!(!Value::Number { optional: false }.is_subset(&Value::Null));
    }

    #[test]
    fn string_is_subset_of_string() {
        assert!(Value::String { optional: false }.is_subset(&Value::String { optional: false }));
    }

    #[test]
    fn string_is_not_subset_of_number() {
        assert!(!Value::String { optional: false }.is_subset(&Value::Number { optional: false }));
    }

    #[test]
    fn string_is_not_subset_of_null() {
        assert!(!Value::String { optional: false }.is_subset(&Value::Null));
    }

    #[test]
    fn boolean_is_subset_of_boolean() {
        assert!(Value::Bool { optional: false }.is_subset(&Value::Bool { optional: false }));
    }

    #[test]
    fn boolean_is_not_subset_of_number() {
        assert!(!Value::Bool { optional: false }.is_subset(&Value::Number { optional: false }));
    }

    #[test]
    fn boolean_is_not_subset_of_string() {
        assert!(!Value::Bool { optional: false }.is_subset(&Value::String { optional: false }));
    }
}
