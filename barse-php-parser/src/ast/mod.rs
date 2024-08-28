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
    pub attrs: Vec<Attribute>,
}

pub struct Attribute {
    pub name: Name,
    pub args: Vec<Arg>,
}

pub struct Name {
    pub name: String,
    pub kind: NameKind,
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
    pub identifier: Option<Identifier>,
    pub value: Expr,
    pub by_ref: bool,
    pub unpack: bool,
}

pub struct Identifier {
    pub name: String,
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
    pub types: Vec<Type_>,
}
pub struct UnionType {
    pub types: Vec<Type_>,
}
pub struct NullableType {
    pub type_: Type_,
}
