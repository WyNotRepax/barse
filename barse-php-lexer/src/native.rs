use std::sync::{Mutex, OnceLock};

use barse_php_exec::PhpExec;
use serde::Deserialize;

use crate::{Token, TokenName};

const EXEC: OnceLock<Mutex<PhpExec>> = OnceLock::new();

fn lex_native(code: impl AsRef<str>) -> Vec<NativeToken> {
    let exec = EXEC;
    let mut exec = exec
        .get_or_init(|| Mutex::new(PhpExec::new().unwrap()))
        .lock()
        .unwrap();
    let code = serde_json::to_string(code.as_ref()).unwrap();
    let code = code.replace("$", "\\$");
    println!("{code}");
    dbg!(exec.exec(dbg!(format!("token_get_all({code})"))))
        .unwrap()
        .to_result()
        .unwrap()
}

pub fn lex(code: impl AsRef<str>) -> Vec<Token> {
    lex_native(code).into_iter().map(Into::into).collect()
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum NativeToken {
    Simple(char),
    Complex(TokenName, String, u32),
}

impl From<NativeToken> for Token {
    fn from(value: NativeToken) -> Self {
        match value {
            NativeToken::Simple(c) => Token::Simple(c),
            NativeToken::Complex(name, content, _) => Token::Complex { content, name },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::native::lex;

    #[test]
    fn test() {
        dbg!(lex("<?php echo \"Hello World\";"));
    }
}
