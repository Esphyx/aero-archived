use std::collections::HashMap;

use ast::{expression::Binder, identifier::Identifier};

use crate::lowering::expr::BinderId;

pub struct BinderInfo {
    pub binder: Binder,
}

pub struct LocalBinder {
    pub binder: Binder,
    pub id: BinderId,
}

pub struct Scope {
    parent: Option<usize>,
    binders: Vec<LocalBinder>,
}

pub struct LocalContext {
    current: usize,
    scopes: Vec<Scope>,
    next_id: usize,
    binder_info: HashMap<BinderId, BinderInfo>,
}

impl LocalContext {
    pub fn new() -> Self {
        Self {
            current: 0,
            scopes: vec![Scope {
                parent: None,
                binders: Vec::new(),
            }],
            next_id: 0,
            binder_info: HashMap::new(),
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

    pub fn bind(&mut self, binder: Binder) -> BinderId {
        let id = BinderId { id: self.next_id };

        self.next_id += 1;

        self.binder_info.insert(
            id,
            BinderInfo {
                binder: binder.clone(),
            },
        );

        self.scopes[self.current]
            .binders
            .push(LocalBinder { binder, id });

        id
    }

    pub fn binder_name(&self, id: &BinderId) -> Option<&Binder> {
        self.binder_info.get(id).map(|i| &i.binder)
    }

    pub fn extend(&mut self, binder: Binder) -> BinderId {
        self.enter_scope();
        self.bind(binder)
    }

    pub fn pop(&mut self) {
        self.leave_scope();
    }

    pub fn resolve(&self, id: &Identifier) -> Option<(usize, BinderId)> {
        let mut scope = self.current;
        let mut depth = 0;

        loop {
            let current = &self.scopes[scope];
            for binder in current.binders.iter().rev() {
                match &binder.binder {
                    Binder::Named(bound) if bound == id => return Some((depth, binder.id)),
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
