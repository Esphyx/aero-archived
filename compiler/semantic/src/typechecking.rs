use hir::lowering::{
    expr::{Expr, Ref},
    namespace::Namespace,
};

use crate::{
    context::Context, kernel::Kernel, positivity::check_inductive_definition, traversal::substitute,
};

pub struct TypeChecker<'a> {
    pub namespace: &'a Namespace,
    pub kernel: &'a Kernel<'a>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(namespace: &'a Namespace, kernel: &'a Kernel<'a>) -> Self {
        Self { namespace, kernel }
    }

    pub fn check_namespace(&self) {
        for (ind_index, inductive) in self.namespace.inductives.iter().enumerate() {
            for cons in inductive.constructors.iter() {
                check_inductive_definition(ind_index, &cons.typ);
            }
        }

        for func in self.namespace.functions.iter() {
            let mut ctx = Context::new();
            self.check_type(&func.definition, &func.return_type, &mut ctx);
        }
    }

    fn infer_type(&self, expr: &Expr, ctx: &mut Context) -> Expr {
        match expr {
            Expr::Var { index, .. } => ctx.lookup(*index).clone(),
            Expr::Ref(r) => self.infer_global(r),
            Expr::App { func, arg } => {
                let func_ty = self.infer_type(&func, ctx);
                let func_ty = self.kernel.whnf_step(&func_ty).value;

                match func_ty {
                    Expr::Pi { body, .. } => substitute(&body, &arg, 0),
                    _ => panic!("Expected pi type {:?}", func_ty),
                }
            }
            Expr::Lambda { binder, typ, body } => {
                let param_ty = *typ.clone();
                ctx.extend(param_ty.clone());

                let body_ty = self.infer_type(&body, ctx);
                ctx.pop();

                Expr::construct_pi(*binder, param_ty, body_ty)
            }
            Expr::Match { .. } => todo!(),
            _ => todo!(),
        }
    }

    fn infer_global(&self, r: &Ref) -> Expr {
        match r {
            Ref::Fn(i) => self.namespace.functions[*i].return_type.clone(),
            Ref::Ind(i) => self.namespace.inductives[*i].typ.clone(),
            Ref::Cons(cons_ref) => self.namespace.inductives[cons_ref.ind].constructors
                [cons_ref.cons]
                .typ
                .clone(),
        }
    }

    fn check_type(&self, expr: &Expr, expected: &Expr, ctx: &mut Context) {
        let inferred = self.infer_type(expr, ctx);

        let inferred_nf = self.kernel.whnf_step(&inferred).value;
        let expected_nf = self.kernel.whnf_step(expected).value;

        if inferred_nf != expected_nf {
            panic!(
                "Type mismatch:\n inferred: {:?}\n expected: {:?}",
                inferred_nf, expected_nf
            )
        }
    }
}
