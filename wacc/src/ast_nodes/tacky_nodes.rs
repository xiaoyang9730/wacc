pub mod ast_node_variants {
    pub use super::TackyProgram::*;
    pub use super::TackyFunctionDefinition::*;
    pub use super::TackyIdentifier::*;
    pub use super::TackyInstruction::*;
    pub use super::TackyValue::*;
    pub use super::TackyUnaryOperator::*;
}

#[derive(Debug)]
pub enum TackyProgram {
    Program(TackyFunctionDefinition),
}

#[derive(Debug)]
pub enum TackyFunctionDefinition {
    Function(TackyIdentifier, Vec<TackyInstruction>)
}

#[derive(Debug, Clone)]
pub enum TackyIdentifier {
    Identifier(String),
}

#[derive(Debug)]
pub enum TackyInstruction {
    Return(TackyValue),
    Unary(TackyUnaryOperator, TackyValue, TackyValue),
}

#[derive(Debug, Clone)]
pub enum TackyValue {
    Constant(u32),
    Variable(TackyIdentifier),
}

#[derive(Debug)]
pub enum TackyUnaryOperator {
    Complement,
    Negate,
}
