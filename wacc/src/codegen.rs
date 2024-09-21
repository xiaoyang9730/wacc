use crate::ast_nodes::*;

pub struct Generator {
    c_program: CProgram,
}

impl Generator {
    pub fn gen(&self) -> AsmProgram {
        gen_program(&self.c_program)
    }
}

impl From<CProgram> for Generator {
    fn from(c_program: CProgram) -> Self {
        Self { c_program }
    }
}

fn gen_program(c_program: &CProgram) -> AsmProgram {
    let c::Program(function_definition) = c_program;
    asm::Program(
        gen_function_definition(function_definition)
    )
}

fn gen_function_definition(c_function_definition: &CFunctionDefinition) -> AsmFunctionDefinition {
    let c::Function(name, body) = c_function_definition;
    asm::Function(
        gen_identifier(name),
        gen_statement(body),
    )
}

fn gen_identifier(c_identifier: &CIdentifier) -> AsmIdentifier {
    let c::Identifier(name) = c_identifier;
    asm::Identifier(name.clone())
}

fn gen_statement(c_statement: &CStatement) -> Vec<AsmInstruction> {
    let c::Return(expression) = c_statement;
    vec![
        asm::Mov(gen_expression(expression), asm::Register),
        asm::Ret,
    ]
}

fn gen_expression(c_expression: &CExpression) -> AsmOperand {
    let &c::Constant(integer) = c_expression;
    asm::Imm(integer)
}
