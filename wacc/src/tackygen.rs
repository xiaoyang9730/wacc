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
    let (mut instructions, operand) = gen_expression(expression);
    instructions.push(tacky::Return(operand));
    tacky::Function(tacky::Identifier(name), instructions)
}

fn gen_expression(c_expression: CExpression) -> (Vec<TackyInstruction>, TackyOperand) {
    match c_expression {
        c::Constant(integer) => {
            (vec![], tacky::Constant(integer))
        },
        c::Unary(operator, inner) => {
            let (mut instructions, src) = gen_expression(*inner);
            let dst = tacky::Variable(tacky::Identifier(format!("tmp{}", instructions.len())));
            instructions.push(TackyInstruction::Unary(gen_unary_operator(operator), src, dst.clone()));
            (instructions, dst)
        },
    }
}

fn gen_unary_operator(c_operator: CUnaryOperator) -> TackyUnaryOperator {
    match c_operator {
        c::Complement => tacky::Complement,
        c::Negate => tacky::Negate,
    }
}
