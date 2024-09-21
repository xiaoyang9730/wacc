use std::process::exit;
use crate::lexer::{Token, Tokens};
use crate::ast_nodes::*;

pub struct Parser<'a> {
    tokens: Tokens<'a>,
}

impl<'a> From<Tokens<'a>> for Parser<'a> {
    fn from(tokens: Tokens<'a>) -> Self {
        Self { tokens }
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> CProgram {
        self.parse_program()
    }

    fn expect_next(&mut self, expected: Token) {
        let Some(actual) = self.tokens.next() else {
            eprintln!("[parser] Expect `{expected}` but no tokens left");
            exit(1);
        };
        if actual != expected {
            eprintln!("[parser] Expect `{expected}`, found {actual}");
            exit(1);
        }
    }

    fn parse_program(&mut self) -> CProgram {
        let function_definition = self.parse_function_definition();
        if self.tokens.next().is_some() {
            eprintln!("[parser] Expect no tokens after function");
            exit(1);
        }
        c::Program(function_definition)
    }

    fn parse_function_definition(&mut self) -> CFunctionDefinition {
        self.expect_next(Token::from("int"));
        let name = self.parse_identifier();
        self.expect_next(Token::from("("));
        self.expect_next(Token::from("void"));
        self.expect_next(Token::from(")"));
        self.expect_next(Token::from("{"));
        let statement = self.parse_statement();
        self.expect_next(Token::from("}"));
        c::Function(name, statement)
    }

    fn parse_identifier(&mut self) -> CIdentifier {
        let Some(Token::Identifier(identifier)) = self.tokens.next() else {
            eprintln!("[parser] No tokens left when parsing identifier");
            exit(1);
        };
        c::Identifier(identifier.to_string())
    }

    fn parse_statement(&mut self) -> CStatement {
        self.expect_next(Token::from("return"));
        let expression = self.parse_exxpression();
        self.expect_next(Token::from(";"));
        c::Return(expression)
    }

    fn parse_exxpression(&mut self) -> CExpression {
        let Some(Token::Constant(integer)) = self.tokens.next() else {
            eprintln!("[parser] No tokens left when parsing expression");
            exit(1);
        };
        c::Constant(integer)
    }
}
