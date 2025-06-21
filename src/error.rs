#![cfg(not(tarpaulin_include))]

use std::ops::Range;

use crate::value::Value;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
/// JSON shapes related errors
pub enum Error {
    /// Unexpected error has occurred. Usually something with parsing.
    #[error("unknown error has occurred")]
    Unknown,
    /// Empty file while parsing.
    #[error("JSON content is empty")]
    EmptyFile,
    /// Invalid JSON, returns the invalid portion with the range where it is located.
    #[error("invalid JSON `{value}`: {}..{}", span.start, span.end)]
    InvalidJson {
        /// Invalid json content
        value: String,
        /// Invalid json range
        span: Range<usize>,
    },
    /// JSON that has too many root nodes. eg, two objects not contained in an array.
    #[error("invalid JSON: Too many root nodes, expected 1.")]
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
    /// Not able to merge the two [`JsonShapes`]
    #[error("not able to merge `{0}` with `{1}`.")]
    CannotMerge(Value, Value),
}
