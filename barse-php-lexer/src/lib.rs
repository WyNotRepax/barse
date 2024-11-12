#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
//! A lexer for PHP code.

mod lexer;
mod span;
mod token;
mod token_stream;

pub fn lex(code: impl ToString) -> token_stream::TokenStream {
    lexer::Lexer::new(code.to_string()).lex()
}

#[cfg(test)]
mod test {

    #[test]
    fn test_lexer_1() {
        use crate::token::TokenKind;

        let code = r#"
            $a = 1;
        "#;

        let tokens = super::lex(code);
        dbg!(tokens
            .peek_n_kinds([
                TokenKind::Whitespace,
                TokenKind::Variable,
                TokenKind::Whitespace,
                TokenKind::Eq,
                TokenKind::Whitespace,
                TokenKind::Number,
                TokenKind::Semicolon,
            ])
            .unwrap());
    }
}
