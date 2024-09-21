pub(super) mod variants {
    pub use super::CProgram::*;
    pub use super::CFunctionDefinition::*;
    pub use super::CIdentifier::*;
    pub use super::CStatement::*;
    pub use super::CExpression::*;
}

#[derive(Debug)]
pub enum CProgram {
    Program(CFunctionDefinition),
}

#[derive(Debug)]
pub enum CFunctionDefinition {
    Function(CIdentifier, CStatement),
}

#[derive(Debug)]
pub enum CIdentifier {
    Identifier(String),
}

#[derive(Debug)]
pub enum CStatement {
    Return(CExpression),
}

#[derive(Debug)]
pub enum CExpression {
    Constant(u32)
}
