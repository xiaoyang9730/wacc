pub mod ast_node_variants {
    pub use super::AsmProgram::*;
    pub use super::AsmFunctionDefinition::*;
    pub use super::AsmIdentifier::*;
    pub use super::AsmInstruction::*;
    pub use super::AsmUnaryOperator::*;
    pub use super::AsmOperand::*;
    pub use super::AsmReg::*;
}

#[derive(Debug)]
pub enum AsmProgram {
    Program(AsmFunctionDefinition),
}

#[derive(Debug)]
pub enum AsmFunctionDefinition {
    Function(AsmIdentifier, Vec<AsmInstruction>),
}

#[derive(Debug)]
pub enum AsmIdentifier {
    Identifier(String),
}

#[derive(Debug)]
pub enum AsmInstruction {
    Mov(AsmOperand, AsmOperand),
    Unary(AsmUnaryOperator, AsmOperand),
    AllocateStack(u32),
    Ret,
}

#[derive(Debug)]
pub enum AsmUnaryOperator {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum AsmOperand {
    Imm(u32),
    Register(AsmReg),
    Pseudo(AsmIdentifier),
    Stack(u32),
}

#[derive(Debug)]
pub enum AsmReg {
    AX,
    R10,
}
