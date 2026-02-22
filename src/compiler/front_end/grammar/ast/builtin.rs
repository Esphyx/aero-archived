use super::SourceTerm;

#[derive(Debug, Clone)]
pub enum SourceBuiltinType {
    Prop,
    Type(u32),
    U8,
    Unit,
    Array { dependent: Box<SourceTerm> },
}

#[derive(Debug, Clone)]
pub enum SourceBuiltin {
    Type(SourceBuiltinType),
    Primitive(SourcePrimitive),
}

#[derive(Debug, Clone)]
pub enum SourcePrimitive {
    ReadU8,
    WriteU8,
    ZeroU8,
    SuccU8,
    ElimU8,
}
