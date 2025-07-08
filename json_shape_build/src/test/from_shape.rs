use codegen::Scope;
use json_shape::JsonShape;

use crate::first_pass;

#[test]
fn from_null() {
    let shape = JsonShape::Null;

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(file, "pub type Void = ();");
}

#[test]
fn from_number() {
    let shape = JsonShape::Number { optional: false };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(file, "pub type Number = f64;");
}

#[test]
fn from_opt_number() {
    let shape = JsonShape::Number { optional: true };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(file, "pub type NullableNumber = Option<f64>;");
}

#[test]
fn from_bool() {
    let shape = JsonShape::Bool { optional: false };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(file, "pub type Bool = bool;");
}

#[test]
fn from_opt_bool() {
    let shape = JsonShape::Bool { optional: true };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(file, "pub type NullableBool = Option<bool>;");
}

#[test]
fn from_str() {
    let shape = JsonShape::String { optional: false };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(file, "pub type Str = String;");
}

#[test]
fn from_opt_str() {
    let shape = JsonShape::String { optional: true };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(file, "pub type NullableStr = Option<String>;");
}

#[test]
fn from_array() {
    let shape = JsonShape::Array {
        r#type: Box::new(JsonShape::Number { optional: false }),
        optional: false,
    };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(file, "pub type ArrayOfNumber = Vec<f64>;");
}

#[test]
fn from_opt_array() {
    let shape = JsonShape::Array {
        r#type: Box::new(JsonShape::Number { optional: true }),
        optional: true,
    };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(
        file,
        "pub type OptionalArrayOfOptionalNumber = Option<Vec<Option<f64>>>;"
    );
}

#[test]
fn from_array_of_enum() {
    let shape = JsonShape::Array {
        r#type: Box::new(JsonShape::OneOf {
            variants: [
                JsonShape::Number { optional: false },
                JsonShape::String { optional: false },
            ]
            .into(),
            optional: false,
        }),
        optional: false,
    };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(
        file,
        "pub type ArrayOfEnum2CrcB0F27C9A = Vec<Enum2CrcB0F27C9A>;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Enum2CrcB0F27C9A {
    Number(f64),
    Str(String),
}"
    );
}

#[test]
fn from_tuple() {
    let shape = JsonShape::Tuple {
        elements: [
            JsonShape::Number { optional: false },
            JsonShape::String { optional: false },
            JsonShape::OneOf {
                variants: [
                    JsonShape::Number { optional: false },
                    JsonShape::Object {
                        content: [("key".to_string(), JsonShape::Number { optional: false })]
                            .into(),
                        optional: false,
                    },
                ]
                .into(),
                optional: false,
            },
        ]
        .into(),
        optional: false,
    };

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);

    let file = scope.to_string();
    assert_eq!(
        file,
        "pub type Tuple3CrcAF3E2524 = (f64, String, Enum2CrcEDFEBA3B);
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Enum2CrcEDFEBA3B {
    Number(f64),
    Struct1Crc913C1A62(Struct1Crc913C1A62),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Struct1Crc913C1A62 {
    pub key: f64,
}"
    );
}
