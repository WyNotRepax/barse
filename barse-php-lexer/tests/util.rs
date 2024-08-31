use std::{fs, path::PathBuf};

pub fn assert_file_lex_same(file: PathBuf) {
    let content = String::from_utf8(fs::read(file).unwrap()).unwrap();
    assert_lex_same(content);
}

pub fn assert_lex_same(code: impl AsRef<str>) {
    let code = code.as_ref();
    let mut tokens = barse_php_lexer::lexer::lex(code).into_iter();
    let mut native_tokens = barse_php_lexer::native::lex(code).into_iter();
    while let (token @ Some(_), native_token) | (token, native_token @ Some(_)) =
        (tokens.next(), native_tokens.next())
    {
        assert_eq!(token, native_token)
    }
}
