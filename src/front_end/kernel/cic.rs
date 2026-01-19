#[derive(Debug)]
pub struct AST {
    pub environment: Environment,
}

#[derive(Debug)]
pub struct Environment {
    pub inductives: Vec<Inductive>,
    pub constants: Vec<(ConstName, Term)>,
}

#[derive(Debug)]
pub enum ConstKind {
    User,
    Builtin(BuiltinType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstName(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Universe {
    Prop,
    Type(u32),
}

#[derive(Debug)]
pub enum BuiltinType {
    Str,
    U8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug)]
pub struct Inductive {
    pub name: String,
    pub r#type: Term,
    pub constructors: Vec<Constructor>,
    pub eliminator: ConstName,
}

#[derive(Debug)]
pub struct Constructor {
    pub name: String,
    pub r#type: Term,
    pub index: usize,
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Identifier(Identifier),
    Universe(Universe),
    Pi {
        dependent: Identifier,
        from_type: Box<Term>,
        to_type: Box<Term>,
    },
    Lambda {
        parameter: Identifier,
        r#type: Box<Term>,
        body: Box<Term>,
    },
    App {
        function: Box<Term>,
        argument: Box<Term>,
    },
    Const(ConstName),
}
