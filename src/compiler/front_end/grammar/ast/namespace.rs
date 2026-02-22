use super::{ParseError, Parser, TokenKind, function::SourceFunction, inductive::SourceInductive};

#[derive(Debug)]
pub struct SourceNamespace {
    pub inductives: Vec<SourceInductive>,
    pub constants: Vec<SourceFunction>,
}

impl SourceNamespace {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut inductives = Vec::new();
        let mut constants = Vec::new();

        while !matches!(parser.current()?.kind, TokenKind::EoF) {
            let current_token = parser.current()?;
            match current_token.kind {
                TokenKind::Inductive => {
                    inductives.push(SourceInductive::parse(parser)?);
                }
                TokenKind::Fn => {
                    constants.push(SourceFunction::parse(parser)?);
                }
                TokenKind::Comment => {
                    parser.advance()?;
                }
                _ => {
                    return Err(ParseError::ExpectedToken {
                        expected: vec![TokenKind::Fn, TokenKind::Inductive],
                        found: current_token.clone(),
                        position: current_token.position,
                    });
                }
            }
        }

        Ok(Self {
            inductives,
            constants,
        })
    }
}
