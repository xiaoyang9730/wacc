use crate::ast_nodes::*;

pub fn gen_tacky_program(c_program: CProgram) -> TackyProgram {
    gen_program(c_program)
}

fn gen_program(c_program: CProgram) -> TackyProgram {
    let c::Program(function_definition) = c_program;
    tacky::Program(gen_function_definition(function_definition))
}

fn gen_function_definition(c_function_definition: CFunctionDefinition) -> TackyFunctionDefinition {
    let c::Function(c::Identifier(name), c::Return(expression)) = c_function_definition;

    let mut instructions = Vec::new();
    let result = emit_tacky(expression, &mut instructions);
    instructions.push(tacky::Return(result));
    tacky::Function(tacky::Identifier(name), instructions)
}

fn emit_tacky(c_expression: CExpression, instructions: &mut Vec<TackyInstruction>) -> TackyValue {
    match c_expression {
        c::Constant(integer) => {
            tacky::Constant(integer)
        },
        c::UnaryOperation(operator, inner) => {
            let src = emit_tacky(*inner, instructions);
            let dst = tacky::Variable(tacky::Identifier(format!("tmp{}", instructions.len())));
            let operator = match operator {
                c::Complement => tacky::Complement,
                c::Negate => tacky::Negate,
            };
            instructions.push(TackyInstruction::Unary(operator, src, dst.clone()));
            dst
        },
    }
}
