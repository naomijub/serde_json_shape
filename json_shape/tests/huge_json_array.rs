#![allow(missing_docs)]
use std::{fs::read_to_string, str::FromStr};

use insta::assert_snapshot;
use json_shape::JsonShape;

#[test]
fn test_json_array_variant() {
    let json = huge_json();

    let array = JsonShape::from_str(&json).unwrap();

    assert_snapshot!(array);
}

fn huge_json() -> String {
    read_to_string("./tests/fixture/test.json").unwrap()
}
