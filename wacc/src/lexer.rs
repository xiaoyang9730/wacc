use std::process::exit;

pub struct Lexer<'a> {
    src: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    pub fn next_token(&mut self) -> Option<&'a str> {
        self.src = self.src.trim_start();
        if self.src.is_empty() {
            return None;
        }
        
        let mut char_indices = self.src.char_indices().peekable();
        let (_, ch0) = char_indices.next()?;

        if ch0 == '(' || ch0 == ')' || ch0 == '{' || ch0 == '}' || ch0 == ';' {
            let token;
            (token, self.src) = self.src.split_at(ch0.len_utf8());
            return Some(token);
        }

        if ch0.is_ascii_digit() {
            while let Some(&(i, ch)) = char_indices.peek() {
                if ch.is_ascii_digit() {
                    char_indices.next();
                } else if ch.is_ascii_alphabetic() {
                    eprintln!("[lexer] Unrecognized token");
                    exit(1);
                } else {
                    let token;
                    (token, self.src) = self.src.split_at(i);
                    return Some(token);
                }
            }

            let token;
            (token, self.src) = self.src.split_at(ch0.len_utf8());
            return Some(token);
        }

        if ch0.is_ascii_alphabetic() || ch0 == '_' {
            while let Some(&(i, ch)) = char_indices.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    char_indices.next();
                } else {
                    let token;
                    (token, self.src) = self.src.split_at(i);
                    return Some(token);
                }
            }

            let token;
            (token, self.src) = self.src.split_at(ch0.len_utf8());
            return Some(token);
        }

        eprintln!("[lexer] Unrecognized token");
        exit(1);
    }
}
