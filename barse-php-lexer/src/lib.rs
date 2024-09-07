use std::ops::Range;

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Clone)]
pub struct Span {
    range: Range<usize>,
}

impl Span {
    pub fn new(start: usize, len: usize) -> Self {
        Self {
            range: start..start + len,
        }
    }

    pub fn fetch<'a>(&self, code: &'a str) -> &'a str {
        &code[self.range.clone()]
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self { range }
    }
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Whitespace,
    Eq,
    Semicolon,
    Variable,
    Number,
}

pub fn lex(code: &str) -> Vec<Token> {
    let mut offset = 0;
    let mut tokens = Vec::new();
    while offset < code.len() {
        tokens.push(lex_next(&mut offset, code));
    }
    tokens
}

pub fn lex_next(offset: &mut usize, code: &str) -> Token {
    fn lex_next_inner(offset: &mut usize, code: &str) -> Option<Token> {
        lex_whitespace(offset, code)?;
        lex_eq(offset, code)?;
        lex_semicolon(offset, code)?;
        lex_variable(offset, code)?;
        lex_number(offset, code)?;
        None
    }
    lex_next_inner(offset, code).unwrap()
}

fn lex_whitespace(offset: &mut usize, code: &str) -> Option<Token> {
    let len: usize = code[*offset..]
        .chars()
        .take_while(char::is_ascii_whitespace)
        .map(char::len_utf8)
        .sum();
    if len > 0 {
        let token = Some(Token {
            kind: TokenKind::Whitespace,
            span: Span::new(*offset, len),
        });
        *offset += len;
        token
    } else {
        None
    }
}

macro_rules! lex_char {
    ($name:ident($c:literal){..}) => {
        
    };
}

fn lex_eq(offset: &mut usize, code: &str) -> Option<Token> {
    if let Some('=') = code[*offset..].chars().next() {
        let token = Token::new(TokenKind::Eq, Span::new(*offset, 1));
        *offset += 1;
        Some(token)
    } else {
        None
    }
}
fn lex_semicolon(offset: &mut usize, code: &str) -> Option<Token> {
    todo!()
}
fn lex_variable(offset: &mut usize, code: &str) -> Option<Token> {
    todo!()
}
fn lex_number(offset: &mut usize, code: &str) -> Option<Token> {
    todo!()
}
