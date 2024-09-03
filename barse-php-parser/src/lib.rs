pub mod ast;

pub mod lexer {
    pub use barse_php_lexer::Lexer as LexerTrait;
    pub type Lexer = barse_php_lexer::native::Lexer;
}
