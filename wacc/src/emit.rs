use crate::ast_nodes::*;

pub fn emit_asm_program(asm_program: AsmProgram) -> String {
    let asm::Program(function_definition) = asm_program;
    let mut asm_code = emit_asm_function_definition(function_definition);
    asm_code.push_str("\n\t.section .note.GNU-stack,\"\",@progbits");
    asm_code
}

fn emit_asm_function_definition(function_definition: AsmFunctionDefinition) -> String {
    let asm::Function(asm::Identifier(name), instructions) = function_definition;
    let mut asm_code = String::new();
    asm_code.push_str(&format!("\t.globl {name}\n"));
    asm_code.push_str(&format!("{name}:\n"));
    asm_code.push_str(&format!("\tpushq\t%rbp\n"));
    asm_code.push_str(&format!("\tmovq\t%rsp, %rbp\n"));
    for instruction in emit_asm_instructions(instructions).lines() {
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
                    format!("movl\t{src}, {dst}\n")
                },
                asm::Ret => {
                    let epilogue = "movq\t%rbp, %rsp\npopq\t%rbp";
                    format!("{epilogue}\nret\n")
                },
                asm::Unary(operator, operand) => {
                    let operator = match operator {
                        asm::Neg => "negl",
                        asm::Not => "notl",
                    };
                    let operand = emit_asm_operand(operand);
                    format!("{operator}\t{operand}\n")
                },
                asm::AllocateStack(integer) => format!("subq\t${}, %rsp\n", integer * 4),
            }
        })
        .collect()
}

fn emit_asm_operand(operand: AsmOperand) -> String {
    match operand {
        asm::Register(asm::AX) => "%eax".into(),
        asm::Register(asm::R10) => "%r10d".into(),
        asm::Stack(integer) => format!("-{}(%rbp)", (integer + 1) * 4),
        asm::Imm(integer) => format!("${integer}"),
        _ => panic!("Unsupported asm operand"),
    }
}
