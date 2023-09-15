#[macro_export]
macro_rules! element_struct {
    ($name:ident, $tag_name:ident) => {
        pub struct $name {
            element: GenericElement,
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
                    .insert(key.to_string(), value.to_string());
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

        pub fn $tag_name() -> $name {
            $name {
                element: GenericElement {
                    tag_name: stringify!($tag_name).to_string(),
                    attributes: $crate::Attributes::default(),
                    children: Vec::new(),
                },
            }
        }
    };
}

#[macro_export]
macro_rules! void_element_struct {
    ($name:ident, $tag_name:ident) => {
        pub struct $name {
            element: VoidElement,
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
                    .insert(key.to_string(), value.to_string());
                self
            }
        }

        pub fn $tag_name() -> $name {
            $name {
                element: VoidElement {
                    tag_name: stringify!($tag_name).to_string(),
                    attributes: $crate::Attributes::default(),
                },
            }
        }
    };
}

#[macro_export]
macro_rules! element_attribute {
    ($element_name:ident, $method_name:ident, $html_name:literal) => {
        impl $element_name {
            pub fn $method_name(self, value: impl std::fmt::Display) -> Self {
                self.attribute($html_name, value)
            }
        }
    };
}
