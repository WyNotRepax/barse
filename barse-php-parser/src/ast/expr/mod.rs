use super::{
    stmt::Stmt, ArgOrVariadicPlaceholder, ArrayItem, AttributeGroup, ClosureUse,
    ExprOrInterpolatedStringPart, Identifier, IdentifierOrExpr, IdentifierOrName, MatchArm, Name,
    NameOrExpr, NameOrExprOrClass, Node, Param, Type_,
};

pub mod binary_op;
pub mod cast;

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

pub use cast::Cast;

pub struct ClassConstFetch {
    pub class: NameOrExpr,
    pub name: IdentifierOrName,
}

pub struct Clone {
    pub expr: Expr,
}

pub struct Closure {
    pub static_: bool,
    pub by_ref: bool,
    pub params: Vec<Param>,
    pub uses: Vec<ClosureUse>,
    pub return_type: Type_,
    pub stmts: Vec<Stmt>,
    pub attr_groups: Vec<AttributeGroup>,
}

pub struct ConstFetch {
    pub name: Name,
}

pub struct Empty {
    pub name: Expr,
}

pub struct Eval {
    pub expr: Expr,
}

pub struct Exit {
    pub expr: Option<Expr>,
    pub exit_kind: ExitKind,
}

pub enum ExitKind {
    Exit,
    Die,
}

pub struct FuncCall {
    pub name: NameOrExpr,
    pub args: Vec<ArgOrVariadicPlaceholder>,
}

pub struct Include {
    pub type_: IncludeType,
    pub expr: Expr,
}

pub enum IncludeType {
    Include,
    IncludeOnce,
    Require,
    RequireOnce,
}

pub struct InstanceOf {
    pub expr: Expr,
    pub class: NameOrExpr,
}

pub struct Isset {
    pub vars: Vec<Expr>,
}

pub struct List {
    pub kind: ListKind,
    pub items: Vec<Option<ArrayItem>>,
}

pub enum ListKind {
    List,
    Array,
}

pub struct Match {
    pub cond: Expr,
    pub arms: Vec<MatchArm>,
}

pub struct MethodCall {
    pub var: Expr,
    pub name: IdentifierOrExpr,
}

pub struct New {
    pub class: NameOrExprOrClass,
    pub args: Vec<ArgOrVariadicPlaceholder>,
}

pub struct NullsafeMethodCall {
    pub var: Expr,
    pub name: IdentifierOrExpr,
    pub args: Vec<ArgOrVariadicPlaceholder>,
}

pub struct NullsafePropertyFetch {
    pub var: Expr,
    pub name: IdentifierOrExpr,
}

pub struct PostDec {
    pub var: Expr,
}

pub struct PostInc {
    pub var: Expr,
}

pub struct PreDec {
    pub var: Expr,
}

pub struct PreInc {
    pub var: Expr,
}

pub struct Print {
    pub expr: Expr,
}

pub struct PropertyFetch {
    pub var: Expr,
    pub name: IdentifierOrExpr,
}

pub struct ShellExec {
    pub parts: ExprOrInterpolatedStringPart,
}

pub struct StaticCall {
    
}
pub struct Variable {}
