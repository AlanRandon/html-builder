use html_escape::encode_safe as escape;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Default)]
pub struct Attributes(pub HashMap<String, String>);

impl Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        write!(
            f,
            " {}",
            self.0
                .iter()
                .map(|(key, value)| (escape(key), escape(value)))
                .map(|(key, value)| format!(r#"{key}="{value}""#))
                .join(" ")
        )
    }
}
