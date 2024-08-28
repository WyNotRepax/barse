use super::{ArrayItem, Param, Type_};

pub mod binary_op;

pub enum Expr {
    ArrayDimFetch(Box<ArrayDimFetch>),
}

pub struct ArrayDimFetch {
    pub var: Expr,
    pub dim: Option<Expr>,
}

pub struct Array {
    pub kind: ArrayKind,
    pub items: Vec<ArrayItem>,
}

pub enum ArrayKind {
    /**
     * array()
     */
    Long,
    /**
     * []
     */
    Short,
}

pub struct ArrowFunction {
    pub static_: bool,
    pub by_ref: bool,
    pub params: Vec<Param>,
    pub return_type: Type_,
}

pub struct Assign {
    pub var: Expr,
    pub expr: Expr,
}

pub struct AssignOp {
    pub var: Expr,
    pub expr: Expr,
}

pub struct AssignRef {
    pub var: Expr,
    pub expr: Expr,
}

pub use binary_op::BinaryOp;

pub struct BitwiseNot {
    pub expr: Expr,
}

pub struct BooleanNot {
    pub expr: Expr,
}

pub struct Variable {}
