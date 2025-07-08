#![allow(missing_docs)]

use std::{fs::read_to_string, str::FromStr};

use insta::assert_snapshot;
use json_shape::JsonShape;
use serde_json::Value;


#[test]
fn example_1() {
    let json_str = read_to_string("./testdata/rfc-9535-example-1.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}

#[test]
fn example_2() {
    let json_str = read_to_string("./testdata/rfc-9535-example-2.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}

#[test]
fn example_3() {
    let json_str = read_to_string("./testdata/rfc-9535-example-3.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}

#[test]
fn example_4() {
    let json_str = read_to_string("./testdata/rfc-9535-example-4.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}

#[test]
fn example_5() {
    let json_str = read_to_string("./testdata/rfc-9535-example-5.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}

#[test]
fn example_6() {
    let json_str = read_to_string("./testdata/rfc-9535-example-6.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}

#[test]
fn example_7() {
    let json_str = read_to_string("./testdata/rfc-9535-example-7.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}

#[test]
fn example_8() {
    let json_str = read_to_string("./testdata/rfc-9535-example-8.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}

#[test]
fn example_9() {
    let json_str = read_to_string("./testdata/rfc-9535-example-9.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}

#[test]
fn example_10() {
    let json_str = read_to_string("./testdata/rfc-9535-example-10.json").unwrap();
    let shape = JsonShape::from_str(&json_str).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();
    let shape_from_value = JsonShape::from(&json);

    assert_eq!(shape, shape_from_value);
    assert_snapshot!(shape_from_value);
}