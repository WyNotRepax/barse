use std::{fs, path::PathBuf};

use barse_php_lexer::Token;

pub fn assert_file_lex_same(file: PathBuf) {
    println!("File {}", file.display());
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
        if token != native_token {
            eprintln!("Tokens do not match");
            eprintln!("Expected:");
            eprintln!("{:#?}\n", native_token);
            eprintln!("Got:");
            eprintln!("{:#?}\n", token);
        }
        assert_eq!(token, native_token)
    }
}
