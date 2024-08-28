pub mod lexer;
mod token_name;

#[cfg(feature = "native")]
pub mod native;

pub use token_name::TokenName;

pub trait Lexer {
    fn lex<C: AsRef<str>>(self, code: C) -> Vec<Token>;
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "native", derive(serde::Deserialize))]
#[cfg_attr(feature = "native", serde(rename_all = "camelCase"))]
pub enum Token {
    Simple(char),
    Complex { content: String, name: TokenName },
}


impl Token {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        match self {
            Self::Simple(c) => c.len_utf8(),
            Self::Complex { content, .. } => content.len(),
        }
    }
}

impl Token {
    pub fn is_complex_named(&self, name: TokenName) -> bool {
        match self {
            Self::Complex {
                name: self_name, ..
            } => name == *self_name,
            _ => false,
        }
    }

    pub fn is_simple(&self, c: char) -> bool {
        match self {
            Self::Simple(self_c) => c == *self_c,
            _ => false,
        }
    }
}
