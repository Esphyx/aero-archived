use ast::{expression::Binder, identifier::Identifier};

pub struct Scope {
    pub parent: Option<usize>,
    pub binders: Vec<Binder>,
}

pub struct LocalContext {
    current: usize,
    scopes: Vec<Scope>,
}

impl LocalContext {
    pub fn new() -> Self {
        Self {
            current: 0,
            scopes: vec![Scope {
                parent: None,
                binders: Vec::new(),
            }],
        }
    }

    pub fn enter_scope(&mut self) {
        let id = self.scopes.len();
        self.scopes.push(Scope {
            parent: Some(self.current),
            binders: Vec::new(),
        });

        self.current = id;
    }

    pub fn leave_scope(&mut self) {
        self.current = self.scopes[self.current]
            .parent
            .expect("Cannot leave root scope")
    }

    pub fn bind(&mut self, binder: Binder) {
        self.scopes[self.current].binders.push(binder);
    }

    pub fn extend(&mut self, binder: Binder) {
        self.enter_scope();
        self.bind(binder);
    }

    pub fn pop(&mut self) {
        self.leave_scope();
    }

    pub fn resolve(&self, id: &Identifier) -> Option<usize> {
        let mut scope = self.current;
        let mut depth = 0;

        loop {
            let current = &self.scopes[scope];
            for binder in current.binders.iter().rev() {
                match binder {
                    Binder::Named(bound) if bound == id => return Some(depth),
                    _ => depth += 1,
                }
            }

            match current.parent {
                Some(parent) => scope = parent,
                None => return None,
            }
        }
    }
}
