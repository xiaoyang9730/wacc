use std::fmt;

use Keyword::*;
use TokenCheckResult::*;
use TokenCheckerType::*;

#[derive(Default)]
pub struct Lexer {
    src: String,
}

impl Lexer {
    pub fn tokens(&self) -> Tokens {
        if !self.src.is_ascii() {
            unimplemented!("Supporting Non-ASCII characters in source code");
        }
        Tokens::from_src(&self.src)
    }

    pub fn get_src_mut(&mut self) -> &mut String {
        &mut self.src
    }
}

pub struct Tokens<'a> {
    src: &'a str,
}

impl<'a> Tokens<'a> {
    fn from_src(src: &'a str) -> Self {
        Self { src }
    }

    fn next_token_len(&self) -> Result<usize, usize> {
        let mut checker = TokenChecker::default();
        for (i, ch) in self.src.char_indices().peekable() {
            match checker.check(ch) {
                EndChar => { return Ok(i); },
                InvalidChar => { return Err(i); },
                _ => {},
            }
        }
        Ok(self.src.len())
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.src = self.src.trim_ascii_start();
        if self.src.is_empty() {
            return None;
        }

        match self.next_token_len() {
            Ok(len) => {
                let token_str;
                (token_str, self.src) = self.src.split_at(len);
                return Some(Token::from(token_str));
            },
            Err(len) => {
                return Some(Token::Invalid(&self.src[..=len]));
            },
        }
    }
}

#[derive(Default)]
struct TokenChecker {
    type_: Option<TokenCheckerType>,
}

impl TokenChecker {
    fn check(&mut self, ch: char) -> TokenCheckResult {
        // First character
        let Some(type_) = &self.type_ else {
            self.type_ = {
                if ch == '(' || ch == ')' || ch == '{' || ch == '}' || ch == ';' {
                    Some(StartWithSymbol)
                } else if ch.is_ascii_digit() {
                    Some(StartWithDigit)
                } else if ch.is_ascii_alphabetic() || ch == '_' {
                    Some(StartWithAlphabetic)
                } else {
                    return InvalidChar;
                }
            };
            return AcceptableChar;
        };

        // Following characters
        match type_ {
            StartWithSymbol => {
                return EndChar;
            },
            StartWithDigit => {
                if ch.is_ascii_digit() {
                    return AcceptableChar;
                }
                if ch.is_ascii_alphabetic() || ch == '_' {
                    return InvalidChar;
                }
                return EndChar;
            },
            StartWithAlphabetic => {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    return AcceptableChar;
                }
                return EndChar;
            },
        }
    }
}

enum TokenCheckerType {
    StartWithDigit,
    StartWithAlphabetic,
    StartWithSymbol,
}

enum TokenCheckResult {
    AcceptableChar,
    EndChar,
    InvalidChar,
}

#[derive(PartialEq, Eq)]
pub enum Token<'a> {
    Invalid(&'a str),
    Keyword(Keyword),
    Identifier(&'a str),
    Constant(u32),
    Semicolon,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "(" => Self::OpenParenthesis,
            ")" => Self::CloseParenthesis,
            "{" => Self::OpenBrace,
            "}" => Self::CloseBrace,
            ";" => Self::Semicolon,
            value => {
                if let Ok(integer) = value.parse::<u32>() {
                    return Self::Constant(integer);
                }
                if let Ok(keyword) = Keyword::try_from(value) {
                    return Self::Keyword(keyword);
                }
                return Self::Identifier(value);
            },
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(token) => f.write_fmt(format_args!("INVALID TOKEN: `{token}`")),
            Self::Keyword(kw) => f.write_str(&kw.to_string()),
            Self::Identifier(identifier) => f.write_str(identifier),
            Self::Constant(integer) => f.write_fmt(format_args!("{integer}")),
            Self::Semicolon => f.write_str(";"),
            Self::OpenParenthesis => f.write_str("("),
            Self::CloseParenthesis => f.write_str(")"),
            Self::OpenBrace => f.write_str("{"),
            Self::CloseBrace => f.write_str("}"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Keyword {
    CInt,
    CVoid,
    CReturn,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CInt => f.write_str("int"),
            CVoid => f.write_str("void"),
            CReturn => f.write_str("return"),
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "int" => Ok(CInt),
            "void" => Ok(CVoid),
            "return" => Ok(CReturn),
            _ => Err(()),
        }
    }
}
