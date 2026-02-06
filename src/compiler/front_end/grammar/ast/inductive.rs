use super::{ParseError, Parser, SourceIdentifier, SourceTerm, TokenKind};

#[derive(Debug)]
pub struct SourceInductive {
    pub name: SourceIdentifier,
    pub parameters: Vec<(SourceIdentifier, SourceTerm)>,
    pub typ: SourceTerm,
    pub constructors: Vec<SourceConstructor>,
    pub eliminator: SourceIdentifier,
}

impl SourceInductive {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Inductive)?;

        let name = SourceIdentifier::parse(parser)?;

        // PARAMETERS
        let mut parameters = Vec::new();
        while matches!(parser.current()?.kind, TokenKind::OpenParen) {
            parser.advance()?;
            let parameter_name = SourceIdentifier::parse(parser)?;
            parser.expect_token(TokenKind::Colon)?;
            let parameter_type = SourceTerm::parse(parser)?;
            parser.expect_token(TokenKind::CloseParen)?;
            parameters.push((parameter_name, parameter_type));
        }
        
        let typ = SourceTerm::parse_type_specifier(parser)?;

        // body
        parser.expect_token(TokenKind::OpenBrace)?;

        // constructors
        let mut constructors = Vec::new();
        while !(matches!(
            parser.current()?.kind,
            TokenKind::CloseBrace | TokenKind::SemiColon
        )) {
            constructors.push(SourceConstructor::parse(parser, &parameters)?);

            if matches!(parser.current()?.kind, TokenKind::Comma) {
                parser.advance()?;
            }
        }
        parser.expect_token(TokenKind::SemiColon)?;
        // ELIMINATOR
        let eliminator = SourceIdentifier::parse(parser)?;

        parser.expect_token(TokenKind::CloseBrace)?;

        Ok(Self {
            name,
            parameters,
            typ,
            constructors,
            eliminator,
        })
    }
}

#[derive(Debug)]
pub struct SourceConstructor {
    pub name: SourceIdentifier,
    pub typ: SourceTerm,
}

impl SourceConstructor {
    pub fn parse(
        parser: &mut Parser,
        parameters: &Vec<(SourceIdentifier, SourceTerm)>,
    ) -> Result<Self, ParseError> {
        let name = SourceIdentifier::parse(parser)?;
        parser.expect_token(TokenKind::Colon)?;

        // wrap with parameters
        let mut typ = SourceTerm::parse(parser)?;
        for (parameter_name, parameter_type) in parameters.iter().rev() {
            typ = SourceTerm::Arrow {
                dependent: Some(parameter_name.clone()),     // CLONE
                from_type: Box::new(parameter_type.clone()), // CLONE
                to_type: Box::new(typ),
            };
        }

        Ok(Self { name, typ })
    }
}
