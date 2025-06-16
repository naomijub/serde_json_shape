#![doc = include_str!("../README.md")]
#![allow(clippy::redundant_pub_crate)]
/// Module containing Error types
pub mod error;
mod value;

mod lexer;
pub(crate) mod parser;
pub(crate) mod shape;

use std::str::FromStr;

use crate::{
    error::Error,
    parser::Parser,
    shape::{merger::merge, parse_cst},
    value::Value,
};

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

        merge(values)
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
