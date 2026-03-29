#[derive(Debug, Clone)]
pub enum BuiltinType {
    Prop,
    Type(u32),
    Unit,
    SelfType,
}

#[derive(Debug, Clone)]
pub enum Builtin {
    Type(BuiltinType),
    Primitive(Primitive),
}

#[derive(Debug, Clone)]
pub enum Primitive {
    ReadU8,
    WriteU8,
    ZeroU8,
    SuccU8,
    ElimU8,
}
