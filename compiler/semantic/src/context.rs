use hir::lowering::expr::Expr;

use crate::traversal::shift_indices;

pub struct Context {
    types: Vec<Expr>,
}

impl Context {
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    pub fn extend(&mut self, typ: Expr) {
        self.types.push(shift_indices(&typ, 0, 1));
    }

    pub fn pop(&mut self) {
        self.types.pop();
    }

    pub fn lookup(&self, index: usize) -> Expr {
        let ty = &self.types[self.types.len() - 1 - index];
        shift_indices(ty, 0, (index + 1) as isize)
    }
}
