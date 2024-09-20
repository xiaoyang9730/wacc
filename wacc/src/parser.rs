use std::process::exit;
use crate::lexer::{Token, Tokens};

// TODO: Try to use struct to construct AST
#[allow(unused)]
#[derive(Debug)]
pub enum AstNode {
    Program(Box<AstNode>),
    Function((String, Box<AstNode>)),
    Return(Box<AstNode>),
    Constant(u32),
}

fn expect(expected: Token, tokens: &mut Tokens) {
    let Some(actual) = tokens.next() else {
        eprintln!("[parser] Expect `{expected}` but no tokens left");
        exit(1);
    };
    if actual != expected {
        eprintln!("[parser] Expect `{expected}`, found {actual}");
        exit(1);
    }
}

pub fn parse_program(tokens: &mut Tokens) -> AstNode {
    let function_definition = Box::new(parse_function_definition(tokens));
    if tokens.next().is_some() {
        eprintln!("[parser] Expect no tokens after function");
        exit(1);
    }
    AstNode::Program(function_definition)
}

fn parse_function_definition(tokens: &mut Tokens) -> AstNode {
    expect(Token::from("int"), tokens);
    // identifier
    let name = parse_identifier(tokens).unwrap_or_else(|| {
        eprintln!("[parser] Expected an identifier for function name");
        exit(1);
    });
    expect(Token::from("("), tokens);
    expect(Token::from("void"), tokens);
    expect(Token::from(")"), tokens);
    expect(Token::from("{"), tokens);
    // statement
    let body = Box::new(parse_statement(tokens));
    expect(Token::from("}"), tokens);
    AstNode::Function((name, body))
}

fn parse_identifier(tokens: &mut Tokens) -> Option<String> {
    let Some(Token::Identifier(identifier)) = tokens.next() else {
        eprintln!("[parser] No tokens left when parsing identifier");
        exit(1);
    };
    // TODO: Try to use &str here
    Some(String::from(identifier))
}

fn parse_statement(tokens: &mut Tokens) -> AstNode {
    expect(Token::from("return"), tokens);
    // expression
    let expression = Box::new(parse_expression(tokens));
    expect(Token::from(";"), tokens);
    AstNode::Return(expression)
}

fn parse_expression(tokens: &mut Tokens) -> AstNode {
    let Some(Token::Constant(integer)) = tokens.next() else {
        eprintln!("[parser] No tokens left when parsing expression");
        exit(1);
    };
    AstNode::Constant(integer)
}
