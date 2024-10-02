use std::fmt;

use Keyword::*;
use TokenSyntaxCheckResult::*;
use TokenSyntaxCheckerType::*;

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
        let mut tsc = TokenSyntaxChecker::default();
        for ch in self.src.chars() {
            match tsc.check(ch) {
                TokenEnd => { return Ok(tsc.len as _); },
                TokenInvalid => { return Err(tsc.len as _); },
                _ => {},
            }
        }
        Ok(self.src.len())
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.src = self.src.trim_ascii_start();
        if self.src.is_empty() {
            return None;
        }

        match self.next_token_len() {
            Ok(len) => {
                let token_str;
                (token_str, self.src) = self.src.split_at(len);
                return Some(Ok(Token::from(token_str)));
            },
            Err(len) => {
                return Some(Err(format!("Invalid token: `{}`", &self.src[..=len])));
            },
        }
    }
}

struct TokenSyntaxChecker {
    type_: Option<TokenSyntaxCheckerType>,
    len: isize,
}

impl TokenSyntaxChecker {
    fn check(&mut self, ch: char) -> TokenSyntaxCheckResult {
        self.len += 1;

        // First character
        let Some(type_) = &self.type_ else {
            self.type_ = {
                if ch == '(' || ch == ')' || ch == '{' || ch == '}' || ch == ';' || ch == '~' || ch == '-' {
                    Some(StartWithSymbol(ch))
                } else if ch.is_ascii_digit() {
                    Some(StartWithDigit)
                } else if ch.is_ascii_alphabetic() || ch == '_' {
                    Some(StartWithAlphabetic)
                } else {
                    return TokenInvalid;
                }
            };
            return TokenAcceptable;
        };

        // Following characters
        match type_ {
            StartWithSymbol(start_symbol) => {
                match start_symbol {
                    '-' => {
                        if self.len == 1 && ch == '-' {
                            return TokenAcceptable;
                        } else {
                            return TokenEnd;
                        }
                    },
                    _ => {
                        return TokenEnd;
                    },
                }
            },
            StartWithDigit => {
                if ch.is_ascii_digit() {
                    return TokenAcceptable;
                }
                if ch.is_ascii_alphabetic() || ch == '_' {
                    return TokenInvalid;
                }
                return TokenEnd;
            },
            StartWithAlphabetic => {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    return TokenAcceptable;
                }
                return TokenEnd;
            },
        }
    }
}

impl Default for TokenSyntaxChecker {
    fn default() -> Self {
        Self { type_: None, len: -1 }
    }
}

enum TokenSyntaxCheckerType {
    StartWithSymbol(char),
    StartWithDigit,
    StartWithAlphabetic,
}

enum TokenSyntaxCheckResult {
    TokenAcceptable,
    TokenEnd,
    TokenInvalid,
}

#[derive(PartialEq, Eq)]
pub enum Token<'a> {
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
