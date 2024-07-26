mod constants;
pub mod lexer;
mod token_name;
use serde::Deserialize;

pub use token_name::TokenName;

pub mod native;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Token {
    Simple(char),
    Complex { content: String, name: TokenName },
}
