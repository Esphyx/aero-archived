#[derive(Debug, Clone, PartialEq)]
pub enum GlobalRef {
    ConstRef(usize),
    InductiveRef(usize),
}
