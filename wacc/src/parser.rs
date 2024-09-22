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
    pub fn parse(&mut self) -> Result<CProgram, String> {
        self.parse_program()
    }

    fn expect_next(&mut self, expected: Token) -> Result<(), String> {
        let Some(actual) = self.tokens.next() else {
            return Err(format!("Expect `{expected}` but no tokens left"));
        };
        if actual != expected {
            return Err(format!("Expect `{expected}`, found `{actual}`"));
        }
        Ok(())
    }

    fn parse_program(&mut self) -> Result<CProgram, String> {
        let function_definition = self.parse_function_definition()?;
        if self.tokens.next().is_some() {
            return Err("Expect no tokens after function".into());
        }
        Ok(c::Program(function_definition))
    }

    fn parse_function_definition(&mut self) -> Result<CFunctionDefinition, String> {
        self.expect_next(Token::from("int"))?;
        let name = self.parse_identifier()?;
        self.expect_next(Token::from("("))?;
        self.expect_next(Token::from("void"))?;
        self.expect_next(Token::from(")"))?;
        self.expect_next(Token::from("{"))?;
        let statement = self.parse_statement()?;
        self.expect_next(Token::from("}"))?;
        Ok(c::Function(name, statement))
    }

    fn parse_identifier(&mut self) -> Result<CIdentifier, String> {
        let Some(Token::Identifier(identifier)) = self.tokens.next() else {
            return Err("No tokens left when parsing identifier".into());
        };
        Ok(c::Identifier(identifier.to_string()))
    }

    fn parse_statement(&mut self) -> Result<CStatement, String> {
        self.expect_next(Token::from("return"))?;
        let expression = self.parse_exxpression()?;
        self.expect_next(Token::from(";"))?;
        Ok(c::Return(expression))
    }

    fn parse_exxpression(&mut self) -> Result<CExpression, String> {
        let Some(Token::Constant(integer)) = self.tokens.next() else {
            return Err("No tokens left when parsing expression".into());
        };
        Ok(c::Constant(integer))
    }
}
