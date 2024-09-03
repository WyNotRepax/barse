use super::Expr;

pub struct Cast {
    pub cast_type: CastType,
    pub expr: Expr,
}

pub enum CastType {
    Array,
    Bool,
    Double(DoubleCastKind),
    Int,
    Object,
    String,
    Unset,
}

pub enum DoubleCastKind {
    Double,
    Float,
    Real,
}
