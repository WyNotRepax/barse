use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
/// Represents a lexical token in the source code.
///
/// A `Token` consists of a `kind` which indicates the type of the token,
/// and a `span` which specifies the location of the token in the source code.
/// Represents a lexical token with a specific kind and span.
pub struct Token {
    /// The kind of the token, indicating its type.
    pub kind: TokenKind,
    /// The span of the token in the source code, representing its position.
    pub span: Span,
}

impl Token {
    /// Creates a new `Token` instance with the specified `kind` and `span`.
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TokenKind {
    Whitespace,
    Eq,
    Semicolon,
    Variable,
    Number,
    Plus,
}