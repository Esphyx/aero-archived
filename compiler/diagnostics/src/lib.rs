#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Location {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }
}

pub struct Diagnostic {
    pub message: String,
    pub primary_span: Option<Span>,
    pub labels: Vec<Label>,
    pub severity: Severity,
}

pub struct Label {
    pub span: Span,
    pub message: Option<String>,
}

pub enum Severity {
    Hint,
    Warning,
    Error,
}
