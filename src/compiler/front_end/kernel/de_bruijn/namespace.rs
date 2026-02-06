use super::{Inductive, constant::Function};

#[derive(Debug)]
pub struct Namespace {
    pub inductives: Vec<Inductive>,
    pub constants: Vec<Function>,
}

impl Namespace {
    pub fn to_string(&self) -> String {
        todo!()
    }
}

impl From<(Vec<Inductive>, Vec<Function>)> for Namespace {
    fn from(value: (Vec<Inductive>, Vec<Function>)) -> Self {
        let (inductives, constants) = value;
        Self {
            inductives,
            constants,
        }
    }
}
