pub mod ast;

use barse_php_lexer::Token as LexerToken;

pub use barse_php_lexer::Span as Span;

pub trait Parse: Sized {
    fn parse(code: &str, tokens: &[LexerToken]) -> Option<Self>;
}
