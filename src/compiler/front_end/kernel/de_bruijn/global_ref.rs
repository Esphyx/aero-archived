use super::namespace::Namespace;

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalRef {
    ConstRef(usize),     // index into namespace constants
    InductiveRef(usize), // index into namespace inductives
}

impl GlobalRef {
    pub fn to_string(&self, namespace: &Namespace) -> String {
        match self {
            GlobalRef::ConstRef(index) => namespace.constants[*index].to_string(),
            GlobalRef::InductiveRef(index) => todo!(),
        }
    }
}
