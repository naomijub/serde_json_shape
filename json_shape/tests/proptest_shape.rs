#![allow(missing_docs)]

use std::{collections::BTreeMap, str::FromStr};

use json_shape::JsonShape;
use proptest::prelude::*;
use serde_json::{Map, Value};

fn arb_json_key() -> impl Strategy<Value = String> {
    prop::collection::vec((b'a'..=b'z').prop_map(|byte| byte as char), 1..6)
        .prop_map(|bytes| bytes.into_iter().collect::<String>())
}

fn arb_json_string() -> impl Strategy<Value = String> {
    prop::collection::vec((b'a'..=b'z').prop_map(|byte| byte as char), 0..16)
        .prop_map(|bytes| bytes.into_iter().collect::<String>())
}

fn arb_json_value() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        2 => Just(Value::Null),
        3 => any::<bool>().prop_map(Value::Bool),
        3 => any::<i64>().prop_map(|n| Value::Number(n.into())),
        3 => arb_json_string().prop_map(Value::String),
    ];

    leaf.prop_recursive(3, 24, 6, |inner| {
        let nested_object = inner.clone();
        prop_oneof![
            2 => prop::collection::vec(inner, 1..6).prop_map(Value::Array),
            2 => {
                prop::collection::btree_map(arb_json_key(), nested_object, 0..6)
                    .prop_map(|object: BTreeMap<String, Value>| {
                        Value::Object(object.into_iter().collect::<Map<String, Value>>())
                    })
            }
        ]
    })
}

fn arb_object_value() -> impl Strategy<Value = Value> {
    prop_oneof![
        1 => any::<bool>().prop_map(Value::Bool),
        1 => any::<i64>().prop_map(|n| Value::Number(n.into())),
        1 => arb_json_string().prop_map(Value::String),
    ]
}

fn arb_same_kind_sources() -> impl Strategy<Value = Vec<Value>> {
    prop_oneof![
        1 => (1usize..8).prop_flat_map(|n| {
            prop::collection::vec(any::<bool>(), n).prop_map(|values| {
                values.into_iter().map(Value::Bool).collect()
            })
        }),
        1 => (1usize..8).prop_flat_map(|n| {
            prop::collection::vec(any::<i64>(), n).prop_map(|values| {
                values.into_iter().map(|n| Value::Number(n.into())).collect()
            })
        }),
        1 => (1usize..8).prop_flat_map(|n| {
            prop::collection::vec(arb_json_string(), n).prop_map(|values| {
                values.into_iter().map(Value::String).collect()
            })
        }),
        1 => (1usize..8).prop_flat_map(|n| {
            prop::collection::vec(arb_object_value(), n)
                .prop_map(|values| {
                    values
                        .into_iter()
                        .map(|value| {
                            Value::Object(
                                std::iter::once(("k".to_string(), value)).collect::<Map<String, Value>>(),
                            )
                        })
                        .collect()
                })
        }),
        1 => (1usize..8).prop_map(|n| vec![Value::Null; n]),
    ]
}

proptest! {
    #[test]
    fn from_str_matches_from_json_value(value in arb_json_value()) {
        let source = serde_json::to_string(&value).unwrap();
        let from_str_shape = JsonShape::from_str(&source).unwrap();
        let from_json_shape = JsonShape::from(value);

        prop_assert_eq!(from_str_shape, from_json_shape);
    }

    #[test]
    fn from_sources_is_superset_of_each_source(values in arb_same_kind_sources()) {
        let sources = values
            .iter()
            .map(|value| serde_json::to_string(value).unwrap())
            .collect::<Vec<_>>();
        let shape = JsonShape::from_sources(&sources).unwrap();

        for source in &sources {
            prop_assert!(shape.is_superset(source));
            prop_assert!(shape.is_superset_checked(source).is_ok());
        }
    }

    #[test]
    fn from_sources_of_same_shape_is_idempotent(value in arb_json_value()) {
        let source = serde_json::to_string(&value).unwrap();
        let repeated_sources = vec![source.clone(), source.clone(), source];
        let merged = JsonShape::from_sources(&repeated_sources).unwrap();
        let base = JsonShape::from(value);

        prop_assert_eq!(merged, base);
    }
}
