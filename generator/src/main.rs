use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use serde::{de::IntoDeserializer, Deserialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Deserialize)]
struct Description {
    kind: String,
    value: String,
}

#[derive(Deserialize)]
struct Reference {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct Attribute {
    name: String,
    description: Option<Value>,
}

#[derive(Deserialize)]
struct Tag {
    name: String,
    description: Value,
    attributes: Vec<Attribute>,
    references: Vec<Reference>,
    void: Option<bool>,
}

impl Tag {
    fn void(&self) -> bool {
        self.void.unwrap_or_default()
    }

    fn tag_name(&self) -> Ident {
        format_ident!("{}", self.name)
    }

    fn name(&self) -> Ident {
        format_ident!("{}", self.name.to_upper_camel_case())
    }
}

fn main() {
    let data =
        include_str!("../../node_modules/@vscode/web-custom-data/data/browsers.html-data.json");

    let data = serde_json::Value::from_str(data).unwrap();

    let data = Vec::<Tag>::deserialize(data["tags"].clone().into_deserializer()).unwrap();

    let data = data.into_iter().map(|value| {
        let name = value.name();
        let tag_name = value.tag_name();
        let element = if value.void() {
            quote! { void_element_struct!(#name, #tag_name); }
        } else {
            quote! { element_struct!(#name, #tag_name); }
        };
        let attributes = value
            .attributes
            .iter()
            .unique_by(|attribute| attribute.name.clone())
            .map(|attribute| {
                let attribute_name = &attribute.name;
                let method_name = format_ident!(
                    "{}",
                    if matches!(
                        attribute.name.as_str(),
                        "type" | "as" | "for" | "loop" | "async"
                    ) {
                        format!("r#{}", attribute.name)
                    } else {
                        attribute_name.to_snake_case()
                    }
                );
                quote! {
                    element_attribute!(#name, #method_name, #attribute_name);
                }
            });
        quote! {
            #element
            #(#attributes)*
        }
    });

    let data = quote! {
        use crate::{element_struct, void_element_struct, element_attribute};
        use crate::node::prelude::*;

        #(#data)*
    };

    std::fs::write(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../src/gen.rs"),
        data.to_string(),
    )
    .unwrap();
}
