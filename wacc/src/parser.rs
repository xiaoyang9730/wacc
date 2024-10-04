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

    // TODO: parse_identifier() and parse_expression() have similar code structure
    fn expect_next(&mut self, expected: Token) -> Result<(), String> {
        let next_token = self.tokens.next()
            .unwrap_or(Err(format!("Expect `{expected}` but no tokens left")))?;
        if next_token != expected {
            return Err(format!("Expect `{expected}`, found `{next_token}`"));
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

    // TODO: similar code structure with expect_next
    fn parse_identifier(&mut self) -> Result<CIdentifier, String> {
        let next_token = self.tokens.next()
            .unwrap_or(Err("Expect an identifier but no tokens left".into()))?;
        let Token::Identifier(identifier) = next_token else {
            return Err(format!("Expect an identifier, found `{next_token}`"));
        };
        Ok(c::Identifier(identifier.to_string()))
    }

    fn parse_statement(&mut self) -> Result<CStatement, String> {
        self.expect_next(Token::from("return"))?;
        let expression = self.parse_expression()?;
        self.expect_next(Token::from(";"))?;
        Ok(c::Return(expression))
    }

    // TODO: similar code structure with expect_next
    fn parse_expression(&mut self) -> Result<CExpression, String> {
        let next_token = self.tokens.next()
            .unwrap_or(Err("Expect an expression but no tokens left".into()))?;
        match next_token {
            Token::Constant(integer) => {
                Ok(c::Constant(integer))
            },
            Token::Complement => {
                let inner_expression = Box::new(self.parse_expression()?);
                Ok(c::Unary(c::Complement, inner_expression))
            },
            Token::Negate => {
                let inner_expression = Box::new(self.parse_expression()?);
                Ok(c::Unary(c::Negate, inner_expression))
            },
            Token::OpenParenthesis => {
                let inner_expression = self.parse_expression()?;
                self.expect_next(Token::from(")"))?;
                Ok(inner_expression)
            },
            _ => {
                Err(format!("Malformed expression: `{next_token}`"))
            },
        }
    }
}
