use super::{Builtin, global_ref::GlobalRef};

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
