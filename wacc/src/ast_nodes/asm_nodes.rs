pub(super) mod variants {
    pub use super::AsmProgram::*;
    pub use super::AsmFunctionDefinition::*;
    pub use super::AsmIdentifier::*;
    pub use super::AsmInstruction::*;
    pub use super::AsmOperand::*;
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
    Ret,
}

#[derive(Debug)]
pub enum AsmOperand {
    Imm(u32),
    Register,
}
