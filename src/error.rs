#![cfg(not(tarpaulin_include))]

use std::ops::Range;

use crate::value::Value;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
/// Json shapes related errors
pub enum Error {
    /// Unexpected error has occurred. Usually something with parsing.
    #[error("unknown error has occurred")]
    Unknown,
    /// Invalid json, returns the invalid portion with the range where it is located.
    #[error("invalid json")]
    InvalidJson(String, Range<usize>),
    /// Json that has too many root nodes. eg, two objects not contained in an array.
    #[error("invalid json: Too many root nodes, expected 1.")]
    TooManyRootNodes,
    /// Invalid type was found
    #[error("invalid type `{0}`. Expected number, string, boolean, null, array or object.")]
    InvalidType(String),
    /// Object member doesnt have a key for the value.
    #[error("JSON::object requires key value pairs. Key missing")]
    InvalidObjectKey,
    /// Object member requires an expected type: `Boolean`, `String`, `Number`, `Null`, `Array`, `Object`, `OneOf`.
    #[error(
        "JSON::object requires value to be one of `Boolean`, `String`, `Number`, `Null`, `Array`, `Object`, `OneOf`"
    )]
    InvalidObjectValue,
    /// Object expected a type but found something unexpected.
    #[error("invalid type `{0}`. Expected `{1}`.")]
    InvalidObjectValueType(Value, Value),
}
