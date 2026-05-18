use diagnostics::Span;
use lexer::token::TokenKind;
use parser::parser::Parser;

#[derive(Clone, Debug)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn parse(parser: &mut Parser) -> Self {
        parser.expect_token(TokenKind::Identifier);

        let token = match parser.peek().cloned() {
            Some(tok) if tok.kind == TokenKind::Identifier => tok,
            Some(tok) => {
                parser.error("expected identifier");
                tok
            }
            None => {
                parser.error("expected identifier, found eof");
                return Identifier {
                    name: "_".into(),
                    span: Span {
                        start: Default::default(),
                        end: Default::default(),
                    },
                };
            }
        };
        let span = token.span;
        let name = token.lexeme.clone().unwrap_or("_".to_string());

        parser.advance();
        Self { name, span }
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
