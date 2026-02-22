pub struct OperandInfo {
    pub kind: OperandKind,
}

pub enum OperandKind {
    Register(&'static str),
    Immediate { bits: usize },
    Memory,
}
