use std::fmt::Debug;

use ast::builtin::{
    Builtin as SourceBuiltin, BuiltinType as SourceBuiltinType, Primitive as SourcePrimitive,
};

use crate::lowering::{de_bruijn::DeBruijnContext, program::Program};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(usize),
    Builtin(Builtin),
    Ref(Ref),
    Pi {
        from_type: Box<Self>,
        to_type: Box<Self>,
    },
    Lambda {
        typ: Box<Option<Self>>,
        body: Box<Self>,
    },
    App {
        func: Box<Self>,
        arg: Box<Self>,
    },
    Match {
        scrutinee: Box<Self>,
        branches: Vec<(ConsRef, Self)>,
    },
}

impl Expr {
    pub fn to_string(&self, program: &Program) -> String {
        match self {
            Expr::Var(index) => format!("#{}", index),
            Expr::Builtin(b) => b.to_string(),
            Expr::Ref(r) => r.to_string(program),
            Expr::Pi { from_type, to_type } => format!(
                "(Π _ : {} -> {})",
                from_type.to_string(program),
                to_type.to_string(program)
            ),
            Expr::Lambda { typ, body } => match typ.as_ref() {
                Some(t) => format!("λ: {}. {}", t.to_string(program), body.to_string(program)),
                None => format!("λ. {}", body.to_string(program)),
            },
            Expr::App {
                func: function,
                arg: argument,
            } => format!(
                "({}, {})",
                function.to_string(program),
                argument.to_string(program)
            ),
            Expr::Match {
                scrutinee,
                branches,
            } => {
                format!(
                    "(match {} with {})",
                    scrutinee.to_string(program),
                    branches
                        .iter()
                        .map(|(ctor, body)| {
                            format!("{} := {}", ctor.to_string(program), body.to_string(program))
                                .to_string()
                        })
                        .collect::<Vec<_>>()
                        .join("|")
                )
            }
        }
    }

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
            func: Box::new(function),
            arg: Box::new(argument),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ref {
    Fn(usize),
    Ind(usize),
    Cons(ConsRef),
    // SelfRef(usize),
}

impl Ref {
    pub fn to_string(&self, program: &Program) -> String {
        match self {
            Ref::Fn(i) => program.namespace.functions[*i]
                .name
                .get_name_str()
                .to_string(),
            Ref::Ind(i) => program.namespace.inductives[*i]
                .name
                .get_name_str()
                .to_string(),
            Ref::Cons(r) => r.to_string(program),
            // Ref::SelfRef(i) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsRef {
    pub inductive: usize,
    pub constructor: usize,
}

impl ConsRef {
    pub fn to_string(&self, program: &Program) -> String {
        program.namespace.inductives[self.inductive].constructors[self.constructor]
            .name
            .get_name_str()
            .to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Type(BuiltinType),
    Primitive(Primitive),
}

impl Builtin {
    pub fn to_string(&self) -> String {
        match self {
            Builtin::Type(builtin_type) => builtin_type.to_string(),
            Builtin::Primitive(primitive) => primitive.to_string(),
        }
    }

    pub fn from_source<'a>(builtin: &'a SourceBuiltin, context: &mut DeBruijnContext) -> Self {
        match builtin {
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
    pub fn to_string(&self) -> String {
        match self {
            Primitive::ReadU8 => "read_u8",
            Primitive::WriteU8 => "write_u8",
            Primitive::SuccU8 => "succ_u8",
            Primitive::ZeroU8 => "zero_u8",
            Primitive::ElimU8 => "elim_u8",
        }
        .to_string()
    }

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
    SelfType,
    Unit,
}

impl BuiltinType {
    pub fn to_string(&self) -> String {
        match self {
            BuiltinType::Prop => "Prop".to_string(),
            BuiltinType::Type(i) => format!("Type {}", i),
            BuiltinType::SelfType => "SelfType".to_string(),
            BuiltinType::Unit => "Unit".to_string(),
        }
    }

    pub fn from_source<'a>(
        builtin_type: &'a SourceBuiltinType,
        context: &mut DeBruijnContext,
    ) -> Self {
        match builtin_type {
            SourceBuiltinType::Prop => Self::Prop,
            SourceBuiltinType::Type(u) => Self::Type(*u),
            SourceBuiltinType::Unit => Self::Unit,
            SourceBuiltinType::SelfType => Self::SelfType,
        }
    }
}
