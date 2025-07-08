use crate::{serde::JsonVisitor, value::Value as JsonShape};

impl<'json> From<&'json serde_json::Value> for JsonVisitor<'json> {
    fn from(value: &'json serde_json::Value) -> Self {
        let shape = JsonShape::from(value);
        Self { value, shape }
    }
}

impl JsonVisitor<'_> {
    /// Returns the original [`serde_json::Value`]
    #[must_use]
    pub const fn value(&self) -> &serde_json::Value {
        self.value
    }

    /// Returns the [`JsonShape`] of the original [`serde_json::Value`]
    #[must_use]
    pub const fn shape(&self) -> &JsonShape {
        &self.shape
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_value_null() {
        let value = serde_json::Value::Null;
        let visitor = JsonVisitor::from(&value);
        assert_eq!(visitor.value(), &value);
        assert_eq!(visitor.shape(), &JsonShape::Null);
    }

    #[test]
    fn from_value_number() {
        let value = serde_json::Value::Number(3.into());
        let visitor = JsonVisitor::from(&value);
        assert_eq!(visitor.value(), &value);
        assert_eq!(visitor.shape(), &JsonShape::Number { optional: false });
    }
}
