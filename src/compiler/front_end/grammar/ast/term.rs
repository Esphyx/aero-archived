use super::{ParseError, Parser, SourceBuiltinType, TokenKind, identifier::SourceIdentifier};

#[derive(Debug, Clone)]
pub enum SourceTerm {
    Identifier(SourceIdentifier),
    Builtin(SourceBuiltinType),
    Let {
        name: SourceIdentifier,
        type_specifier: Box<Option<Self>>,
        value: Box<Self>,
        body: Box<Self>,
    },
    Arrow {
        dependent: Option<SourceIdentifier>,
        from_type: Box<Self>,
        to_type: Box<Self>,
    },
    App {
        function: Box<Self>,
        argument: Box<Self>,
    },
}

impl SourceTerm {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        if matches!(parser.current()?.kind, TokenKind::Let) {
            return Self::parse_let(parser);
        }

        let mut lhs = Self::parse_atom(parser)?;

        while matches!(
            parser.current()?.kind,
            TokenKind::Identifier
                | TokenKind::U8
                | TokenKind::Prop
                | TokenKind::Type
                | TokenKind::OpenParen
                | TokenKind::OpenBracket
                | TokenKind::Unit
        ) {
            let rhs = Self::parse_atom(parser)?;
            lhs = Self::App {
                function: Box::new(lhs),
                argument: Box::new(rhs),
            }
        }

        while matches!(parser.current()?.kind, TokenKind::Arrow) {
            parser.advance()?;
            let rhs = Self::parse(parser)?;
            lhs = Self::Arrow {
                dependent: None,
                from_type: Box::new(lhs),
                to_type: Box::new(rhs),
            }
        }

        Ok(lhs)
    }

    fn parse_atom(parser: &mut Parser) -> Result<Self, ParseError> {
        match &parser.current()?.kind {
            TokenKind::U8 => {
                parser.advance()?;
                Ok(Self::Builtin(SourceBuiltinType::U8))
            }
            TokenKind::Prop => {
                parser.advance()?;
                Ok(Self::Builtin(SourceBuiltinType::Prop))
            }
            TokenKind::Type => {
                parser.advance()?;
                Ok(Self::Builtin(SourceBuiltinType::Type(0))) // TODO: for now assume Type 0
            }
            TokenKind::Unit => {
                parser.advance()?;
                Ok(Self::Builtin(SourceBuiltinType::Unit))
            }
            TokenKind::Identifier => Ok(Self::Identifier(SourceIdentifier::parse(parser)?)),
            TokenKind::OpenParen => {
                parser.advance()?;
                let t = Self::parse(parser)?;
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(t)
            }
            TokenKind::OpenBracket => {
                parser.advance()?;
                let inner = Self::parse(parser)?;
                parser.expect_token(TokenKind::CloseBracket)?;
                Ok(Self::Builtin(SourceBuiltinType::Array {
                    dependent: Box::new(inner),
                }))
            }
            _ => Err(ParseError::ExpectedType {
                found: parser.current()?.kind,
                position: parser.current()?.position,
            }),
        }
    }

    pub fn parse_type_specifier(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Colon)?;
        Self::parse(parser)
    }

    pub fn parse_let(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Let)?;

        let name = SourceIdentifier::parse(parser)?;

        parser.expect_token(TokenKind::Assign)?;

        let value = Self::parse(parser)?;

        let type_specifier = Box::new(parser.optional(Self::parse_type_specifier)?);

        parser.expect_token(TokenKind::SemiColon)?;

        let body = Box::new(Self::parse(parser)?);

        Ok(Self::Let {
            name,
            type_specifier,
            value: Box::new(value),
            body,
        })
    }
}
