use super::token::{Eq, Token, Variable, Whitespace};

pub enum Expr {
    BinOp(BinOp),
}

pub enum BinOp {
    Assign(Box<Assign>),
}

pub struct Assign {
    pub variable: Token<Variable>,
    pub whitespace1: Option<Token<Whitespace>>,
    pub eq: Token<Eq>,
    pub whitespace2: Option<Token<Whitespace>>,
    pub value: Expr,
}
