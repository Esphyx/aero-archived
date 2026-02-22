use crate::compiler::front_end::kernel::de_bruijn::context::DeBruijnContext;

use super::{
    super::super::grammar::ast::builtin::{SourceBuiltin, SourceBuiltinType, SourcePrimitive},
    term::Term,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Type(BuiltinType),
    Primitive(Primitive),
}

impl Builtin {
    pub fn from_source<'a>(
        builtin_source: &'a SourceBuiltin,
        context: &mut DeBruijnContext,
    ) -> Self {
        match builtin_source {
            SourceBuiltin::Type(source_builtin_type) => {
                Builtin::Type(BuiltinType::from_source(source_builtin_type, context))
            }
            SourceBuiltin::Primitive(source_primitive) => {
                Builtin::Primitive(Primitive::from_source(source_primitive))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    ReadU8,
    WriteU8,
    SuccU8,
    // SubU8,
    ZeroU8,
    ElimU8,
    // Exit,
}

impl Primitive {
    pub fn from_source<'a>(primitive: &'a SourcePrimitive) -> Self {
        match primitive {
            SourcePrimitive::ReadU8 => Self::ReadU8,
            SourcePrimitive::WriteU8 => Self::WriteU8,
            SourcePrimitive::ZeroU8 => Self::ZeroU8,
            SourcePrimitive::SuccU8 => Self::SuccU8,
            SourcePrimitive::ElimU8 => Self::ElimU8,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinType {
    Prop,
    Type(u32),
    U8,
    Bool,
    Unit,
    Array { dependent: Box<Term> },
}

impl BuiltinType {
    pub fn from_source<'a>(
        builtin_type: &'a SourceBuiltinType,
        context: &mut DeBruijnContext,
    ) -> Self {
        match builtin_type {
            SourceBuiltinType::Prop => Self::Prop,
            SourceBuiltinType::Type(u) => Self::Type(*u),
            SourceBuiltinType::U8 => Self::U8,
            SourceBuiltinType::Unit => Self::Unit,
            SourceBuiltinType::Array { dependent } => Self::Array {
                dependent: Box::new(context.convert_term(&dependent)),
            },
        }
    }
}
