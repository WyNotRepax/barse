use std::ops::Range;

pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    fn len(&self) -> usize {
        self.span.range.len()
    }
}

pub struct Span {
    range: Range<usize>,
}
impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self { range }
    }
}

impl Span {
    pub fn new(from: usize, to: usize) -> Self {
        Self { range: from..to }
    }
}

pub enum TokenKind {
    Whitespace,
    Variable,
    Eq,
}

pub fn lex(code: impl AsRef<str>) -> Result<Vec<Token>, (LexerError, Vec<Token>)> {
    let code = code.as_ref();
    let mut tokens = Vec::new();
    let mut pos = 0;
    while pos < tokens.len() {
        match lex_next(code, &mut pos) {
            Err(err) => return Err((err, tokens)),
            Ok(token) => {
                tokens.push(token);
            }
        }
    }

    Ok(tokens)
}

pub enum LexerError {
    UnknownToken,
}


fn lex_next(code: &str, pos: &mut usize) -> Result<Token, LexerError> {
    for f in [lex_whitespace, lex_variable] {
        if let Some(token) = f(code, pos) {
            return Ok(token);
        }
    }
    return Err(LexerError::UnknownToken);
}

fn lex_whitespace(code: &str, pos: &mut usize) -> Option<Token> {
    let len: usize = code[*pos..]
        .chars()
        .take_while(char::is_ascii_whitespace)
        .map(char::len_utf8)
        .sum();
    if len > 0 {
        let span = Span::new(*pos, *pos + len);
        *pos += len;
        Some(Token::new(TokenKind::Whitespace, span))
    } else {
        None
    }
}

fn lex_variable(code: &str, pos: &mut usize) -> Option<Token> {
}
