// Mostly codegen file
#![cfg(not(tarpaulin_include))]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
use super::lexer::{Token, tokenize};
use codespan_reporting::diagnostic::Label;
use serde::Serialize;

pub type Diagnostic = codespan_reporting::diagnostic::Diagnostic<()>;

#[derive(Default)]
pub struct Context<'a> {
    marker: std::marker::PhantomData<&'a ()>,
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

impl ParserCallbacks for Parser<'_> {
    fn create_tokens(source: &str, diags: &mut Vec<Diagnostic>) -> (Vec<Token>, Vec<Span>) {
        tokenize(source, diags)
    }
    fn create_diagnostic(&self, span: Span, message: String) -> Diagnostic {
        Diagnostic::error()
            .with_message(message)
            .with_labels(vec![Label::primary((), span)])
    }
}

impl PartialEq for CstIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rule(l0, l1), Self::Rule(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::Token(l0, l1), Self::Token(r0, r1)) => l0 == r0 && l1 == r1,
            _ => false,
        }
    }
}

impl PartialEq for Cst<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.spans == other.spans
            && self.nodes == other.nodes
            && self.token_count == other.token_count
            && self.non_skip_len == other.non_skip_len
    }
}

impl std::fmt::Debug for Cst<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cst")
            .field("source", &self.source)
            .field("spans", &self.spans)
            .field("nodes", &self.nodes)
            .field("token_count", &self.token_count)
            .field("non_skip_len", &self.non_skip_len)
            .finish()
    }
}

impl Serialize for Cst<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.source)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use insta::assert_snapshot;

    #[test]
    fn parse_json() {
        let json = r#"{
            "str": "this is a string",
            "number": 123.456,
            "bool_true": true,
            "bool_false": false,
            "nil": null,
            "array": [123, "string", true],
            "map": {
            "a": "b",
            "c": 123
            },
            "array of maps": [
                {
                    "a": "b",
                    "c": 123
                },
                {
                    "a": "b",
                    "b": true
                }
            ]
        }"#;

        let mut diags = Vec::new();
        let parsed = Parser::parse(json, &mut diags);

        assert_snapshot!(parsed);
    }
}
