pub struct Register {
    pub name: &'static str,
    pub layout: Vec<RegisterLayout>,
}

impl Register {
    pub fn new(name: &'static str, layout: Vec<RegisterLayout>) -> Self {
        Self { name, layout }
    }

    pub fn size(&self) -> usize {
        self.layout.iter().map(RegisterLayout::size).sum()
    }
}

pub enum RegisterLayout {
    Bits(usize),
    Subregister(Box<Register>),
}

impl RegisterLayout {
    pub fn size(&self) -> usize {
        match self {
            RegisterLayout::Bits(length) => *length,
            RegisterLayout::Subregister(register) => register.size(),
        }
    }
}
