#![doc = include_str!("../README.md")]

/// Module containing Error types
pub mod error;
mod value;

mod lexer;
pub(crate) mod parser;
mod shape;

use std::str::FromStr;

use crate::{error::Error, parser::Parser, shape::parse_cst, value::Value};

pub use value::Value as JsonShape;

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
