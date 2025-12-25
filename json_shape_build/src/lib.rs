//! Build-time compiler for `json_shape`

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use checksum::crc32::Crc32;
use codegen::{Field, Scope, Variant};
use convert_case::{Case, Casing};
use json_shape::JsonShape;

#[cfg(test)]
mod test;

/// Include generated json shapes as serializable structs.
///
/// You must specify the json collections name.
///
/// ```rust,ignore
/// mod shapes {
///     json_shape_build::include_json_shape!("helloworld");
/// }
/// ```
///
/// > # Note:
/// > **This only works if the `json_shape_build` output directory has been unmodified**.
/// > The default output directory is set to the [`OUT_DIR`] environment variable.
/// > If the output directory has been modified, the following pattern may be used
/// > instead of this macro.
///
/// ```rust,ignore
/// mod shapes {
///     include!("/relative/json_shape/directory/helloworld.rs");
/// }
/// ```
/// You can also use a custom environment variable using the following pattern.
/// ```rust,ignore
/// mod shapes {
///     include!(concat!(env!("JSON_SHAPE_DIR"), "/helloworld.rs"));
/// }
/// ```
///
/// [`OUT_DIR`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
#[macro_export]
macro_rules! include_json_shape {
    ($package: tt) => {
        include!(concat!(
            env!("OUT_DIR"),
            concat!("/", $package, ".gen.shape.rs")
        ));
    };
}

/// Simple `.json` shape compiling.
///
/// The include directory will be the parent folder of the specified path.
/// The package name will be the filename without the extension.
///
/// In your `build.rs`:
/// ```rust
/// let dir = env!("CARGO_MANIFEST_DIR");
/// let extension = "fixture/object.json";
/// let path = std::path::Path::new(dir).join(extension);
/// json_shape_build::compile_json("collection_name", &[path]);
/// ```
///
///
/// # Errors
/// - failed to write json shape file
#[allow(clippy::missing_panics_doc)]
pub fn compile_json(
    collection_name: &'static str,
    jsons: &[impl AsRef<Path>],
) -> std::io::Result<String> {
    for path in jsons {
        println!("cargo:rerun-if-changed={}", path.as_ref().display());
    }

    let sources = jsons
        .iter()
        .filter_map(|path| path.as_ref().to_str())
        .map(std::fs::read_to_string)
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    let shape = json_shape::JsonShape::from_sources(&sources).map_err(std::io::Error::other)?;
    let target: PathBuf =
        std::env::var_os("OUT_DIR").map_or_else(|| std::env::current_dir().unwrap(), PathBuf::from);
    let target = target.join(collection_name).with_extension("gen.shape.rs");

    let mut scope = Scope::new();

    first_pass(&shape, &mut scope);
    let content = format!(
        "//! Generated `JsonShape` file.\nuse serde;\n\n{}",
        scope.to_string()
    );
    std::fs::write(target, content)?;
    Ok(scope.to_string())
}

pub(crate) fn first_pass(shape: &JsonShape, scope: &mut Scope) {
    match &shape {
        json_shape::JsonShape::Null => {
            scope.new_type_alias("Void", "()").vis("pub");
        }
        json_shape::JsonShape::Bool { optional } => {
            if *optional {
                scope
                    .new_type_alias("NullableBool", "Option<bool>")
                    .vis("pub");
            } else {
                scope.new_type_alias("Bool", "bool").vis("pub");
            }
        }
        json_shape::JsonShape::Number { optional } => {
            if *optional {
                scope
                    .new_type_alias("NullableNumber", "Option<f64>")
                    .vis("pub");
            } else {
                scope.new_type_alias("Number", "f64").vis("pub");
            }
        }
        json_shape::JsonShape::String { optional } => {
            if *optional {
                scope
                    .new_type_alias("NullableStr", "Option<String>")
                    .vis("pub");
            } else {
                scope.new_type_alias("Str", "String").vis("pub");
            }
        }
        json_shape::JsonShape::Array {
            r#type: inner,
            optional,
        } => {
            let name = shape_name(shape);
            create_array(scope, &name, *optional, inner);
            create_subtype(scope, inner);
        }
        json_shape::JsonShape::Object { content, .. } => {
            let name = shape_name(shape);
            create_object(scope, &name, content);
            for inner in content.values() {
                create_subtype(scope, inner);
            }
        }
        json_shape::JsonShape::OneOf { variants, .. } => {
            let name = shape_name(shape);
            create_enum(scope, &name, variants);
            for inner in variants {
                create_subtype(scope, inner);
            }
        }
        json_shape::JsonShape::Tuple { elements, optional } => {
            let name = shape_name(shape);
            create_tuple(scope, &name, *optional, elements);
            for inner in elements {
                create_subtype(scope, inner);
            }
        }
    }
}

fn create_subtype(scope: &mut Scope, shape: &JsonShape) {
    match shape {
        json_shape::JsonShape::Array {
            r#type: inner,
            optional: _,
        } => {
            create_subtype(scope, inner);
        }
        json_shape::JsonShape::Object { content, .. } => {
            let name = shape_name(shape);
            create_object(scope, &name, content);
            for inner in content.values() {
                create_subtype(scope, inner);
            }
        }
        json_shape::JsonShape::OneOf { variants, .. } => {
            let name = shape_name(shape);
            create_enum(scope, &name, variants);
            for inner in variants {
                create_subtype(scope, inner);
            }
        }
        json_shape::JsonShape::Tuple {
            elements,
            optional: _,
        } => {
            for inner in elements {
                create_subtype(scope, inner);
            }
        }
        _ => {}
    }
}

fn create_object(scope: &mut Scope, name: &str, content: &BTreeMap<String, json_shape::JsonShape>) {
    let struct_data = scope
        .new_struct(name)
        .vis("pub")
        .derive("Debug")
        .derive("Clone")
        .derive("serde::Serialize")
        .derive("serde::Deserialize");
    for (name, r#type) in content {
        let mut field = Field::new(name.to_case(Case::Snake), shape_representation(r#type));
        field.vis("pub");
        struct_data.push_field(field);
    }
}

fn create_enum(scope: &mut Scope, name: &str, variants: &BTreeSet<json_shape::JsonShape>) {
    let variants = variants
        .iter()
        .map(|shape| (shape_name(shape), shape_representation(shape)))
        .map(|(name, representation)| {
            let mut var = Variant::new(name);
            var.tuple(&representation);
            var
        });
    let enum_data = scope
        .new_enum(name)
        .vis("pub")
        .derive("Debug")
        .derive("Clone")
        .derive("serde::Serialize")
        .derive("serde::Deserialize");
    for var in variants {
        enum_data.push_variant(var);
    }
}

fn create_array(scope: &mut Scope, name: &str, optional: bool, r#type: &JsonShape) {
    let target = if optional {
        format!("Option<Vec<{}>>", shape_representation(r#type))
    } else {
        format!("Vec<{}>", shape_representation(r#type))
    };
    scope.new_type_alias(name, target).vis("pub");
}

fn create_tuple(scope: &mut Scope, name: &str, optional: bool, elements: &[json_shape::JsonShape]) {
    let representations = elements
        .iter()
        .map(shape_representation)
        .collect::<Vec<_>>()
        .join(", ");
    let target = format!(
        "{}{}({representations}){}",
        if optional { "Option" } else { "" },
        if optional { "<" } else { "" },
        if optional { ">" } else { "" }
    );
    scope.new_type_alias(name, target).vis("pub");
}

fn shape_representation(shape: &JsonShape) -> String {
    match shape {
        JsonShape::Null => "()".to_string(),
        JsonShape::Bool { optional } => {
            if *optional {
                "Option<bool>".to_string()
            } else {
                "bool".to_string()
            }
        }
        JsonShape::Number { optional } => {
            if *optional {
                "Option<f64>".to_string()
            } else {
                "f64".to_string()
            }
        }
        JsonShape::String { optional } => {
            if *optional {
                "Option<String>".to_string()
            } else {
                "String".to_string()
            }
        }
        JsonShape::Array { r#type, optional } => {
            let sub_shape = shape_representation(r#type);
            if *optional {
                format!("Optional<Vec<{sub_shape}>>")
            } else {
                format!("Vec<{sub_shape}>")
            }
        }
        JsonShape::Object {
            content: _,
            optional,
        }
        | JsonShape::OneOf {
            variants: _,
            optional,
        } => {
            let name = shape_name(shape);
            if *optional {
                format!("Option<{name}>")
            } else {
                name
            }
        }
        JsonShape::Tuple { elements, optional } => {
            let sub_shapes = elements
                .iter()
                .map(shape_representation)
                .collect::<Vec<_>>()
                .join(", ");
            if *optional {
                format!("Option<({sub_shapes})>")
            } else {
                format!("({sub_shapes})")
            }
        }
    }
}

fn shape_name(shape: &JsonShape) -> String {
    match shape {
        JsonShape::Null => "Null".to_string(),
        JsonShape::Bool { optional } => {
            if *optional {
                "OptionalBool".to_string()
            } else {
                "Bool".to_string()
            }
        }
        JsonShape::Number { optional } => {
            if *optional {
                "OptionalNumber".to_string()
            } else {
                "Number".to_string()
            }
        }
        JsonShape::String { optional } => {
            if *optional {
                "OptionalStr".to_string()
            } else {
                "Str".to_string()
            }
        }
        JsonShape::Array { r#type, optional } => {
            let sub_shape = shape_name(r#type);
            if *optional {
                format!("OptionalArrayOf{sub_shape}")
            } else {
                format!("ArrayOf{sub_shape}")
            }
        }
        JsonShape::Object { content, optional } => {
            let len = content.len();
            let sub_shapes = content
                .values()
                .map(shape_name)
                .collect::<String>()
                .to_case(convert_case::Case::Pascal);
            let mut crc = Crc32::new();
            crc.update(sub_shapes.as_bytes());
            crc.finalize();
            let name = format!("{:X}", crc.getsum());

            if *optional {
                format!("OptionalStruct{len}Crc{name}")
            } else {
                format!("Struct{len}Crc{name}")
            }
        }
        JsonShape::OneOf { variants, optional } => {
            let len = variants.len();
            let sub_shapes = variants
                .iter()
                .map(shape_name)
                .collect::<String>()
                .to_case(convert_case::Case::Pascal);
            let mut crc = Crc32::new();
            crc.update(sub_shapes.as_bytes());
            crc.finalize();
            let name = format!("{:X}", crc.getsum());
            if *optional {
                format!("OptionalEnum{len}Crc{name}")
            } else {
                format!("Enum{len}Crc{name}")
            }
        }
        JsonShape::Tuple { elements, optional } => {
            let len = elements.len();
            let sub_shapes = elements
                .iter()
                .map(shape_representation)
                .collect::<String>()
                .to_case(convert_case::Case::Pascal);
            let mut crc = Crc32::new();
            crc.update(sub_shapes.as_bytes());
            crc.finalize();
            let name = format!("{:X}", crc.getsum());

            if *optional {
                format!("OptionalTuple{len}Crc{name}")
            } else {
                format!("Tuple{len}Crc{name}")
            }
        }
    }
}
