use lexer::token::TokenKind;
use parser::{error::ParseError, parser::Parser};

use crate::{
    builtin::{Builtin, BuiltinType},
    identifier::Identifier,
};

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Builtin(Builtin),
    Let {
        name: Identifier,
        type_specifier: Box<Option<Self>>,
        value: Box<Self>,
        body: Box<Self>,
    },
    Lambda {
        parameter: Identifier,
        type_specifier: Box<Self>,
        body: Box<Self>,
    },
    Arrow {
        dependent: Option<Identifier>,
        typ: Box<Self>,
        body: Box<Self>,
    },
    App {
        function: Box<Self>,
        argument: Box<Self>,
    },
    Match {
        scrutinee: Box<Self>,
        branches: Vec<Branch>,
    },
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub pattern: Identifier,
    pub body: Expression,
}

impl Branch {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let pattern = Identifier::parse(parser)?;
        parser.expect_token(TokenKind::Assign)?;
        let body = Expression::parse(parser)?;

        Ok(Self { pattern, body })
    }
}

impl Expression {
    pub fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let current = parser.current().kind;
        if matches!(current, TokenKind::Lambda) {
            return Self::parse_lambda(parser);
        }

        if matches!(current, TokenKind::Forall) {
            return Self::parse_forall(parser);
        }

        if matches!(current, TokenKind::Let) {
            return Self::parse_let(parser);
        }

        if matches!(current, TokenKind::Match) {
            return Self::parse_match(parser);
        }

        let mut lhs = Self::parse_atom(parser)?;

        while matches!(
            parser.current().kind,
            TokenKind::SelfType
                | TokenKind::Identifier
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

        while matches!(parser.current().kind, TokenKind::Arrow) {
            parser.advance();
            let rhs = Self::parse(parser)?;
            lhs = Self::Arrow {
                dependent: None,
                typ: Box::new(lhs),
                body: Box::new(rhs),
            }
        }

        Ok(lhs)
    }

    fn parse_forall(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Forall)?;

        let parameter = Identifier::parse(parser)?;
        let type_specifier = Self::parse_type_specifier(parser)?;

        parser.expect_token(TokenKind::Comma)?;
        let body = Self::parse(parser)?;

        Ok(Expression::Arrow {
            dependent: Some(parameter),
            typ: Box::new(type_specifier),
            body: Box::new(body),
        })
    }

    fn parse_lambda(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Lambda)?;

        let parameter = Identifier::parse(parser)?;
        let type_specifier = Box::new(Self::parse_type_specifier(parser)?);

        parser.expect_token(TokenKind::FatArrow)?;
        let body = Box::new(Self::parse(parser)?);

        Ok(Self::Lambda {
            parameter,
            type_specifier,
            body,
        })
    }

    fn parse_match(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Match)?;
        let scrutinee = Box::new(Self::parse(parser)?);
        parser.expect_token(TokenKind::With)?;

        let mut branches = Vec::new();
        while matches!(parser.current().kind, TokenKind::Pipe) {
            parser.advance();
            let branch = Branch::parse(parser)?;
            branches.push(branch);
        }

        Ok(Expression::Match {
            scrutinee,
            branches,
        })
    }

    fn parse_atom(parser: &mut Parser) -> Result<Self, ParseError> {
        let kind = parser.current().kind;

        fn parse_type(parser: &mut Parser, ty: BuiltinType) -> Result<Expression, ParseError> {
            parser.advance();
            Ok(Expression::Builtin(Builtin::Type(ty)))
        }

        match &kind {
            TokenKind::SelfType => parse_type(parser, BuiltinType::SelfType),
            TokenKind::Prop => parse_type(parser, BuiltinType::Prop),
            TokenKind::Type => parse_type(parser, BuiltinType::Type(0)), // TODO: assume 0 for now
            TokenKind::Unit => parse_type(parser, BuiltinType::Unit),
            TokenKind::Identifier => Ok(Self::Identifier(Identifier::parse(parser)?)),
            TokenKind::OpenParen => {
                parser.advance();
                let t = Self::parse(parser)?;
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(t)
            }
            _ => {
                panic!("Expected expression, found: {:?}!", parser.current().kind);
            }
        }
    }

    pub fn parse_type_specifier(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Colon)?;
        Self::parse(parser)
    }

    pub fn parse_let(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect_token(TokenKind::Let)?;

        let name = Identifier::parse(parser)?;

        let type_specifier = Box::new(Some(Self::parse_type_specifier(parser)?));

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
