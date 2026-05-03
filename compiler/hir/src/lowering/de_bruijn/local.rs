use ast::{expression::Binder, identifier::Identifier};

pub struct LocalContext {
    stack: Vec<Binder>,
}

impl LocalContext {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, binder: Binder) {
        self.stack.push(binder);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn resolve(&self, id: &Identifier) -> Option<usize> {
        self.stack
            .iter()
            .rev()
            .enumerate()
            .find_map(|(i, binder)| match binder {
                Binder::Named(bound) if bound.name == id.name => Some(i),
                _ => None,
            })
    }
}
