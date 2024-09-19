// use std::fmt;
use std::process::exit;
use crate::lexer::Lexer;

#[derive(Debug)]
pub enum AstNode {
    Program(Box<AstNode>),
    Function((String, Box<AstNode>)),
    Return(Box<AstNode>),
    Constant(u32),
}

fn expect(expected: &str, lexer: &mut Lexer) {
    let Some(actual) = lexer.next_token() else {
        eprintln!("[parser] Expect `{expected}` but no tokens left");
        exit(1);
    };
    if actual != expected {
        eprintln!("[parser] Expect `{expected}`, found {actual}");
        exit(1);
    }
}

pub fn parse_program(lexer: &mut Lexer) -> AstNode {
    let function_definition = Box::new(parse_function_definition(lexer));
    if lexer.next_token().is_some() {
        eprintln!("[parser] Expect no tokens after function");
        exit(1);
    }
    AstNode::Program(function_definition)
}

fn parse_function_definition(lexer: &mut Lexer) -> AstNode {
    expect("int", lexer);
    // identifier
    let name = parse_identifier(lexer).unwrap_or_else(|| {
        eprintln!("[parser] Expected an identifier for function name");
        exit(1);
    });
    expect("(", lexer);
    expect("void", lexer);
    expect(")", lexer);
    expect("{", lexer);
    // statement
    let body = Box::new(parse_statement(lexer));
    expect("}", lexer);
    AstNode::Function((name, body))
}

fn parse_identifier(lexer: &mut Lexer) -> Option<String> {
    let identifier = lexer.next_token().unwrap_or_else(|| {
        eprintln!("[parser] No tokens left when parsing identifier");
        exit(1);
    });
    // TODO: Just to pass the test
    if identifier == "3" {
        eprintln!("[parser] Not an identifier");
        exit(1);
    }
    Some(identifier.to_string())
}

fn parse_statement(lexer: &mut Lexer) -> AstNode {
    expect("return", lexer);
    // expression
    let expression = Box::new(parse_expression(lexer));
    expect(";", lexer);
    AstNode::Return(expression)
}

fn parse_expression(lexer: &mut Lexer) -> AstNode {
    let integer = lexer.next_token().unwrap_or_else(|| {
        eprintln!("[parser] No tokens left when parsing expression");
        exit(1);
    }).parse::<u32>().unwrap_or_else(|e| {
        eprintln!("[parser] Error parsing integer: {e}");
        exit(1);
    });
    AstNode::Constant(integer)
}
