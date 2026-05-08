use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BinderId {
    pub id: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var {
        index: usize,
        binder: Option<BinderId>,
    },
    Builtin(Builtin),
    Ref(Ref),
    Pi {
        binder: Option<BinderId>,
        typ: Box<Self>,
        body: Box<Self>,
    },
    Lambda {
        binder: Option<BinderId>,
        typ: Box<Self>,
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
    pub fn construct_pi(binder: Option<BinderId>, typ: Self, body: Self) -> Self {
        Self::Pi {
            binder,
            typ: Box::new(typ),
            body: Box::new(body),
        }
    }

    pub fn construct_lambda(binder: Option<BinderId>, typ: Self, body: Self) -> Self {
        Self::Lambda {
            binder,
            typ: Box::new(typ),
            body: Box::new(body),
        }
    }

    pub fn construct_application(func: Self, arg: Self) -> Self {
        Self::App {
            func: Box::new(func),
            arg: Box::new(arg),
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
