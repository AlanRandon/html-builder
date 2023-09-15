pub use attributes::Attributes;
pub use node::prelude::*;

mod attributes;
mod gen;
mod macros;
mod node;

pub mod prelude {
    // TODO: make structs private
    pub use super::gen::*;
    pub use super::node::prelude::*;
}

pub mod html {
    pub use super::node::prelude::*;
}
