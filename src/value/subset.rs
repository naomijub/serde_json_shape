use crate::value::subtypes::{IsArrayOf, IsObjectOf, IsOneOf};
use crate::{
    IsSubset,
    value::{
        Value,
        subtypes::{Array, Boolean, Number, Object, OneOf, Optional, String as Str},
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
    fn is_subset(&self, other: &Self) -> bool {
        match self {
            Value::Null => other.is_optional() || other.is_null(),
            // Optionals
            Value::Bool { optional: true } => {
                other.is_boolean() && other.is_optional()
                    || IsOneOf::<Optional<Boolean>>::is_one_of(other)
            }
            Value::Number { optional: true } => {
                other.is_number() && other.is_optional()
                    || IsOneOf::<Optional<Number>>::is_one_of(other)
            }
            Value::String { optional: true } => {
                other.is_string() && other.is_optional()
                    || IsOneOf::<Optional<Str>>::is_one_of(other)
            }
            Value::Array {
                r#type,
                optional: true,
            } => todo!(),
            Value::Object {
                content,
                optional: true,
            } => todo!(),
            Value::OneOf {
                variants,
                optional: true,
            } => todo!(),
            // Non-optionals
            Value::Bool { optional: false } => {
                other.is_boolean() || IsOneOf::<Boolean>::is_one_of(other)
            }
            Value::Number { optional: false } => {
                other.is_number() || IsOneOf::<Number>::is_one_of(other)
            }
            Value::String { optional: false } => {
                other.is_string() || IsOneOf::<Str>::is_one_of(other)
            }
            Value::Array {
                r#type,
                optional: false,
            } => todo!(),
            Value::Object {
                content,
                optional: false,
            } => todo!(),
            Value::OneOf {
                variants,
                optional: false,
            } => todo!(),
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
}
