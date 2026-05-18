use lexer::token::TokenKind;
use parser::parser::Parser;

use crate::identifier::Identifier;

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Builtin(Builtin),
    Lambda {
        param: Binder,
        type_specifier: Box<Self>,
        body: Box<Self>,
    },
    Pi {
        dependent: Binder,
        typ: Box<Self>,
        body: Box<Self>,
    },
    App {
        func: Box<Self>,
        arg: Box<Self>,
    },
    Match {
        scrutinee: Box<Self>,
        branches: Vec<Branch>,
    },
    Placeholder,
}

#[derive(Debug, Clone)]
pub enum Binder {
    Named(Identifier),
    Anonymous,
}

#[derive(Debug, Clone)]
pub enum Builtin {
    Prop,
    Type(u32),
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub pattern: Identifier,
    pub body: Expression,
}

impl Branch {
    pub fn parse(parser: &mut Parser) -> Self {
        let pattern = Identifier::parse(parser);
        parser.expect_token(TokenKind::Assign);
        let body = Expression::parse(parser);

        Self { pattern, body }
    }
}

impl Expression {
    fn parse_with_guard(parser: &mut Parser) -> Self {
        let start = parser.position;

        let expr = Self::parse(parser);

        if parser.position == start {
            parser.advance();
        }

        expr
    }

    pub fn parse(parser: &mut Parser) -> Self {
        match parser.peek_kind() {
            TokenKind::Lambda => return Self::parse_lambda(parser),
            TokenKind::Forall => return Self::parse_forall(parser),
            TokenKind::Match => return Self::parse_match(parser),
            _ => {}
        }

        let mut lhs = Self::parse_atom(parser);

        while matches!(
            parser.peek_kind(),
            TokenKind::SelfType
                | TokenKind::Identifier
                | TokenKind::Prop
                | TokenKind::Type
                | TokenKind::OpenParen
                | TokenKind::OpenBracket
        ) {
            let rhs = Self::parse_atom(parser);
            lhs = Self::App {
                func: Box::new(lhs),
                arg: Box::new(rhs),
            }
        }

        while matches!(parser.peek_kind(), TokenKind::Arrow) {
            parser.advance();
            let rhs = Self::parse_with_guard(parser);
            lhs = Self::Pi {
                dependent: Binder::Anonymous,
                typ: Box::new(lhs),
                body: Box::new(rhs),
            }
        }

        lhs
    }

    fn parse_forall(parser: &mut Parser) -> Self {
        parser.expect_token(TokenKind::Forall);

        let parameter = Identifier::parse(parser);
        let type_specifier = Self::parse_type_specifier(parser);

        parser.expect_token(TokenKind::Comma);
        let body = Self::parse(parser);

        Expression::Pi {
            dependent: Binder::Named(parameter),
            typ: Box::new(type_specifier),
            body: Box::new(body),
        }
    }

    fn parse_lambda(parser: &mut Parser) -> Self {
        parser.expect_token(TokenKind::Lambda);

        let param = Binder::Named(Identifier::parse(parser));
        let type_specifier = Box::new(Self::parse_type_specifier(parser));

        parser.expect_token(TokenKind::FatArrow);
        let body = Box::new(Self::parse(parser));

        Self::Lambda {
            param,
            type_specifier,
            body,
        }
    }

    fn parse_match(parser: &mut Parser) -> Self {
        parser.expect_token(TokenKind::Match);
        let scrutinee = Box::new(Self::parse(parser));
        parser.expect_token(TokenKind::With);

        let mut branches = Vec::new();
        while matches!(parser.peek_kind(), TokenKind::Pipe) {
            parser.advance();
            let branch = Branch::parse(parser);
            branches.push(branch);
        }

        Expression::Match {
            scrutinee,
            branches,
        }
    }

    fn parse_atom(parser: &mut Parser) -> Self {
        let kind = parser.peek_kind();

        fn parse_type(parser: &mut Parser, builtin: Builtin) -> Expression {
            parser.advance();
            Expression::Builtin(builtin)
        }

        match &kind {
            TokenKind::Prop => parse_type(parser, Builtin::Prop),
            TokenKind::Type => parse_type(parser, Builtin::Type(0)), // TODO: assuming 0 for now
            TokenKind::Identifier => Self::Identifier(Identifier::parse(parser)),
            TokenKind::OpenParen => {
                parser.advance();
                let t = Self::parse(parser);
                parser.expect_token(TokenKind::CloseParen);
                t
            }
            _ => {
                parser.error("expected expression");
                parser.advance();
                parser.synchronize();
                Expression::Placeholder
            }
        }
    }

    pub fn parse_type_specifier(parser: &mut Parser) -> Self {
        parser.expect_token(TokenKind::Colon);
        Self::parse(parser)
    }
}
