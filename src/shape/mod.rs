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
        let range = cst.span(NodeRef::ROOT);
        let content = &source[range.clone()];
        return Err(Error::InvalidJson(content.to_string(), range));
    };

    has_errors(cst, source, NodeRef::ROOT)?;
    if cst.children(NodeRef::ROOT).count() > 1 {
        return Err(Error::TooManyRootNodes);
    }
    let Some(first_node_ref) = cst.children(NodeRef::ROOT).find(|node_ref| {
        !matches!(
            cst.get(*node_ref),
            Node::Token(Token::Whitespace | Token::Newline, _)
        )
    }) else {
        let range = cst.span(NodeRef::ROOT);
        let content = &source[range.clone()];
        return Err(Error::InvalidJson(content.to_string(), range));
    };

    parse_rule(cst, first_node_ref, source)
}

fn has_errors(cst: &Cst<'_>, source: &str, root: NodeRef) -> Result<(), Error> {
    if cst
        .children(root)
        .any(|node_ref| matches!(cst.get(node_ref), Node::Token(Token::Error, _)))
    {
        let error = cst
            .children(NodeRef::ROOT)
            .find(|node_ref| matches!(cst.get(*node_ref), Node::Token(Token::Error, _)))
            .unwrap();
        let span = cst.span(error);
        let value = &source[span.clone()];
        return Err(Error::InvalidJson(value.to_string(), span));
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
            let range = cst.span(node_ref);
            let content = &source[range.clone()];
            Err(Error::InvalidJson(content.to_string(), range))
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
