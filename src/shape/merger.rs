use std::collections::BTreeSet;

use crate::{error::Error, value::Value};

pub fn merge(values: Vec<Value>) -> Result<Value, Error> {
    if values.iter().all(super::super::value::Value::is_single) {
        let merged = values.into_iter().fold(
            Value::OneOf {
                variants: BTreeSet::new(),
                optional: false,
            },
            |mut set, v| {
                let Value::OneOf { variants, .. } = &mut set else {
                    return set;
                };
                variants.insert(v);
                set
            },
        );
        return Ok(merged);
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn merge_simple_json_objects_as_one_of() {
        let value_1 = Value::Null;
        let value_2 = Value::Bool { optional: false };
        let value_3 = Value::Number { optional: false };
        let value_4 = Value::String { optional: false };

        let result = merge(vec![value_1, value_2, value_3, value_4]).unwrap();

        assert_eq!(
            result,
            Value::OneOf {
                variants: BTreeSet::from_iter([
                    Value::Null,
                    Value::Bool { optional: false },
                    Value::Number { optional: false },
                    Value::String { optional: false }
                ]),
                optional: false
            }
        );
    }
}
