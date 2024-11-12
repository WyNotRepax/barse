use std::str::Chars;

use crate::{
    span::Span,
    token::{Token, TokenKind},
    token_stream::TokenStream,
};

#[derive(Debug)]
pub struct Lexer {
    code: String,
    offset: usize,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self { code, offset: 0 }
    }

    pub fn lex(mut self) -> TokenStream {
        let mut tokens = Vec::new();
        while self.offset < self.code.len() {
            tokens.push(self.lex_next());
        }
        TokenStream::new(tokens)
    }

    fn next_str(&self) -> &str {
        &self.code[self.offset..]
    }

    fn next_chars(&self) -> Chars<'_> {
        self.next_str().chars()
    }

    fn eat(&mut self, len: usize) -> Span {
        let span = Span::new(self.offset, len);
        self.offset += len;
        span
    }

    fn lex_next(&mut self) -> Token {
        macro_rules! lex {
            ($f:ident) => {
                if let Some(token) = self.$f() {
                    return token;
                }
            };
        }
        lex!(lex_whitespace);
        lex!(lex_eq);
        lex!(lex_semicolon);
        lex!(lex_variable);
        lex!(lex_number);
        lex!(lex_plus);
        panic!("No token found at offset {}", self.offset);
    }

    fn lex_whitespace(&mut self) -> Option<Token> {
        let len: usize = self
            .next_chars()
            .take_while(char::is_ascii_whitespace)
            .map(char::len_utf8)
            .sum();
        if len > 0 {
            Some(Token {
                kind: TokenKind::Whitespace,
                span: self.eat(len),
            })
        } else {
            None
        }
    }

    fn lex_variable(&mut self) -> Option<Token> {
        if let Some('$') = self.next_chars().next() {
            let len: usize = self
                .next_chars()
                .skip(1) // Skip the '$' character
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .map(char::len_utf8)
                .sum::<usize>()
                + 1; // Include the '$' character

            if len > 1 {
                Some(Token {
                    kind: TokenKind::Variable,
                    span: self.eat(len),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn lex_number(&mut self) -> Option<Token> {
        let len = self
            .next_chars()
            .take_while(char::is_ascii_digit)
            .map(char::len_utf8)
            .sum();
        if len > 0 {
            Some(Token {
                kind: TokenKind::Number,
                span: self.eat(len),
            })
        } else {
            None
        }
    }
}
macro_rules! impl_lex_char {
    ($fname:ident, $kind:ident, $char:literal) => {
        impl Lexer {
            fn $fname(&mut self) -> Option<Token> {
                if let Some($char) = self.next_chars().next() {
                    Some(Token::new(TokenKind::$kind, self.eat(1)))
                } else {
                    None
                }
            }
        }
    };
}

impl_lex_char!(lex_eq, Eq, '=');
impl_lex_char!(lex_semicolon, Semicolon, ';');
impl_lex_char!(lex_plus, Plus, '+');

