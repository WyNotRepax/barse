use bitflags::bitflags;
use expr::{Expr, Variable};

pub mod expr;
pub trait Node {}

pub struct ArrayItem {
    pub key: Option<Expr>,
    pub value: Expr,
    pub by_ref: bool,
    pub unpack: bool,
}

pub struct Param {
    pub param_type: Type_,
    pub by_ref: bool,
    pub variadic: bool,
    pub var: Variable,
    pub default: Option<Expr>,
    pub flags: VisibilityFlag,
    pub attr_groups: Vec<AttributeGroup>,
}

bitflags! {
pub struct VisibilityFlag: u8 {
    const PUBLIC    =  1;
    const PROTECTED =  2;
    const PRIVATE   =  4;
    const STATIC    =  8;
    const ABSTRACT  = 16;
    const FINAL     = 32;
    const READONLY  = 64;
}
}

pub enum Type_ {
    None,
    Identifier(Box<Identifier>),
    Name(Box<Name>),
    ComplexType(Box<ComplexType>),
}

pub struct AttributeGroup {
    attrs: Vec<Attribute>,
}

pub struct Attribute {
    name: Name,
    args: Vec<Arg>,
}

pub struct Name {
    name: String,
    kind: NameKind,
}

pub enum NameKind {
    Special(SpecialName),
    FullyQualified,
    Relative,
}

pub enum SpecialName {
    Self_,
    Parent,
    Static,
}

pub struct Arg {
    identifier: Option<Identifier>,
    value: Expr,
    by_ref: bool,
    unpack: bool,
}

pub struct Identifier {
    name: String,
}

pub enum IdentifierOrName {
    Name(Name),
    Identifier(Identifier),
}

pub enum ComplexType {
    IntersectionType(IntersectionType),
    UnionType(IntersectionType),
    NullableType(NullableType),
}
pub struct IntersectionType {
    types: Vec<Type_>,
}
pub struct UnionType {
    types: Vec<Type_>,
}
pub struct NullableType {
    type_: Type_,
}

