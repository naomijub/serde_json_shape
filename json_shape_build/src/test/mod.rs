static LARGE_OBJECT: &str = "#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Struct9Crc477AE068 {
    pub array: Vec<f64>,
    pub array_of_maps: Vec<Struct3CrcFDD0C6E5>,
    pub bool_false: bool,
    pub bool_true: bool,
    pub map: Struct2CrcDEC58CB4,
    pub nil: (),
    pub number: f64,
    pub str: String,
    pub tuple: (f64, String, bool),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Struct3CrcFDD0C6E5 {
    pub a: String,
    pub b: Option<bool>,
    pub c: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Struct2CrcDEC58CB4 {
    pub a: String,
    pub c: f64,
}";

#[test]
fn codegen_large_object() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let extesion = "fixture/object.json";
    let path = std::path::Path::new(dir).join(extesion);

    let file = crate::compile_json("collection", &[path]).unwrap();

    assert_eq!(file, LARGE_OBJECT);
}

#[test]
fn codegen_oneof() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let extesion = "fixture/a.json";
    let path_1 = std::path::Path::new(dir).join(extesion);

    let extesion = "fixture/b.json";
    let path_2 = std::path::Path::new(dir).join(extesion);

    let extesion = "fixture/c.json";
    let path_3 = std::path::Path::new(dir).join(extesion);

    let file = crate::compile_json("collection", &[path_1, path_2, path_3]).unwrap();

    assert_eq!(
        file,
        "#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Struct1CrcB3DA869A {
    pub a: Enum3CrcEFC15C8A,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Enum3CrcEFC15C8A {
    Bool(bool),
    Number(f64),
    Struct1CrcF5B399AC(Struct1CrcF5B399AC),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Struct1CrcF5B399AC {
    pub b: bool,
}"
    );
}
