use barse_php_lexer::Span;
use barse_php_lexer::Token as LexerToken;
use barse_php_lexer::TokenKind as LexerTokenKind;

use crate::Parse;
pub struct Token<TokenKind> {
    span: Span,
    data: TokenKind,
}

impl<TokenKind> Token<TokenKind> {
    pub fn new(span: Span, data: TokenKind) -> Self {
        Self { span, data }
    }
}

trait TokenKind {
    const KIND: LexerTokenKind;
    fn parse(code: &str) -> Self;
}

impl<T> Parse for Token<T>
where
    T: TokenKind,
{
    fn parse(code: &str, tokens: &[LexerToken]) -> Option<Self> {
        let next_token = tokens.first()?;
        if next_token.kind == T::KIND {
            let span = next_token.span.clone();
            let token_code = span.fetch(code);
            let data = T::parse(token_code);
            Some(Token { span, data })
        } else {
            None
        }
    }
}

/// # Examples
///
/// ```
/// simple_token!{Whitespace}
/// ```
macro_rules! simple_token {
    ($variant:ident) => {
        pub struct $variant;
        impl TokenKind for $variant {
            const KIND: LexerTokenKind = LexerTokenKind::$variant;
            fn parse(_code: &str) -> Self {
                Self
            }
        }
    };
}

simple_token! {Whitespace}
simple_token! {Eq}
simple_token! {Semicolon}

pub struct Number {
    value: i64,
}

impl TokenKind for Number {
    const KIND: LexerTokenKind = LexerTokenKind::Number;
    fn parse(code: &str) -> Self {
        let _ = code;
        todo!();
    }
}

pub struct Variable {
    pub name: String,
}

impl TokenKind for Variable {
    const KIND: LexerTokenKind = LexerTokenKind::Variable;
    fn parse(code: &str) -> Self {
        Self {
            name: code[1..].to_string(),
        }
    }
}
