#![doc = include_str!("../../README.md")]
#![allow(clippy::redundant_pub_crate)]
/// Module containing Error types
pub mod error;
mod value;

mod lexer;
pub(crate) mod parser;
/// [`serde_json`] related functions and types
pub mod serde;
pub(crate) mod shape;

use std::str::FromStr;

use crate::{
    error::Error,
    parser::Parser,
    shape::{merger::merge, parse_cst},
    value::Value,
};

pub use value::Similar;
pub use value::Value as JsonShape;

/// Creates a [`JsonShape`] from a single Json source
/// ```
/// use json_shape::JsonShape;
/// use std::str::FromStr;
///
/// let source = "[12, 34, 56]";
/// let json_shape = JsonShape::from_str(source).unwrap();
/// ```
impl FromStr for Value {
    type Err = Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source)?;

        Ok(value)
    }
}

impl Value {
    /// Creates a [`JsonShape`] from multiple Json sources
    ///
    /// # Errors
    ///
    /// Will return `Err` if failed to parse Json or if shapes don't align.
    pub fn from_sources(sources: &[&str]) -> Result<Self, Error> {
        let mut diags = Vec::new();
        let mut values = Vec::new();
        for source in sources {
            let cst = Parser::parse(source, &mut diags);

            values.push(parse_cst(&cst, source)?);
        }

        merge(&values)
    }

    /// Checks if Json is subset of specific [`JsonShape`]
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use json_shape::{IsSubset, JsonShape};
    /// let shape = JsonShape::Object { content: [
    ///     ("name".to_string(), JsonShape::String { optional: false }),
    ///     ("surname".to_string(), JsonShape::String { optional: false }),
    ///     ("middle name".to_string(), JsonShape::String { optional: true }),
    ///     ("age".to_string(), JsonShape::Number { optional: false }),
    ///     ("id".to_string(), JsonShape::OneOf { variants: [
    ///         JsonShape::Object { content: [
    ///             ("number".to_string(), JsonShape::Number { optional: false }),
    ///             ("state".to_string(), JsonShape::String { optional: false }),
    ///         ].into(), optional: false },
    ///         JsonShape::Array { r#type: Box::new(JsonShape::Number { optional: false }), optional: false }
    ///     ].into(), optional: false })
    /// ].into(), optional: false };
    ///
    /// let json = r#"{
    /// "name": "lorem",
    /// "surname": "ipsum",
    /// "age": 30,
    /// "id": {
    ///     "number": 123456,
    ///     "state": "st"
    /// }
    /// }"#;
    ///
    /// let shape_1 = JsonShape::from_str(json).unwrap();
    ///
    /// # assert!(
    /// shape_1.is_subset(&shape)
    /// # );
    /// # assert!(
    /// shape.is_superset(json)
    /// # );
    /// ```
    #[must_use]
    pub fn is_superset(&self, json: &str) -> bool {
        let Ok(value) = Self::from_str(json) else {
            return false;
        };

        value.is_subset(self)
    }

    /// Checks if Json is subset of specific [`JsonShape`]
    ///
    /// - Checked version of [`is_superset`]
    ///
    /// # Errors
    ///
    /// Returns `Err` if failed to parse Json
    pub fn is_superset_checked(&self, json: &str) -> Result<bool, Error> {
        let value = Self::from_str(json)?;

        Ok(value.is_subset(self))
    }
}

/// Determines if `T::self` is subset of `T`.
/// Only implemented for [`JsonShape`]/[`Value`]. A few rule examples
///
/// - `JsonShape::Number` is subset of `JsonShape::Option<Number>`
/// - `JsonShape::Null` is subset of `JsonShape::Option<Number>` and  `JsonShape::Null`
/// - `JsonShape::Number` is subset of `JsonShape::OneOf[Number | String]`
/// - `JsonShape::Number` is *NOT* subset of `JsonShape::Array<Number>` => `1.23 != [1.23]`
/// - `JsonShape::Array<Number>` is subset of `JsonShape::Array<OnOf<[Number | Boolean]>>`
/// - `JsonShape::Object{"key_a": JsonShape::Number}` is *NOT* subset of `JsonShape::Object{"key_b": JsonShape::Number}` => `key_a != key_b`
/// - `JsonShape::Object{"key_a": JsonShape::Number}` is subset of `JsonShape::Object{"key_a": JsonShape::Option<Number>}`
/// - `JsonShape::Object{"key_a": JsonShape::Number}` is subset of `JsonShape::Object{"key_a": JsonShape::OneOf[Number | Boolean]}`
pub trait IsSubset {
    /// Determines if `T::self` is subset of other `T`
    fn is_subset(&self, other: &Self) -> bool;
}
