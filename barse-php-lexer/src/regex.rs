use std::fmt::format;

use regex::{Regex, RegexBuilder};

use crate::TokenName;

pub fn lex(code: impl AsRef<str>) {
    let code = code.as_ref();

    // let next_token = build_regex();
    // next_token.find(code)
    // let len = code.find(&next_token).unwrap();
}
trait TokenPattern {
    fn pattern() -> String;
    fn name() -> TokenName;
    #[cfg(test)]
    const TESTS: &'static [&'static str];
}

#[cfg(test)]
fn test_token<T: TokenPattern>() {
    let regex = RegexBuilder::new(&format!("^{}$", T::pattern()))
        .case_insensitive(true)
        .build()
        .unwrap();
    for test in T::TESTS {
        assert!(regex.is_match(test));
    }
}
#[test]
fn test_build_regex() {
    println!("{}", build_regex());
}

fn build_regex() -> String {
    use TokenName::*;
    format!(
        "^(?:{})",
        [
            keyword_pattern(IncludeOnce, "include_once"),
            keyword_pattern(Include, "include"),
            keyword_pattern(RequireOnce, "require_once"),
            keyword_pattern(Require, "require"),
            keyword_pattern(Eval, "eval"),
            keyword_pattern(LogicalOr, "or"),
            keyword_pattern(LogicalAnd, "and"),
            keyword_pattern(LogicalXor, "xor"),
            whitespace_pattern(),
            "(?<T0>.)".to_string()
        ]
        .join("|")
    )
}

macro_rules! kw {
    ($name:ident, $keyword:literal) => {
        $name => keyword_pattern($name, $keyword)
    };
}
fn keyword_pattern(name: TokenName, keyword: &str) -> String {
    format!("(?:{}(?:[^\\w]|$))", token_pattern(name, keyword))
}

fn token_pattern(name: TokenName, pattern: &str) -> String {
    format!("(?<T{}>{pattern})", name as u32)
}

fn whitespace_pattern() -> String {
    token_pattern(TokenName::Whitespace, "\\s+")
}
