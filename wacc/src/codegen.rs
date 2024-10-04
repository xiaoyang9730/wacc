use std::collections::HashMap;
use crate::ast_nodes::*;

pub fn gen_asm_program(tacky_program: TackyProgram) -> AsmProgram {
    let mut asm_program = gen_program(tacky_program);
    assign_pseudo_registers_to_stack(&mut asm_program);
    fix_invalid_mov_instructions(&mut asm_program);
    asm_program
}

fn gen_program(tacky_program: TackyProgram) -> AsmProgram {
    let tacky::Program(function_definition) = tacky_program;
    asm::Program(gen_function_definition(function_definition))
}

fn gen_function_definition(tacky_function_definition: TackyFunctionDefinition) -> AsmFunctionDefinition {
    let tacky::Function(tacky::Identifier(name), tacky_instructions) = tacky_function_definition;
    let mut asm_instructions = Vec::new();
    for instruction in tacky_instructions {
        match instruction {
            tacky::Return(val) => {
                asm_instructions.push(asm::Mov(gen_operand(val), asm::Register(asm::AX)));
                asm_instructions.push(AsmInstruction::Ret);
            },
            TackyInstruction::Unary(operator, src, dst) => {
                asm_instructions.push(asm::Mov(gen_operand(src), gen_operand(dst.clone())));
                asm_instructions.push(asm::Unary(gen_unary_operator(operator), gen_operand(dst)));
            },
        }
    }
    asm::Function(asm::Identifier(name), asm_instructions)
}

fn gen_operand(tacky_value: TackyOperand) -> AsmOperand {
    match tacky_value {
        tacky::Constant(integer) => {
            asm::Imm(integer)
        },
        tacky::Variable(identifier) => {
            let tacky::Identifier(name) = identifier;
            asm::Pseudo(asm::Identifier(name))
        }
    }
}

fn gen_unary_operator(tacky_operator: TackyUnaryOperator) -> AsmUnaryOperator {
    match tacky_operator {
        tacky::Complement => asm::Not,
        tacky::Negate => asm::Neg,
    }
}

fn assign_pseudo_registers_to_stack(asm_program: &mut AsmProgram) {
    let asm::Program(asm::Function(_, instructions)) = asm_program;
    let mut stack_map = HashMap::new();
    for instruction in instructions.iter_mut() {
        match instruction {
            asm::Mov(src, dst) => {
                check_and_replace_pseudo_register(src, &mut stack_map);
                check_and_replace_pseudo_register(dst, &mut stack_map);
            },
            asm::Unary(_, dst) => {
                check_and_replace_pseudo_register(dst, &mut stack_map);
            },
            _ => {},
        }
    }
    instructions.insert(0, asm::AllocateStack(stack_map.len() as u32));
}

fn check_and_replace_pseudo_register(asm_operand: &mut AsmOperand, stack_map: &mut HashMap<String, u32>) {
    if let asm::Pseudo(asm::Identifier(name)) = asm_operand {
        if !stack_map.contains_key(name) {
            stack_map.insert(name.clone(), stack_map.len() as u32);
        }
        *asm_operand = asm::Stack(*stack_map.get(name).unwrap());
    }
}

fn fix_invalid_mov_instructions(asm_program: &mut AsmProgram) {
    let asm::Program(asm::Function(_, instructions)) = asm_program;
    let mut list = Vec::new();
    for (fix_pos, instruction) in instructions.iter().enumerate() {
        if let &asm::Mov(asm::Stack(a_src), asm::Stack(a_dst)) = instruction {
            list.push((fix_pos, a_src, a_dst));
        }
    }
    for (i, (fix_pos, a_src, a_dst)) in list.iter().enumerate() {
        instructions[fix_pos + i] = asm::Mov(asm::Stack(*a_src), asm::Register(asm::R10));
        instructions.insert(fix_pos + i + 1, asm::Mov(asm::Register(asm::R10), asm::Stack(*a_dst)));
    }
}
