use ast::builtin::{SourceBuiltin, SourceBuiltinType, SourcePrimitive};

use crate::lowering::de_bruijn::DeBruijnContext;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Var(usize),
    Builtin(Builtin),
    GlobalRef(GlobalRef),
    Pi {
        from_type: Box<Term>,
        to_type: Box<Term>,
    },
    Lambda {
        typ: Box<Option<Term>>,
        body: Box<Term>,
    },
    App {
        function: Box<Term>,
        argument: Box<Term>,
    },
}

impl Term {
    pub fn construct_pi(from: Self, to: Self) -> Self {
        Self::Pi {
            from_type: Box::new(from),
            to_type: Box::new(to),
        }
    }

    pub fn construct_binding(typ: Option<Self>, body: Self) -> Self {
        Self::Lambda {
            typ: Box::new(typ),
            body: Box::new(body),
        }
    }

    pub fn construct_application(function: Self, argument: Self) -> Self {
        Self::App {
            function: Box::new(function),
            argument: Box::new(argument),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalRef {
    ConstRef(usize),
    InductiveRef(usize),
    ConstructorRef {
        inductive: usize,
        constructor: usize,
    },
    EliminatorRef(usize),
    SelfRef(usize),
}

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
    ZeroU8,
    ElimU8,
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
