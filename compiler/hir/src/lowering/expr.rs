use std::fmt::Debug;

use ast::expression::Builtin as SourceBuiltin;

use crate::lowering::de_bruijn::Context;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(usize),
    Builtin(Builtin),
    Ref(Ref),
    Pi {
        typ: Box<Self>,
        body: Box<Self>,
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
    pub fn construct_pi(typ: Self, body: Self) -> Self {
        Self::Pi {
            typ: Box::new(typ),
            body: Box::new(body),
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsRef {
    pub ind: usize,
    pub cons: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Prop,
    Type(u32),
}

impl Builtin {
    pub fn from_source<'a>(builtin_type: &'a SourceBuiltin, _: &mut Context) -> Self {
        match builtin_type {
            SourceBuiltin::Prop => Self::Prop,
            SourceBuiltin::Type(u) => Self::Type(*u),
        }
    }
}
