use super::Expr;

pub struct BinaryOp {
    pub left: Expr,
    pub right: Expr,
    pub kind: BinaryOpKind,
}

pub enum BinaryOpKind {
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    
    LogicalAnd,
    LogicalOr,
    LogicalXor,

    BooleanAnd,
    BooleanOr,

    Coalesce,
    Equal,
    NotEqual,
    Identical,
    NotIdentical,
    Greater,
    GreaterOrEqual,
    Smaller,
    SmallerOrEqual,
    Spaceship,
    
    Concat,
    Mod,
    Minus,
    Div,
    Mul,
    Plus,
    Pow,
    ShiftLeft,
    ShiftRight,    
}

