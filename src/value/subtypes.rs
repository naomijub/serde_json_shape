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
/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `Object`.
pub struct Object;
/// Simple helper phantom struct to determine if `JsonShape` is of specific subtype `OneOf`.
pub struct OneOf;

/// Simple helper struct to determine if `JsonShape` is of specific optinal subtype.
pub struct Optional<U>(std::marker::PhantomData<U>);

mod private {
    use crate::value::Value;

    pub trait Sealed {}
    impl Sealed for Value {}
}

/// Checks if [`JsonShape`] is an Array of `T`
pub trait IsArrayOf<T>: private::Sealed {
    /// Checks if [`JsonShape`] is an Array of `T`
    /// - `value.is_array_of<Null>()`.
    fn is_array_of(&self) -> bool;
}

/// Checks if [`JsonShape`] is `OneOf` containing `T`
pub trait IsOneOf<T>: private::Sealed {
    /// Checks if [`JsonShape`] is `OneOf` containing `T`
    /// - `value.is_one_of<Null>()`.
    fn is_one_of(&self) -> bool;
}

/// Checks if [`JsonShape`] is `Object` containing `key: &str` and  `value: T`
pub trait IsObjectOf<T>: private::Sealed {
    /// Checks if [`JsonShape`] is `Object` containing `key: &str` and  `value: T`
    /// - `value.is_object_of<Null>("key_1")`.
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

impl IsOneOf<Optional<Number>> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::Number { optional: true }))
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
                .any(|variant| matches!(&variant, &Value::String { optional: true }))
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
                .any(|variant| matches!(&variant, &Value::Bool { optional: true }))
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
                .any(|variant| matches!(&variant, &Value::Array { optional: true, .. }))
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
                .any(|variant| matches!(&variant, &Value::Object { optional: true, .. }))
        } else {
            false
        }
    }
}

impl IsOneOf<Optional<OneOf>> for Value {
    fn is_one_of(&self) -> bool {
        if let Value::OneOf { variants, .. } = self {
            variants
                .iter()
                .any(|variant| matches!(&variant, &Value::OneOf { optional: true, .. }))
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
