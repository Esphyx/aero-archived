use super::{super::super::grammar::ast::identifier::SourceIdentifier, term::Term};

#[derive(Debug)]
pub struct Function {
    pub name: SourceIdentifier,
    pub parameter_types: Vec<Term>,
    pub return_type: Option<Term>,
    pub definition: Term,
}

impl Function {
    pub fn wrap_with_lambdas(&self) -> Term {
        let mut body = self.definition.clone();
        for typ in self.parameter_types.iter() {
            body = Term::Lambda {
                typ: Box::new(Some(typ.clone())),
                body: Box::new(body),
            }
        }
        body
    }

    pub fn to_string(&self) -> String {
        todo!()
    }
}
