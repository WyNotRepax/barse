use std::sync::{Mutex, OnceLock};

use barse_php_exec::PhpExec;

use crate::Token;

pub struct Lexer;
impl crate::Lexer for Lexer {
    #[inline(always)]
    fn lex<C: AsRef<str>>(self, code: C) -> Vec<Token> {
        lex(code)
    }
}

static EXEC: OnceLock<Mutex<PhpExec>> = OnceLock::new();
const CODE: &str = include_str!("native_lexer.php");

fn lex_native(code: impl AsRef<str>) -> Vec<Token> {
    let mut exec = EXEC
        .get_or_init(|| Mutex::new(PhpExec::new().unwrap()))
        .lock()
        .unwrap();
    let code = serde_json::to_string(code.as_ref()).unwrap();
    let code = code.replace("$", "\\$");

    let code = CODE[5..].replace("/**CODE**/", &code);
    exec.exec(code).unwrap().to_result().unwrap()
}

pub fn lex(code: impl AsRef<str>) -> Vec<Token> {
    lex_native(code).into_iter().map(Into::into).collect()
}

#[cfg(test)]
mod test {
    use crate::native::lex;

    #[test]
    fn test() {
        dbg!(lex("<?php echo \"Hello World\";"));
    }
}
