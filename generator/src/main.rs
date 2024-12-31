// TODO: use better data source

use heck::{ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use serde::{de::IntoDeserializer, Deserialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(Deserialize)]
#[serde(transparent)]
struct Description(Value);

impl Description {
    fn as_str(&self) -> Option<&str> {
        self.0
            .get("value")
            .and_then(|value| value.as_str())
            .or_else(|| self.0.as_str())
    }
}

#[derive(Deserialize)]
struct Reference {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct Attribute {
    name: String,
    description: Option<Description>,
}

#[derive(Deserialize)]
struct Tag {
    name: String,
    description: Description,
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
    let data = std::fs::read_to_string(
        "node_modules/@vscode/web-custom-data/data/browsers.html-data.json",
    )
    .unwrap();

    let data = serde_json::Value::from_str(&data).unwrap();

    let data = Vec::<Tag>::deserialize(data["tags"].clone().into_deserializer()).unwrap();

    let data = data.into_iter().map(|value| {
        let name = value.name();
        let tag_name = value.tag_name();
        let doc = value.description.as_str().unwrap_or_default();

        let element = if value.void() {
            quote! { void_element_struct!(#name, #tag_name, #doc); }
        } else {
            quote! { element_struct!(#name, #tag_name, #doc); }
        };

        let attributes = value
            .attributes
            .iter()
            .unique_by(|attribute| attribute.name.clone())
            .map(|attribute| {
                let attribute_name = &attribute.name;
                let doc = attribute
                    .description
                    .as_ref()
                    .and_then(|description| description.as_str())
                    .unwrap_or_default();

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
                    element_attribute!(#name, #method_name, #attribute_name, #doc);
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
