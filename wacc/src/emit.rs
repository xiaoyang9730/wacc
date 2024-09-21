use crate::ast_nodes::*;

pub fn emit_asm_program(asm_program: AsmProgram) -> String {
    let asm::Program(function_definition) = asm_program;
    let mut asm_code = emit_asm_function_definition(function_definition);
    asm_code.push_str("\n\t.section .note.GNU-stack,\"\",@progbits");
    asm_code
}

fn emit_asm_function_definition(function_definition: AsmFunctionDefinition) -> String {
    let asm::Function(asm::Identifier(name), instructions) = function_definition;
    let instructions = emit_asm_instructions(instructions);
    let mut asm_code = String::new();
    asm_code.push_str(&format!("\t.globl {name}\n"));
    asm_code.push_str(&format!("{name}:\n"));
    for instruction in instructions.lines() {
        asm_code.push_str(&format!("\t{instruction}\n"));
    }
    asm_code
}

fn emit_asm_instructions(instructions: Vec<AsmInstruction>) -> String {
    instructions
        .into_iter()
        .map(|instruction| {
            match instruction {
                asm::Mov(src, dst) => {
                    let src = emit_asm_operand(src);
                    let dst = emit_asm_operand(dst);
                    format!("movl {src}, {dst}\n")
                },
                asm::Ret => format!("ret\n"),
            }
        })
        .collect()
}

fn emit_asm_operand(operand: AsmOperand) -> String {
    match operand {
        asm::Register => format!("%eax"),
        asm::Imm(integer) => format!("${integer}"),
    }
}
