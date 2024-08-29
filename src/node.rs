use crate::Attributes;
use html_escape::encode_safe as escape;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Node {
    Element(GenericElement),
    VoidElement(VoidElement),
    Text(String),
    RawText(String),
}

impl From<GenericElement> for Node {
    fn from(value: GenericElement) -> Self {
        Self::Element(value)
    }
}

impl From<VoidElement> for Node {
    fn from(value: VoidElement) -> Self {
        Self::VoidElement(value)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Element(element) => write!(f, "{element}"),
            Self::VoidElement(element) => write!(f, "{element}"),
            Self::Text(text) => write!(f, "{}", escape(text)),
            Self::RawText(text) => write!(f, "{text}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericElement {
    pub tag_name: String,
    pub attributes: Attributes,
    pub children: Vec<Node>,
}

impl Display for GenericElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self {
            attributes,
            children,
            tag_name,
        } = &self;
        write!(
            f,
            "<{0}{1}>{2}</{0}>",
            tag_name,
            attributes,
            children.iter().map(ToString::to_string).collect::<String>()
        )
    }
}

#[derive(Debug, Clone)]
pub struct VoidElement {
    pub tag_name: String,
    pub attributes: Attributes,
}

impl Display for VoidElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self {
            attributes,
            tag_name,
        } = &self;
        write!(f, "<{0}{1} />", tag_name, attributes,)
    }
}

pub trait Element: Sized {
    fn attribute(self, key: impl Display, value: impl Display) -> Self;

    // TODO: auto-generate global attributes???

    fn class(self, value: impl Display) -> Self {
        self.attribute("class", value)
    }

    // TODO: add docs?
    fn id(self, value: impl Display) -> Self {
        self.attribute("id", value)
    }
}

pub trait Children: Element {
    fn child<T>(self, child: T) -> Self
    where
        Node: From<T>;

    fn children<T>(mut self, children: impl IntoIterator<Item = T>) -> Self
    where
        Node: From<T>,
    {
        for child in children {
            self = self.child(child);
        }
        self
    }

    fn text(self, text: impl Display) -> Self {
        self.child(Node::Text(text.to_string()))
    }

    fn raw_text(self, text: impl Display) -> Self {
        self.child(Node::RawText(text.to_string()))
    }
}

pub(crate) mod prelude {
    pub use super::{Children, Element, GenericElement, Node, VoidElement};
}
