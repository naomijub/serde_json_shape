use std::{collections::BTreeMap, str::FromStr};

use crate::{
    error::Error,
    lexer::Token,
    parser::{Cst, Node, NodeRef, Rule},
    value::Value,
};

pub(crate) mod merger;

pub fn parse_cst(cst: &Cst<'_>, source: &str) -> Result<Value, Error> {
    let Node::Rule(Rule::File, _) = cst.get(NodeRef::ROOT) else {
        let span = cst.span(NodeRef::ROOT);
        let value = source[span.clone()].to_string();
        return Err(Error::InvalidJson { value, span });
    };

    has_errors(cst, source, NodeRef::ROOT)?;
    if cst.children(NodeRef::ROOT).filter(|node_ref| !matches!(cst.get(*node_ref), Node::Token(Token::Whitespace | Token::Newline, _))).count() > 1 {
        if let Some(err) = cst
            .children(NodeRef::ROOT)
            .find(|node_ref| has_errors(cst, source, *node_ref).is_err())
        {
            let span = cst.span(err);
            let value = source[span.clone()].to_string();
            return Err(Error::InvalidJson { value, span });
        }
        return Err(Error::TooManyRootNodes(cst.children(NodeRef::ROOT).count()));
    }
    let Some(first_node_ref) = cst.children(NodeRef::ROOT).find(|node_ref| {
        !matches!(
            cst.get(*node_ref),
            Node::Token(Token::Whitespace | Token::Newline, _)
        )
    }) else {
        let span = cst.span(NodeRef::ROOT);
        let value = source[span.clone()].to_string();
        return Err(Error::InvalidJson { value, span });
    };

    parse_rule(cst, first_node_ref, source)
}

fn has_errors(cst: &Cst<'_>, source: &str, root: NodeRef) -> Result<(), Error> {
    if cst.children(root).any(|node_ref| {
        matches!(
            cst.get(node_ref),
            Node::Token(Token::Error, _) | Node::Rule(Rule::Error, _)
        )
    }) {
        if let Some(error) = cst.children(root).find(|node_ref| {
            matches!(
                cst.get(*node_ref),
                Node::Token(Token::Error, _) | Node::Rule(Rule::Error, _)
            )
        }) {
            let span = cst.span(error);
            let value = source[span.clone()].to_string();
            return Err(Error::InvalidJson { value, span });
        }
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn parse_rule(cst: &Cst<'_>, node_ref: NodeRef, source: &str) -> Result<Value, Error> {
    match cst.get(node_ref) {
        Node::Rule(Rule::Literal, ..) => {
            has_errors(cst, source, node_ref)?;
            parse_token(
                cst,
                cst.children(node_ref)
                    .next()
                    .ok_or_else(|| Error::InvalidType("Empty".to_string()))?,
            )
        }
        Node::Rule(Rule::Boolean, ..) => Ok(Value::Bool { optional: false }),
        Node::Rule(Rule::Array, ..) => {
            has_errors(cst, source, node_ref)?;
            let mut elements = Vec::new();
            for sub_node in cst.children(node_ref).filter(|node_ref| {
                !matches!(
                    cst.get(*node_ref),
                    Node::Token(
                        Token::Whitespace
                            | Token::Newline
                            | Token::Comma
                            | Token::LBrak
                            | Token::RBrak,
                        _
                    )
                )
            }) {
                let shape = parse_rule(cst, sub_node, source)?;
                elements.push(shape);
            }

            if elements.len() == 1 || elements.windows(2).all(|w| w[0] == w[1]) {
                Ok(Value::Array {
                    r#type: Box::new(elements.first().cloned().unwrap()),
                    optional: false,
                })
            } else if elements.len() > 1
                && elements
                    .iter()
                    .all(|value| matches!(value, Value::Object { .. }))
            {
                let mut iter = elements.iter();
                let Some(Value::Object { content, .. }) = iter.next().cloned() else {
                    return Err(Error::Unknown);
                };
                let content =
                    iter.clone()
                        .filter_map(Value::keys)
                        .fold(content, |mut acc, mut keys| {
                            for (key, value) in &mut acc {
                                if !keys.any(|k| k == key) {
                                    value.to_optional_mut();
                                }
                            }
                            acc
                        });
                let object = iter.fold(content, |mut acc, content| {
                    let Value::Object { content, .. } = content else {
                        return acc;
                    };
                    for (key, value) in content {
                        let old_value = acc
                            .entry(key.clone())
                            .or_insert_with(|| value.clone().as_optional());
                        if let Value::OneOf { variants, .. } = old_value {
                            variants.insert(value.clone());
                        }
                    }
                    acc
                });

                Ok(Value::Array {
                    r#type: Box::new(Value::Object {
                        content: object,
                        optional: false,
                    }),
                    optional: false,
                })
            } else if elements.len() > 1 {
                Ok(Value::Tuple {
                    elements,
                    optional: false,
                })
            } else {
                Ok(Value::Array {
                    r#type: Box::new(Value::Null),
                    optional: true,
                })
            }
        }
        Node::Rule(Rule::Object, ..) => {
            let mut content = BTreeMap::default();
            has_errors(cst, source, node_ref)?;
            for sub_node in cst
                .children(node_ref)
                .filter(|node_ref| matches!(cst.get(*node_ref), Node::Rule(Rule::Member, _)))
            {
                parse_member(cst, sub_node, source, &mut content)?;
            }

            Ok(Value::Object {
                content,
                optional: false,
            })
        }
        _ => {
            let span = cst.span(node_ref);
            let value = source[span.clone()].to_string();
            Err(Error::InvalidJson { value, span })
        }
    }
}

fn parse_token(cst: &Cst<'_>, node_ref: NodeRef) -> Result<Value, Error> {
    match cst.get(node_ref) {
        Node::Rule(Rule::Boolean, _) | Node::Token(Token::False | Token::True, _) => {
            Ok(Value::Bool { optional: false })
        }
        Node::Rule(..) => Err(Error::Unknown),
        Node::Token(Token::Null, _) => Ok(Value::Null),
        Node::Token(Token::String, _) => Ok(Value::String { optional: false }),
        Node::Token(Token::Number, _) => Ok(Value::Number { optional: false }),
        Node::Token(token, _) => Err(Error::InvalidType(token.to_string())),
    }
}

fn parse_member(
    cst: &Cst<'_>,
    sub_node: NodeRef,
    source: &str,
    content: &mut BTreeMap<String, Value>,
) -> Result<(), Error> {
    let Some(key) = cst
        .children(sub_node)
        .find(|node_ref| matches!(cst.get(*node_ref), Node::Token(Token::String, _)))
    else {
        return Err(Error::InvalidObjectKey);
    };

    let key_span = cst.span(key);
    let key = String::from_str(&source[key_span.start + 1..key_span.end - 1]).unwrap_or_default();

    has_errors(cst, source, sub_node)?;
    let Some(member_value) = cst.children(sub_node).find(|node_ref| {
        matches!(
            cst.get(*node_ref),
            Node::Rule(
                Rule::Array | Rule::Boolean | Rule::Literal | Rule::Object,
                _
            )
        )
    }) else {
        return Err(Error::InvalidObjectValue);
    };

    let value = parse_rule(cst, member_value, source)?;
    match content.get(&key) {
        Some(Value::OneOf { variants, .. }) => {
            if !variants.contains(&value) {
                return Err(Error::InvalidObjectValueType(
                    value,
                    Value::OneOf {
                        variants: variants.clone(),
                        optional: false,
                    },
                ));
            }
        }
        Some(other) => {
            if value != *other {
                return Err(Error::InvalidObjectValueType(value, other.to_owned()));
            }
        }
        None => {
            content.insert(key, value);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn parse_null() {
        let source = "null";
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(value, Value::Null);
    }

    #[test]
    fn parse_number() {
        let source = "123";
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(value, Value::Number { optional: false });
    }

    #[test]
    fn parse_string() {
        let source = "\"123\"";
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(value, Value::String { optional: false });
    }

    #[test]
    fn parse_bool() {
        let source = "true";
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(value, Value::Bool { optional: false });
    }

    #[test]
    fn parse_array() {
        let source = "[12, 34, 56, 78]";
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(
            value,
            Value::Array {
                r#type: Box::new(Value::Number { optional: false }),
                optional: false
            }
        );
    }

    #[test]
    fn parse_array_other() {
        let source = "[true, false, true]";
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(
            value,
            Value::Array {
                r#type: Box::new(Value::Bool { optional: false }),
                optional: false
            }
        );
    }

    #[test]
    fn parse_tuple() {
        let source = "[12, true, \"str\"]";
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(
            value,
            Value::Tuple {
                elements: vec![
                    Value::Number { optional: false },
                    Value::Bool { optional: false },
                    Value::String { optional: false }
                ],
                optional: false
            }
        );
    }

    #[test]
    fn parse_object() {
        let source = r#"{"key": 123, "key2": true}"#;
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(
            value,
            Value::Object {
                content: [
                    ("key".to_string(), Value::Number { optional: false }),
                    ("key2".to_string(), Value::Bool { optional: false })
                ]
                .into(),
                optional: false
            }
        );
    }

    #[test]
    fn parse_array_of_objects_diff() {
        let source = r#"[{"a": 1}, {"b": 2}, {"c": 3}, {}]"#;
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(
            value,
            Value::Array {
                r#type: Box::new(Value::Object {
                    content: [
                        ("a".to_string(), Value::Number { optional: true }),
                        ("b".to_string(), Value::Number { optional: true }),
                        ("c".to_string(), Value::Number { optional: true })
                    ]
                    .into(),
                    optional: false
                }),
                optional: false
            }
        );
    }

    #[test]
    fn parse_array_of_objects_same() {
        let source = r#"[{"a": 1}, {"a": 2}, {"a": 3}]"#;
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(
            value,
            Value::Array {
                r#type: Box::new(Value::Object {
                    content: [("a".to_string(), Value::Number { optional: false })].into(),
                    optional: false
                }),
                optional: false
            }
        );
    }

    #[test]
    fn parse_array_of_objects_diff_single_key() {
        let source = r#"[{"a": 1}, {"a": 4, "b": 2}, {"a" : 5, "c": 3}]"#;
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(
            value,
            Value::Array {
                r#type: Box::new(Value::Object {
                    content: [
                        ("a".to_string(), Value::Number { optional: false }),
                        ("b".to_string(), Value::Number { optional: true }),
                        ("c".to_string(), Value::Number { optional: true })
                    ]
                    .into(),
                    optional: false
                }),
                optional: false
            }
        );
    }

    #[test]
    fn parse_array_of_emptyobject() {
        let source = "[{}]";
        let cst = Parser::parse(source, &mut Vec::new());

        let value = parse_cst(&cst, source).unwrap();

        assert_eq!(
            value,
            Value::Array {
                r#type: Box::new(Value::Object {
                    content: BTreeMap::default(),
                    optional: false
                }),
                optional: false
            }
        );
    }
}

#[cfg(test)]
mod test_errors {
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn parse_multiple_roots() {
        let source = "123 true \"str\"";
        let cst = Parser::parse(source, &mut Vec::new());

        let err = parse_cst(&cst, source).unwrap_err();

        assert_eq!(err.to_string(), "invalid JSON `true \"str\"`: 4..14");
    }

    #[test]
    fn parse_only_ws() {
        let source = "         ";
        let cst = Parser::parse(source, &mut Vec::new());

        let err = parse_cst(&cst, source).unwrap_err();

        assert_eq!(err.to_string(), "invalid JSON `         `: 0..9");
    }

    #[test]
    fn parse_only_mismatch() {
        let source = "{ 123: 123 }";
        let cst = Parser::parse(source, &mut Vec::new());

        let err = parse_cst(&cst, source).unwrap_err();

        assert_eq!(err.to_string(), "invalid JSON `123: 123 }`: 2..12");
    }

    #[test]
    fn parse_only_mismatch_unterminated_key() {
        let source = "{ \"123: 123 }";
        let cst = Parser::parse(source, &mut Vec::new());

        let err = parse_cst(&cst, source).unwrap_err();

        assert_eq!(err.to_string(), "invalid JSON `\"123: 123 }`: 2..13");
    }

    #[test]
    fn parse_unterminated_string() {
        let source = "\"123";
        let cst = Parser::parse(source, &mut Vec::new());

        let err = parse_cst(&cst, source).unwrap_err();

        assert_eq!(err.to_string(), "invalid JSON `\"123`: 0..4");
    }

    #[test]
    fn parse_uninit_string() {
        let source = "123\"";
        let cst = Parser::parse(source, &mut Vec::new());

        let err = parse_cst(&cst, source).unwrap_err();

        assert_eq!(err.to_string(), "invalid JSON `\"`: 3..4");
    }

    #[test]
    fn parse_invalid_number() {
        let source = "123..43";
        let cst = Parser::parse(source, &mut Vec::new());

        let err = parse_cst(&cst, source).unwrap_err();

        assert_eq!(err.to_string(), "invalid JSON `.`: 3..4");
    }
}
