use std::collections::HashMap;

use super::{
    super::super::grammar::ast::{
        function::SourceFunction, identifier::SourceIdentifier, namespace::SourceNamespace,
    },
    builtin::{Builtin, Primitive},
    global_ref::GlobalRef,
};

pub struct LocalContext {
    stack: Vec<SourceIdentifier>,
}

impl LocalContext {
    // TODO: store the index/size from the time of insertion
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, id: SourceIdentifier) {
        self.stack.push(id);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn lookup_index(&self, id: &SourceIdentifier) -> Option<usize> {
        // Later for better errors: while matching names down the stack, save information on closely matching names
        // or for efficiency only when it fails at symbol resolution
        self.stack
            .iter()
            .rev()
            .position(|bound| bound.get_name_str() == id.get_name_str())
    }
}

pub struct GlobalContext<'a> {
    constants: HashMap<&'a str, usize>,
    inductives: HashMap<&'a str, usize>,
}

impl<'a> GlobalContext<'a> {
    pub fn new(namespace: &'a SourceNamespace) -> Self {
        let constants = namespace
            .constants
            .iter()
            .enumerate()
            .map(|(i, SourceFunction { name, .. })| (name.get_name_str(), i))
            .collect();

        let inductives = namespace
            .inductives
            .iter()
            .enumerate()
            .map(|(i, ind)| (ind.name.get_name_str(), i))
            .collect();

        Self {
            constants,
            inductives,
        }
    }

    pub fn resolve(&self, id: &SourceIdentifier) -> Option<GlobalRef> {
        let name = id.get_name_str();

        if let Some(&index) = self.constants.get(name) {
            Some(GlobalRef::ConstRef(index))
        } else if let Some(&index) = self.inductives.get(name) {
            Some(GlobalRef::InductiveRef(index))
        } else {
            None
        }
    }
}

pub struct BuiltinContext {}

impl BuiltinContext {
    pub fn resolve(&self, name: &SourceIdentifier) -> Option<Builtin> {
        match name.get_name_str() {
            "read_u8" => Some(Builtin::Primitive(Primitive::ReadU8)),
            "write_u8" => Some(Builtin::Primitive(Primitive::WriteU8)),
            "add_u8" => Some(Builtin::Primitive(Primitive::AddU8)),
            "sub_u8" => Some(Builtin::Primitive(Primitive::SubU8)),
            "exit" => Some(Builtin::Primitive(Primitive::Exit)),
            _ => None,
        }
    }
}
