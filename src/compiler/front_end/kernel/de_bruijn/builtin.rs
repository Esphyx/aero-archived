use super::{
    super::super::grammar::ast::SourceBuiltinType,
    context::{BuiltinContext, GlobalContext, LocalContext},
    term::Term,
    term_to_de_bruijn,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Type(BuiltinType),
    Primitive(Primitive),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinType {
    Prop,
    Type(u32),
    U8,
    Bool,
    Unit,
    Array { dependent: Box<Term> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    ReadU8,
    WriteU8,
    AddU8,
    SubU8,
    Exit,
}

impl BuiltinType {
    pub fn from_source<'a>(
        builtin_source: &'a SourceBuiltinType,
        local_context: &mut LocalContext,
        global_context: &GlobalContext,
        builtin_context: &BuiltinContext,
    ) -> Self {
        match builtin_source {
            SourceBuiltinType::Prop => Self::Prop,
            SourceBuiltinType::Type(u) => Self::Type(*u),
            SourceBuiltinType::U8 => Self::U8,
            SourceBuiltinType::Unit => Self::Unit,
            SourceBuiltinType::Bool => Self::Bool,
            SourceBuiltinType::Array { dependent } => Self::Array {
                dependent: Box::new(term_to_de_bruijn(
                    &dependent,
                    local_context,
                    global_context,
                    builtin_context,
                )),
            },
        }
    }
}
