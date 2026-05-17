use lexer::token::{Span, TokenKind};
use parser::{error::ParseError, parser::Parser};

#[derive(Clone, Debug)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let token = parser.current();

        if token.kind != TokenKind::Identifier {
            panic!("hi");
        }

        let span = token.span;
        let name = parser.input[span.start..span.end()].to_string();

        parser.advance();
        Ok(Self { name, span })
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}
