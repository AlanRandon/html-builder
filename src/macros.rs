#[macro_export]
macro_rules! element_struct {
    ($name:ident, $tag_name:ident, $doc:literal) => {
        #[allow(clippy::empty_docs)]
        #[doc = $doc]
        #[derive(Debug, Clone)]
        pub struct $name {
            element: GenericElement,
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.element)
            }
        }

        impl From<$name> for Node {
            fn from(value: $name) -> Self {
                Self::from(value.element)
            }
        }

        impl Element for $name {
            fn attribute(
                mut self,
                key: impl std::fmt::Display,
                value: impl std::fmt::Display,
            ) -> Self {
                self.element
                    .attributes
                    .0
                    .entry(key.to_string())
                    .and_modify(|entry| {
                        if key.to_string() == "class" {
                            entry.push_str(&format!(" {value}"))
                        } else {
                            *entry = value.to_string()
                        }
                    })
                    .or_insert_with(|| value.to_string());
                self
            }

            fn remove_attribute(mut self, key: impl std::fmt::Display) -> Self {
                self.element.attributes.0.remove(&key.to_string());
                self
            }
        }

        impl Children for $name {
            fn child<T>(mut self, child: T) -> Self
            where
                Node: From<T>,
            {
                self.element.children.push(child.into());
                self
            }
        }

        impl $name {
            #[allow(dead_code)]
            fn new_empty() -> Self {
                $name {
                    element: GenericElement {
                        tag_name: stringify!($tag_name).to_string(),
                        attributes: $crate::Attributes::default(),
                        children: Vec::new(),
                    },
                }
            }
        }
    };
}

#[macro_export]
macro_rules! void_element_struct {
    ($name:ident, $tag_name:ident, $doc:literal) => {
        #[doc = $doc]
        pub struct $name {
            element: VoidElement,
        }

        impl $name {
            #[allow(dead_code)]
            fn new_empty() -> Self {
                Self {
                    element: VoidElement {
                        attributes: $crate::Attributes::default(),
                        tag_name: stringify!($tag_name).to_string(),
                    },
                }
            }
        }

        impl From<$name> for Node {
            fn from(value: $name) -> Self {
                Self::from(value.element)
            }
        }

        impl Element for $name {
            fn attribute(
                mut self,
                key: impl std::fmt::Display,
                value: impl std::fmt::Display,
            ) -> Self {
                self.element
                    .attributes
                    .0
                    .entry(key.to_string())
                    .and_modify(|entry| {
                        if key.to_string() == "class" {
                            entry.push_str(&format!(" {value}"))
                        } else {
                            *entry = value.to_string()
                        }
                    })
                    .or_insert_with(|| value.to_string());
                self
            }

            fn remove_attribute(mut self, key: impl std::fmt::Display) -> Self {
                self.element.attributes.0.remove(&key.to_string());
                self
            }
        }
    };
}

#[macro_export]
macro_rules! element_attribute {
    ($element_name:ident, $method_name:ident, $html_name:literal, $doc:literal) => {
        impl $element_name {
            #[doc = $doc]
            #[allow(clippy::empty_docs)]
            pub fn $method_name(self, value: impl std::fmt::Display) -> Self {
                self.attribute($html_name, value)
            }
        }
    };
}

#[macro_export]
macro_rules! element_boolean_attribute {
    ($element_name:ident, $method_name:ident, $set_method_name:ident, $html_name:literal, $doc:literal) => {
        impl $element_name {
            #[doc = $doc]
            #[allow(clippy::empty_docs)]
            pub fn $method_name(self) -> Self {
                self.attribute($html_name, $html_name)
            }

            pub fn $set_method_name(self, value: bool) -> Self {
                if value {
                    self.$method_name()
                } else {
                    self.remove_attribute($html_name)
                }
            }
        }
    };
}
