use crate::value::Value;

/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `Null`.
pub struct Null;
/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `Number`.
pub struct Number;
/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `Boolean`.
pub struct Boolean;
/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `String`.
pub struct String;
/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `Array`.
pub struct Array;
/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `Tuple`.
pub struct Tuple;
/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `Object`.
pub struct Object;
/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `OneOf`.
pub struct OneOf;

/// Simple helper struct to determine if `JsonShape` is of specific optional subtype.
pub struct Optional<U>(std::marker::PhantomData<U>);

mod private {
    use crate::value::Value;

    pub trait Sealed {}
    impl Sealed for Value {}
}

impl Value {
    #[must_use]
    /// Cheecks if [`JsonShape::Tuple`] is tuple containing `&[JsonShape]` in the same order and type.
    pub fn is_tuple_of(&self, types: &[Value]) -> bool {
        if let Value::Tuple { elements, .. } = self {
            elements.len() == types.len() && elements.iter().zip(types.iter()).all(|(a, b)| a == b)
        } else {
            false
        }
    }
}

/// Checks if [`JsonShape`] is an Array of `T`
pub trait IsArrayOf<T>: private::Sealed {
    /// Checks if [`JsonShape`] is an Array of `T`
    /// - `value.is_array_of::<Null>()`.
    #[allow(dead_code)]
    fn is_array_of(&self) -> bool;
}

/// Checks if [`JsonShape`] is a Tuple of `T` at position `i`
pub trait IsTupleOf<T>: private::Sealed {
    /// Checks if [`JsonShape`] is an Tuple of `T` at position `i`
    /// - `value.is_tuple_of::<Null>(2)`.
    #[allow(dead_code)]
    fn is_tuple_of(&self, i: usize) -> bool;
}

/// Checks if [`JsonShape`] is `OneOf` containing `T`
pub trait IsOneOf<T>: private::Sealed {
    /// Checks if [`JsonShape`] is `OneOf` containing `T`
    /// - `value.is_one_of::<Null>()`.
    fn is_one_of(&self) -> bool;
}

/// Checks if [`JsonShape`] is `Object` containing `key: &str` and  `value: T`
pub trait IsObjectOf<T>: private::Sealed {
    /// Checks if [`JsonShape`] is `Object` containing `key: &str` and  `value: T`
    /// - `value.is_object_of::<Null>("key_1")`.
    #[allow(dead_code)]
    fn is_object_of(&self, key: &str) -> bool;
}

// ARRAY
impl IsArrayOf<Null> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            **r#type == Value::Null
        } else {
            false
        }
    }
}

impl IsArrayOf<Number> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::Number { optional: false })
        } else {
            false
        }
    }
}

impl IsArrayOf<Optional<Number>> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::Number { optional: true })
        } else {
            false
        }
    }
}

impl IsArrayOf<Tuple> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(
                **r#type,
                Value::Tuple {
                    optional: false,
                    ..
                }
            )
        } else {
            false
        }
    }
}

impl IsArrayOf<Optional<Tuple>> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::Tuple { optional: true, .. })
        } else {
            false
        }
    }
}

impl IsArrayOf<String> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::String { optional: false })
        } else {
            false
        }
    }
}

impl IsArrayOf<Optional<String>> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::String { optional: true })
        } else {
            false
        }
    }
}

impl IsArrayOf<Boolean> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::Bool { optional: false })
        } else {
            false
        }
    }
}

impl IsArrayOf<Optional<Boolean>> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::Bool { optional: true })
        } else {
            false
        }
    }
}

impl IsArrayOf<Array> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(
                **r#type,
                Value::Array {
                    optional: false,
                    ..
                }
            )
        } else {
            false
        }
    }
}

impl IsArrayOf<Optional<Array>> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::Array { optional: true, .. })
        } else {
            false
        }
    }
}

impl IsArrayOf<Object> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(
                **r#type,
                Value::Object {
                    optional: false,
                    ..
                }
            )
        } else {
            false
        }
    }
}

impl IsArrayOf<Optional<Object>> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::Object { optional: true, .. })
        } else {
            false
        }
    }
}

impl IsArrayOf<OneOf> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(
                **r#type,
                Value::OneOf {
                    optional: false,
                    ..
                }
            )
        } else {
            false
        }
    }
}

impl IsArrayOf<Optional<OneOf>> for Value {
    fn is_array_of(&self) -> bool {
        if let Value::Array { r#type, .. } = self {
            matches!(**r#type, Value::OneOf { optional: true, .. })
        } else {
            false
        }
    }
}

// OneOf
impl IsOneOf<Null> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants.contains(&Value::Null)
        } else {
            false
        }
    }
}

impl IsOneOf<Number> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::Number { optional: false }))
        } else {
            false
        }
    }
}

impl IsOneOf<String> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::String { optional: false }))
        } else {
            false
        }
    }
}

impl IsOneOf<Boolean> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::Bool { optional: false }))
        } else {
            false
        }
    }
}

impl IsOneOf<Array> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants.iter().any(|variant| {
                matches!(
                    &variant,
                    &Value::Array {
                        optional: false,
                        ..
                    }
                )
            })
        } else {
            false
        }
    }
}

impl IsOneOf<Object> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants.iter().any(|variant| {
                matches!(
                    &variant,
                    &Value::Object {
                        optional: false,
                        ..
                    }
                )
            })
        } else {
            false
        }
    }
}

impl IsOneOf<OneOf> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants.iter().any(|variant| {
                matches!(
                    &variant,
                    &Value::OneOf {
                        optional: false,
                        ..
                    }
                )
            })
        } else {
            false
        }
    }
}

impl IsOneOf<Tuple> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants.iter().any(|variant| {
                matches!(
                    &variant,
                    &Value::Tuple {
                        optional: false,
                        ..
                    }
                )
            })
        } else {
            false
        }
    }
}

impl IsOneOf<Optional<Tuple>> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::Tuple { optional: true, .. }))
        } else {
            false
        }
    }
}

impl IsOneOf<Optional<Number>> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::Number { .. }))
                && variants.contains(&Value::Null)
        } else {
            false
        }
    }
}

impl IsOneOf<Optional<String>> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::String { .. }))
                && variants.contains(&Value::Null)
        } else {
            false
        }
    }
}

impl IsOneOf<Optional<Boolean>> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::Bool { .. }))
                && variants.contains(&Value::Null)
        } else {
            false
        }
    }
}

impl IsOneOf<Optional<Array>> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::Array { .. }))
                && variants.contains(&Value::Null)
        } else {
            false
        }
    }
}

impl IsOneOf<Optional<Object>> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::Object { .. }))
                && variants.contains(&Value::Null)
        } else {
            false
        }
    }
}

impl IsOneOf<Optional<OneOf>> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, optional } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::OneOf { optional: true, .. }))
                || *optional
        } else {
            false
        }
    }
}

// Object
impl IsObjectOf<Null> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content
                .iter()
                .any(|(k, value)| k == key && matches!(&value, &Value::Null))
        } else {
            false
        }
    }
}

impl IsObjectOf<Number> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content
                .iter()
                .any(|(k, value)| k == key && matches!(&value, &Value::Number { optional: false }))
        } else {
            false
        }
    }
}

impl IsObjectOf<String> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content
                .iter()
                .any(|(k, value)| k == key && matches!(&value, &Value::String { optional: false }))
        } else {
            false
        }
    }
}

impl IsObjectOf<Boolean> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content
                .iter()
                .any(|(k, value)| k == key && matches!(&value, &Value::Bool { optional: false }))
        } else {
            false
        }
    }
}

impl IsObjectOf<Array> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content.iter().any(|(k, value)| {
                k == key
                    && matches!(
                        &value,
                        &Value::Array {
                            optional: false,
                            ..
                        }
                    )
            })
        } else {
            false
        }
    }
}

impl IsObjectOf<Tuple> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content.iter().any(|(k, value)| {
                k == key
                    && matches!(
                        &value,
                        &Value::Tuple {
                            optional: false,
                            ..
                        }
                    )
            })
        } else {
            false
        }
    }
}

impl IsObjectOf<Object> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content.iter().any(|(k, value)| {
                k == key
                    && matches!(
                        &value,
                        &Value::Object {
                            optional: false,
                            ..
                        }
                    )
            })
        } else {
            false
        }
    }
}

impl IsObjectOf<OneOf> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content.iter().any(|(k, value)| {
                k == key
                    && matches!(
                        &value,
                        &Value::OneOf {
                            optional: false,
                            ..
                        }
                    )
            })
        } else {
            false
        }
    }
}

impl IsObjectOf<Optional<Number>> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content
                .iter()
                .any(|(k, value)| k == key && matches!(&value, &Value::Number { optional: true }))
        } else {
            false
        }
    }
}

impl IsObjectOf<Optional<String>> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content
                .iter()
                .any(|(k, value)| k == key && matches!(&value, &Value::String { optional: true }))
        } else {
            false
        }
    }
}

impl IsObjectOf<Optional<Boolean>> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content
                .iter()
                .any(|(k, value)| k == key && matches!(&value, &Value::Bool { optional: true }))
        } else {
            false
        }
    }
}

impl IsObjectOf<Optional<Array>> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content.iter().any(|(k, value)| {
                k == key && matches!(&value, &Value::Array { optional: true, .. })
            })
        } else {
            false
        }
    }
}

impl IsObjectOf<Optional<Tuple>> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content.iter().any(|(k, value)| {
                k == key && matches!(&value, &Value::Tuple { optional: true, .. })
            })
        } else {
            false
        }
    }
}

impl IsObjectOf<Optional<Object>> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content.iter().any(|(k, value)| {
                k == key && matches!(&value, &Value::Object { optional: true, .. })
            })
        } else {
            false
        }
    }
}

impl IsObjectOf<Optional<OneOf>> for Value {
    fn is_object_of(&self, key: &str) -> bool {
        if let Value::Object { content, .. } = self {
            content.iter().any(|(k, value)| {
                k == key && matches!(&value, &Value::OneOf { optional: true, .. })
            })
        } else {
            false
        }
    }
}

// Tuple
impl IsTupleOf<Null> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            elements.get(i).is_some_and(|v| v == &Value::Null)
        } else {
            false
        }
    }
}

impl IsTupleOf<Number> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(elements.get(i), Some(Value::Number { optional: false }))
        } else {
            false
        }
    }
}

impl IsTupleOf<Optional<Number>> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(elements.get(i), Some(Value::Number { optional: true }))
        } else {
            false
        }
    }
}

impl IsTupleOf<String> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(elements.get(i), Some(Value::String { optional: false }))
        } else {
            false
        }
    }
}

impl IsTupleOf<Optional<String>> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(elements.get(i), Some(Value::String { optional: true }))
        } else {
            false
        }
    }
}

impl IsTupleOf<Boolean> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(elements.get(i), Some(Value::Bool { optional: false }))
        } else {
            false
        }
    }
}

impl IsTupleOf<Optional<Boolean>> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(elements.get(i), Some(Value::Bool { optional: true }))
        } else {
            false
        }
    }
}

impl IsTupleOf<Array> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(
                elements.get(i),
                Some(Value::Array {
                    optional: false,
                    ..
                })
            )
        } else {
            false
        }
    }
}

impl IsTupleOf<Optional<Array>> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(elements.get(i), Some(Value::Array { optional: true, .. }))
        } else {
            false
        }
    }
}

impl IsTupleOf<Object> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(
                elements.get(i),
                Some(Value::Object {
                    optional: false,
                    ..
                })
            )
        } else {
            false
        }
    }
}

impl IsTupleOf<Optional<Object>> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(elements.get(i), Some(Value::Object { optional: true, .. }))
        } else {
            false
        }
    }
}

impl IsTupleOf<OneOf> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(
                elements.get(i),
                Some(Value::OneOf {
                    optional: false,
                    ..
                })
            )
        } else {
            false
        }
    }
}

impl IsTupleOf<Optional<OneOf>> for Value {
    fn is_tuple_of(&self, i: usize) -> bool {
        if let Value::Tuple { elements, .. } = self {
            matches!(elements.get(i), Some(Value::OneOf { optional: true, .. }))
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_array_of_null() {
        assert!(IsArrayOf::<Null>::is_array_of(&Value::Array {
            r#type: Box::new(Value::Null),
            optional: false
        }));
        assert!(!IsArrayOf::<Null>::is_array_of(&Value::Array {
            r#type: Box::new(Value::Number { optional: true }),
            optional: false
        }));
    }

    #[test]
    fn is_array_of_number() {
        assert!(IsArrayOf::<Number>::is_array_of(&Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false
        }));
        assert!(!IsArrayOf::<Number>::is_array_of(&Value::Array {
            r#type: Box::new(Value::Bool { optional: true }),
            optional: false
        }));
    }

    #[test]
    fn is_array_of_string() {
        assert!(IsArrayOf::<String>::is_array_of(&Value::Array {
            r#type: Box::new(Value::String { optional: false }),
            optional: false
        }));
        assert!(!IsArrayOf::<String>::is_array_of(&Value::Array {
            r#type: Box::new(Value::Bool { optional: true }),
            optional: false
        }));
    }

    #[test]
    fn is_array_of_bool() {
        assert!(IsArrayOf::<Boolean>::is_array_of(&Value::Array {
            r#type: Box::new(Value::Bool { optional: false }),
            optional: false
        }));
        assert!(!IsArrayOf::<Boolean>::is_array_of(&Value::Array {
            r#type: Box::new(Value::String { optional: true }),
            optional: false
        }));
    }

    #[test]
    fn is_oneof_of_number() {
        assert!(IsOneOf::<Optional<Number>>::is_one_of(&Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::Bool { optional: false },
                Value::String { optional: false },
                Value::Null
            ]
            .into(),
            optional: false
        }));
        assert!(IsOneOf::<Number>::is_one_of(&Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::Bool { optional: false },
                Value::String { optional: false }
            ]
            .into(),
            optional: false
        }));
    }

    #[test]
    fn is_oneof_of_bool() {
        assert!(IsOneOf::<Optional<Boolean>>::is_one_of(&Value::OneOf {
            variants: [
                Value::Number { optional: true },
                Value::Bool { optional: true },
                Value::String { optional: true },
                Value::Null
            ]
            .into(),
            optional: false
        }));
        assert!(IsOneOf::<Boolean>::is_one_of(&Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::Bool { optional: false },
                Value::String { optional: false }
            ]
            .into(),
            optional: false
        }));
    }

    #[test]
    fn is_oneof_of_string() {
        assert!(IsOneOf::<Optional<String>>::is_one_of(&Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::Bool { optional: false },
                Value::String { optional: false },
                Value::Null
            ]
            .into(),
            optional: false
        }));
        assert!(IsOneOf::<String>::is_one_of(&Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::Bool { optional: false },
                Value::String { optional: false }
            ]
            .into(),
            optional: false
        }));
    }

    #[test]
    fn is_object_of_number() {
        assert!(IsObjectOf::<Number>::is_object_of(
            &Value::Object {
                content: [("key".to_string(), Value::Number { optional: false })].into(),
                optional: false
            },
            "key"
        ));

        assert!(IsObjectOf::<Optional<Number>>::is_object_of(
            &Value::Object {
                content: [("key".to_string(), Value::Number { optional: true })].into(),
                optional: false
            },
            "key"
        ));
    }

    #[test]
    fn is_object_of_string() {
        assert!(IsObjectOf::<String>::is_object_of(
            &Value::Object {
                content: [("key".to_string(), Value::String { optional: false })].into(),
                optional: false
            },
            "key"
        ));

        assert!(IsObjectOf::<Optional<String>>::is_object_of(
            &Value::Object {
                content: [("key".to_string(), Value::String { optional: true })].into(),
                optional: false
            },
            "key"
        ));
    }

    #[test]
    fn is_object_of_bool() {
        assert!(IsObjectOf::<Boolean>::is_object_of(
            &Value::Object {
                content: [("key".to_string(), Value::Bool { optional: false })].into(),
                optional: false
            },
            "key"
        ));

        assert!(IsObjectOf::<Optional<Boolean>>::is_object_of(
            &Value::Object {
                content: [("key".to_string(), Value::Bool { optional: true })].into(),
                optional: false
            },
            "key"
        ));
    }
}

#[cfg(test)]
mod ai_tests {
    use super::*;

    #[test]
    fn value_is_oneof_number() {
        let value = Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::String { optional: false },
            ]
            .into(),
            optional: false,
        };
        assert!(IsOneOf::<Number>::is_one_of(&value));
    }

    #[test]
    fn value_is_not_oneof_string() {
        let value = Value::OneOf {
            variants: [
                Value::Number { optional: false },
                Value::Bool { optional: false },
            ]
            .into(),
            optional: false,
        };
        assert!(!IsOneOf::<String>::is_one_of(&value));
    }

    #[test]
    fn value_is_oneof_optional_number() {
        let value = Value::OneOf {
            variants: [Value::Null, Value::Number { optional: false }].into(),
            optional: false,
        };
        assert!(IsOneOf::<Optional<Number>>::is_one_of(&value));
    }

    #[test]
    fn value_is_not_oneof_optional_string() {
        let value = Value::OneOf {
            variants: [
                Value::Bool { optional: false },
                Value::Number { optional: false },
                Value::Null,
            ]
            .into(),
            optional: false,
        };
        assert!(!IsOneOf::<Optional<String>>::is_one_of(&value));
    }

    #[test]
    fn value_is_not_oneof_optional_oneof() {
        let value = Value::OneOf {
            variants: [
                Value::Bool { optional: false },
                Value::Number { optional: false },
                Value::Null,
            ]
            .into(),
            optional: false,
        };
        assert!(!IsOneOf::<Optional<OneOf>>::is_one_of(&value));
    }

    #[test]
    fn value_is_oneof_optional_oneof() {
        let value = Value::OneOf {
            variants: [
                Value::Bool { optional: false },
                Value::Number { optional: false },
                Value::Null,
            ]
            .into(),
            optional: true,
        };
        assert!(IsOneOf::<Optional<OneOf>>::is_one_of(&value));
    }

    #[test]
    fn value_is_arrayof_number() {
        let value = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };
        assert!(IsArrayOf::<Number>::is_array_of(&value));
    }

    #[test]
    fn value_is_not_arrayof_string() {
        let value = Value::Array {
            r#type: Box::new(Value::Number { optional: false }),
            optional: false,
        };
        assert!(!IsArrayOf::<String>::is_array_of(&value));
    }

    #[test]
    fn value_is_arrayof_optional_number() {
        let value = Value::Array {
            r#type: Box::new(Value::Number { optional: true }),
            optional: false,
        };
        assert!(IsArrayOf::<Optional<Number>>::is_array_of(&value));
    }

    #[test]
    fn value_is_not_arrayof_optional_string() {
        let value = Value::Array {
            r#type: Box::new(Value::Number { optional: true }),
            optional: false,
        };
        assert!(!IsArrayOf::<Optional<String>>::is_array_of(&value));
    }

    #[test]
    fn value_is_objectof_number() {
        let value = Value::Object {
            content: [("key".to_string(), Value::Number { optional: false })].into(),
            optional: false,
        };
        assert!(IsObjectOf::<Number>::is_object_of(&value, "key"));
    }

    #[test]
    fn value_is_not_objectof_string() {
        let value = Value::Object {
            content: [("key".to_string(), Value::Number { optional: false })].into(),
            optional: false,
        };
        assert!(!IsObjectOf::<String>::is_object_of(&value, "key"));
    }

    #[test]
    fn test_is_tuple_of_match() {
        let value = Value::Tuple {
            elements: vec![
                Value::Number { optional: false },
                Value::String { optional: false },
            ],
            optional: false,
        };
        let types = vec![
            Value::Number { optional: false },
            Value::String { optional: false },
        ];
        assert!(value.is_tuple_of(&types));
    }

    #[test]
    fn test_is_tuple_of_match_with_optional() {
        let value = Value::Tuple {
            elements: vec![
                Value::Number { optional: true },
                Value::String { optional: false },
            ],
            optional: false,
        };
        let types = vec![
            Value::Number { optional: false },
            Value::String { optional: false },
        ];
        assert!(!value.is_tuple_of(&types));
    }

    #[test]
    fn test_is_tuple_of_mismatch() {
        let value = Value::Tuple {
            elements: vec![
                Value::Number { optional: false },
                Value::String { optional: false },
            ],
            optional: false,
        };
        let types = vec![
            Value::Number { optional: false },
            Value::Bool { optional: false },
        ];
        assert!(!value.is_tuple_of(&types));
    }

    #[test]
    fn test_is_tuple_of_length_mismatch() {
        let value = Value::Tuple {
            elements: vec![
                Value::Number { optional: false },
                Value::String { optional: false },
            ],
            optional: false,
        };
        let types = vec![Value::Number { optional: false }];
        assert!(!value.is_tuple_of(&types));
    }

    #[test]
    fn test_is_tuple_of_not_tuple() {
        let value = Value::Number { optional: false };
        let types = vec![
            Value::Number { optional: false },
            Value::String { optional: false },
        ];
        assert!(!value.is_tuple_of(&types));
    }
}
