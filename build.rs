use std::{env, path::PathBuf};

use bindgen::{callbacks::{ParseCallbacks, TypeKind}, EnumVariation};
use convert_case::{Case, Casing};

pub trait ParserStrExt {
    fn strip_type_suffix(&self) -> &Self;
}

impl ParserStrExt for str {
    fn strip_type_suffix(&self) -> &Self {
        self.strip_suffix("_t").unwrap_or(self)
    }
}

#[derive(Debug)]
struct CallbackParser;

impl ParseCallbacks for CallbackParser {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        Some(original_item_name.strip_suffix("_t").unwrap_or(original_item_name).to_case(Case::UpperCamel))
    }

    fn add_derives(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        match (info.name, info.kind) {
            (_, TypeKind::Enum) => {
                vec!["strum::AsRefStr".into(), "strum::IntoStaticStr".into()]
            }
            _ => vec![],
        }
    }

    fn add_attributes(&self, info: &bindgen::callbacks::AttributeInfo<'_>) -> Vec<String> {
        match (info.name, info.kind) {
            (_, TypeKind::Enum) => {
                vec![
                    r#"#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]"#.into(),
                    r#"#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]"#.into(),
                ]
            }
            _ => vec![],
        }
    }    

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        if let Some(enum_name) = enum_name {
            if enum_name.ends_with("_t") {
                let prefix = enum_name.strip_suffix('t').unwrap();
                return Some(
                    original_variant_name
                        .strip_prefix(prefix)
                        .unwrap_or(original_variant_name)
                        .to_case(Case::UpperCamel),
                );
            }
        }
        None
    }    
} 

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=example.h");

    let bindings = bindgen::Builder::default()
    .header("example.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .default_enum_style(EnumVariation::Rust {
        non_exhaustive: true,
    })
    .parse_callbacks(Box::new(CallbackParser))
    .allowlist_type("color_t")
    .generate()

    .expect("Unable to generate bindings");

let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
bindings
    .write_to_file(out_path.join("src/bindings.rs"))
    .expect("Couldn't write bindings!");
}