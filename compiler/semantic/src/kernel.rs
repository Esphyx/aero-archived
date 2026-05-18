use hir::lowering::{
    expr::{ConsRef, Expr, Ref},
    namespace::Namespace,
};

use crate::traversal::{collect_spine, substitute};

#[derive(Debug, Clone)]
pub enum ReductionKind {
    Beta,
    Delta,
    Iota,
}

#[derive(Debug, Clone)]
pub struct ReductionStep {
    kind: ReductionKind,
    before: Expr,
    after: Expr,
}

impl ReductionStep {
    pub fn new(kind: ReductionKind, before: Expr, after: Expr) -> Self {
        Self {
            kind,
            before,
            after,
        }
    }
}

pub struct EvalResult {
    pub value: Expr,
    pub steps: Vec<ReductionStep>,
}

impl EvalResult {
    pub fn pure(value: Expr) -> Self {
        Self {
            value,
            steps: Vec::new(),
        }
    }

    pub fn with_step(mut self, step: ReductionStep) -> Self {
        self.steps.push(step);
        self
    }

    pub fn extend(mut self, other: EvalResult) -> Self {
        self.steps.extend(other.steps);
        self
    }

    pub fn then(mut self, other: EvalResult) -> Self {
        self.steps.extend(other.steps);
        self.value = other.value;
        self
    }
}

pub struct Kernel<'a> {
    pub namespace: &'a Namespace,
}

impl<'a> Kernel<'a> {
    pub fn from(namespace: &'a Namespace) -> Self {
        Self { namespace }
    }

    pub fn whnf_step(&self, term: &Expr) -> EvalResult {
        match term {
            Expr::App { func, arg } => self.beta(func, arg),
            Expr::Match {
                scrutinee,
                branches,
            } => self.iota(scrutinee, branches),
            Expr::Ref(g) => self.delta(&g),
            _ => EvalResult {
                value: term.clone(),
                steps: Vec::new(),
            },
        }
    }

    fn beta(&self, func: &Expr, arg: &Expr) -> EvalResult {
        let function_res = self.whnf_step(func);

        let before = Expr::construct_application(func.clone(), arg.clone());

        match function_res.value {
            Expr::Lambda { body, .. } => {
                let reduced = substitute(&body, arg, 0);
                let result = self.whnf_step(&reduced);

                let mut steps = Vec::new();

                steps.push(ReductionStep::new(
                    ReductionKind::Beta,
                    before,
                    result.value.clone(),
                ));

                steps.extend(result.steps);

                EvalResult {
                    value: result.value,
                    steps,
                }
            }
            _ => EvalResult {
                value: before,
                steps: Vec::new(),
            },
        }
    }

    fn iota(&self, scrutinee: &Expr, branches: &[(ConsRef, Expr)]) -> EvalResult {
        let scrutinee_reduced = self.whnf_step(scrutinee);

        let before = scrutinee_reduced.value.clone();

        let (head, args) = collect_spine(&scrutinee_reduced.value);

        let Expr::Ref(Ref::Cons(cons_ref)) = head else {
            return EvalResult::pure(Expr::construct_match(
                scrutinee.clone(),
                branches.to_owned(),
            ))
            .extend(scrutinee_reduced);
        };

        let (_, branch_body) = branches
            .iter()
            .find(|(g, _)| *g == cons_ref)
            .expect("Missing branch!");

        let applied = args.into_iter().fold(branch_body.clone(), |acc, arg| {
            Expr::construct_application(acc, arg)
        });

        let result = self.whnf_step(&applied);
        let value = result.value.clone();

        scrutinee_reduced.then(result).with_step(ReductionStep::new(
            ReductionKind::Iota,
            before,
            value,
        ))
    }

    fn delta(&self, r: &Ref) -> EvalResult {
        let before = Expr::Ref(r.clone());

        let after = match r {
            Ref::Fn(index) => {
                let definition = self.namespace.functions[*index].definition.clone();
                definition
            }
            Ref::Ind(ind) => Expr::Ref(Ref::Ind(*ind)),
            Ref::Cons(cr) => Expr::Ref(Ref::Cons(cr.clone())),
        };

        EvalResult {
            value: after.clone(),
            steps: vec![ReductionStep::new(ReductionKind::Delta, before, after)],
        }
    }
}
