use hir::lowering::expr::Expr;

pub struct Context {
    types: Vec<Expr>,
}

impl Context {
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    pub fn extend(&mut self, typ: Expr) {
        self.types.push(typ);
    }

    pub fn pop(&mut self) {
        self.types.pop();
    }

    pub fn lookup(&self, index: usize) -> Expr {
        self.types[self.types.len() - 1 - index].clone()
    }
}
