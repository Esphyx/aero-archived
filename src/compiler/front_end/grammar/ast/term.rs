use super::{
    ParseError, Parser, SourceBuiltin, SourceBuiltinType, SourcePrimitive, TokenKind,
    identifier::SourceIdentifier,
};

#[derive(Debug, Clone)]
pub enum SourceTerm {
    Identifier(SourceIdentifier),
    Builtin(SourceBuiltin),
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
                | TokenKind::ReadU8
                | TokenKind::WriteU8
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
        let kind = parser.current()?.kind;

        fn parse_primitive(
            parser: &mut Parser,
            primitive: SourcePrimitive,
        ) -> Result<SourceTerm, ParseError> {
            parser.advance()?;
            Ok(SourceTerm::Builtin(SourceBuiltin::Primitive(primitive)))
        }

        fn parse_type(
            parser: &mut Parser,
            ty: SourceBuiltinType,
        ) -> Result<SourceTerm, ParseError> {
            parser.advance()?;
            Ok(SourceTerm::Builtin(SourceBuiltin::Type(ty)))
        }

        match &kind {
            TokenKind::ElimU8 => parse_primitive(parser, SourcePrimitive::ElimU8),
            TokenKind::ZeroU8 => parse_primitive(parser, SourcePrimitive::ZeroU8),
            TokenKind::SuccU8 => parse_primitive(parser, SourcePrimitive::SuccU8),
            TokenKind::WriteU8 => parse_primitive(parser, SourcePrimitive::WriteU8),
            TokenKind::ReadU8 => parse_primitive(parser, SourcePrimitive::ReadU8),
            TokenKind::U8 => parse_type(parser, SourceBuiltinType::U8),
            TokenKind::Prop => parse_type(parser, SourceBuiltinType::Prop),
            TokenKind::Type => parse_type(parser, SourceBuiltinType::Type(0)), // TODO: assume 0 for now
            TokenKind::Unit => parse_type(parser, SourceBuiltinType::Unit),
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
                Ok(Self::Builtin(SourceBuiltin::Type(
                    SourceBuiltinType::Array {
                        dependent: Box::new(inner),
                    },
                )))
            }
            _ => Err(ParseError::ExpectedExpression {
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

        let type_specifier = Box::new(parser.optional(Self::parse_type_specifier)?);

        parser.expect_token(TokenKind::Assign)?;

        let value = Self::parse(parser)?;

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
