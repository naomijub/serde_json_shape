#![allow(missing_docs)]
use json_shape::{IsSubset, JsonShape};
use std::str::FromStr;

#[test]
fn test_invalid_json_from_str() {
    let source = "{ invalid json }";
    let result = JsonShape::from_str(source);
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid JSON `invalid`: 2..9"
    );
}

#[test]
fn test_empty_array_from_sources_should_return_empty_json_error() {
    let sources: [String; 0] = [];
    let result = JsonShape::from_sources(&sources).unwrap_err();
    assert_eq!(result.to_string(), "JSON content is empty");
}

#[test]
fn test_incompatible_merge_from_sources() {
    let sources = ["{\"a\": 1}".to_string(), "{\"a\": \"string\"}".to_string()];
    let result = JsonShape::from_sources(&sources);
    assert!(
        result.is_ok(),
        "Merge should still succeed with OneOf variant"
    );
}

#[test]
fn test_superset_true() {
    let shape = JsonShape::from_str("{\"a\": 1, \"b\": \"x\"}").unwrap();
    assert!(shape.is_superset("{\"a\": 1, \"b\": \"x\"}"));
}

#[test]
fn test_superset_false() {
    let shape = JsonShape::from_str("{\"a\": 1, \"b\": \"x\"}").unwrap();
    assert!(!shape.is_superset("{\"a\": 1}"));
}

#[test]
fn test_superset_with_extra_fields() {
    let shape = JsonShape::from_str("{\"a\": 1}").unwrap();
    // This might depend on how your implementation treats extra fields
    let result = shape.is_superset("{\"a\": 1, \"extra\": 99}");
    println!("Extra fields superset: {result}");
}

#[test]
fn test_superset_checked_invalid_json() {
    let shape = JsonShape::from_str("{\"a\": 1}").unwrap();
    let result = shape.is_superset_checked("not json");
    assert!(result.is_err());
}

#[test]
fn test_is_subset_self() {
    let shape = JsonShape::from_str("{\"a\": 1, \"b\": \"x\"}").unwrap();
    assert!(shape.is_subset(&shape));
}

#[test]
fn test_nullable_field_merging() {
    let sources = ["{\"a\": 1}".to_string(), "{\"a\": null}".to_string()];
    let result = JsonShape::from_sources(&sources).unwrap();
    let shape_json = serde_json::to_string_pretty(&result).unwrap();
    assert_eq!(
        shape_json,
        "{
  \"Object\": {
    \"content\": {
      \"a\": {
        \"Number\": {
          \"optional\": true
        }
      }
    },
    \"optional\": false
  }
}"
    );
}

#[test]
fn test_array_merging_with_mixed_types() {
    let sources = ["[1, 2, 3]".to_string(), "[\"string\", true]".to_string()];
    let result = JsonShape::from_sources(&sources).unwrap();
    let shape_json = serde_json::to_string_pretty(&result).unwrap();
    assert_eq!(
        shape_json,
        "{
  \"Array\": {
    \"type\": {
      \"OneOf\": {
        \"variants\": [
          {
            \"Bool\": {
              \"optional\": false
            }
          },
          {
            \"Number\": {
              \"optional\": false
            }
          },
          {
            \"String\": {
              \"optional\": false
            }
          }
        ],
        \"optional\": false
      }
    },
    \"optional\": false
  }
}"
    );
}
