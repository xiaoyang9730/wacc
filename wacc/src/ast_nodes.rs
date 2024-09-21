#![allow(unused)]

mod asm_nodes;
mod c_nodes;

pub use asm_nodes::*;
pub use c_nodes::*;

pub mod asm {
    pub use super::asm_nodes::variants::*;
}

pub mod c {
    pub use super::c_nodes::variants::*;
}
