#[derive(Debug)]
pub struct SourceAST {
    pub environment: SourceNamespace,
}

#[derive(Debug)]
pub struct SourceNamespace {
    pub inductives: Vec<SourceInductive>,
    pub constants: Vec<(SourceIdentifier, SourceTerm)>,
}

#[derive(Debug, Clone)]
pub enum SourceBuiltinType {
    Prop,
    Type(u32),
    U8,
    Array { dependent: Box<SourceTerm> },
}

#[derive(Debug, PartialEq, Clone)]
pub struct SourceIdentifier {
    pub position: usize,
    pub length: usize,
}

impl SourceIdentifier {
    pub fn get_name_str<'a>(&self, input: &'a str) -> &'a str {
        &input[self.position..self.position + self.length]
    }
}

#[derive(Debug)]
pub struct SourceInductive {
    pub name: SourceIdentifier,
    pub r#type: SourceTerm,
    pub constructors: Vec<SourceConstructor>,
    pub eliminator: SourceIdentifier,
}

#[derive(Debug)]
pub struct SourceConstructor {
    pub name: SourceIdentifier,
    pub r#type: SourceTerm,
    pub index: usize,
}

#[derive(Debug, Clone)]
pub enum SourceTerm {
    Identifier(SourceIdentifier),
    Builtin(SourceBuiltinType),
    Lambda {
        parameter: SourceIdentifier,
        r#type: Box<SourceTerm>,
        body: Box<SourceTerm>,
    },
    Pi {
        dependent: SourceIdentifier,
        from_type: Box<SourceTerm>,
        to_type: Box<SourceTerm>,
    },
    App {
        function: Box<SourceTerm>,
        argument: Box<SourceTerm>,
    },
}
