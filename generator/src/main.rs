use serde::{de::IntoDeserializer, Deserialize};
use std::str::FromStr;
use serde_json::Value;

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
}

fn main() {
    let data =
        include_str!("../../node_modules/@vscode/web-custom-data/data/browsers.html-data.json");

    let data = serde_json::Value::from_str(data).unwrap();
    let data = Vec::<Tag>::deserialize(data["tags"].clone().into_deserializer()).unwrap();
    for item in data {
        dbg!(item.name);
    }
}
