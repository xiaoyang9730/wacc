//! # Module for representing AST nodes conveniently
//!
//! This module is set up as-is.
//! - Import all the contents in this module via `use path/to/ast_nodes::*;`
//! - Use `Asm/C<NodeType>` to represent the type of a node (which are all enums).
//! - Use `asm/c::<NodeType>(data)` to represent the value of a node (which is a variant of these enums).
//! - Module name `ast_node_variants` is brought into scope, which you should not (and actually could not) use directly.
//!   - These 2 modules have the same name and thus ambiguous with each other;
//!   - Their super modules are private, so you cannot specify which `ast_node_variants` to use;
//!   - `ast_node_variants` modules are not marked as `pub(super)` because doing so prevents LSP autocompletion (like `asm/c::Iden...` won't work);

mod asm_nodes;
mod c_nodes;

pub use asm_nodes::*;
pub use c_nodes::*;

pub mod asm {
    pub use super::asm_nodes::ast_node_variants::*;
}

pub mod c {
    pub use super::c_nodes::ast_node_variants::*;
}
