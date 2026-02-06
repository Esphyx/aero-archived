use super::{Function, Inductive, Namespace};

#[derive(Debug)]
pub struct Program {
    pub namespace: Namespace,
    pub entry_point: usize,
}

impl Program {
    pub fn to_string(&self) -> String {
        todo!()
    }
}

impl From<((Vec<Inductive>, Vec<Function>), usize)> for Program {
    fn from(value: ((Vec<Inductive>, Vec<Function>), usize)) -> Self {
        let (namespace, entry_point) = value;
        Self {
            namespace: Namespace::from(namespace),
            entry_point,
        }
    }
}
