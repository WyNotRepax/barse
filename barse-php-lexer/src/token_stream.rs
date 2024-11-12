use std::array;

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn peek_n<const N: usize>(&self) -> Option<[&Token; N]> {
        if N <= self.remaining() {
            Some(array::from_fn(|i| &self.tokens[self.pos + i]))
        } else {
            None
        }
    }
    pub fn peek_n_kinds<const N: usize>(&self, kinds: [TokenKind; N]) -> Option<[&Token; N]> {
        if let Some(tokens) = self.peek_n::<N>() {
            for i in 0..N {
                if kinds[i] != tokens[i].kind {
                    return None;
                }
            }
            Some(tokens)
        } else {
            None
        }
    }

    pub fn remaining(&self) -> usize {
        self.tokens.len() - self.pos
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Self {
        Self::new(FromIterator::from_iter(iter))
    }
}
