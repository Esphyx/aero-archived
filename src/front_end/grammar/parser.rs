use std::fmt::Display;

use crate::front_end::{
    grammar::{
        lexer::LexicalError,
        token::{Token, TokenKind},
    },
    kernel::parser_ast::{
        SourceAST, SourceBuiltinType, SourceConstructor, SourceNamespace, SourceIdentifier, SourceInductive, SourceTerm,
    },
};

#[allow(unused)]
#[derive(Debug)]
pub enum ParseError {
    ExpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },
    ExpectedIdentifier {
        found: TokenKind,
    },
    ExpectedExpression {
        found: TokenKind,
    },
    ExpectedType {
        found: TokenKind,
    },
    LexicalError(LexicalError),
    OutOfTokens,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Result<Self, ParseError> {
        Ok(Self {
            tokens,
            position: 0,
        })
    }

    fn current(&mut self) -> Result<&Token, ParseError> {
        self.tokens
            .get(self.position)
            .ok_or(ParseError::OutOfTokens)
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        loop {
            self.position += 1;

            if !matches!(self.current()?.kind, TokenKind::Comment) {
                break;
            }
        }

        Ok(())
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), ParseError> {
        if self.current()?.kind == expected {
            self.advance()
        } else {
            Err(ParseError::ExpectedToken {
                expected,
                found: self.current()?.kind,
            })
        }
    }

    fn parse_identifier(&mut self) -> Result<SourceIdentifier, ParseError> {
        if let TokenKind::Identifier = &self.current()?.kind {
            let id = SourceIdentifier {
                position: self.current()?.position,
                length: self.current()?.length,
            };
            self.advance()?;
            Ok(id)
        } else {
            Err(ParseError::ExpectedIdentifier {
                found: self.current()?.kind,
            })
        }
    }

    pub fn parse(&mut self) -> Result<SourceAST, ParseError> {
        Ok(SourceAST {
            environment: self.parse_environment()?,
        })
    }

    fn parse_environment(&mut self) -> Result<SourceNamespace, ParseError> {
        let mut inductives = Vec::new();
        let mut constants = Vec::new();

        while !matches!(self.current()?.kind, TokenKind::EndOfFile) {
            match &self.current()?.kind {
                TokenKind::Inductive => {
                    inductives.push(self.parse_inductive()?);
                }
                TokenKind::Fn => {
                    let (name, ty) = self.parse_fn()?;
                    constants.push((name, ty));
                }
                _ => {
                    return Err(ParseError::ExpectedToken {
                        expected: TokenKind::Inductive, // or function
                        found: self.current()?.kind,
                    });
                }
            }
        }

        Ok(SourceNamespace {
            inductives,
            constants,
        })
    }

    fn parse_fn(&mut self) -> Result<(SourceIdentifier, SourceTerm), ParseError> {
        // DEFINITION
        self.expect(TokenKind::Fn)?;
        let name = self.parse_identifier()?;

        // PARAMETERS
        let mut parameters = Vec::new();
        while matches!(self.current()?.kind, TokenKind::OpenParen) {
            self.advance()?;
            let param_name = self.parse_identifier()?;
            self.expect(TokenKind::Colon)?;
            let param_type = self.parse_term()?;
            self.expect(TokenKind::CloseParen)?;
            parameters.push((param_name, param_type));
        }

        // RETURN TYPE

        self.expect(TokenKind::Colon)?;
        let _return_type = self.parse_term()?;

        // BODY
        self.expect(TokenKind::OpenBrace)?;
        let body = self.parse_term()?;
        self.expect(TokenKind::CloseBrace)?;

        let mut lambda = body;
        for (parameter, parameter_type) in parameters.into_iter().rev() {
            lambda = SourceTerm::Lambda {
                parameter,
                r#type: Box::new(parameter_type),
                body: Box::new(lambda),
            }
        }

        Ok((name, lambda))
    }

    fn parse_inductive(&mut self) -> Result<SourceInductive, ParseError> {
        self.expect(TokenKind::Inductive)?;
        let name = self.parse_identifier()?;

        self.expect(TokenKind::Colon)?;
        let ty = self.parse_term()?;

        self.expect(TokenKind::OpenBrace)?;

        let mut constructors = Vec::new();
        let mut index = 0;
        while !(matches!(
            self.current()?.kind,
            TokenKind::CloseBrace | TokenKind::SemiColon
        )) {
            let cname = self.parse_identifier()?;
            self.expect(TokenKind::Colon)?;
            let cty = self.parse_term()?;
            constructors.push(SourceConstructor {
                name: cname,
                r#type: cty,
                index,
            });
            index += 1;

            if matches!(self.current()?.kind, TokenKind::Comma) {
                self.advance()?;
            }
        }
        self.expect(TokenKind::SemiColon)?;
        let eliminator = self.parse_identifier()?;

        self.expect(TokenKind::CloseBrace)?;

        Ok(SourceInductive {
            name,
            r#type: ty,
            constructors,
            eliminator,
        })
    }

    fn parse_term(&mut self) -> Result<SourceTerm, ParseError> {
        let mut lhs = match &self.current()?.kind {
            TokenKind::U8 => {
                self.advance()?;
                Ok(SourceTerm::Builtin(SourceBuiltinType::U8))
            }
            TokenKind::Prop => {
                self.advance()?;
                Ok(SourceTerm::Builtin(SourceBuiltinType::Prop))
            }
            TokenKind::Type => {
                self.advance()?;
                Ok(SourceTerm::Builtin(SourceBuiltinType::Type(0))) // DEFAULT TO TYPE 0 FOR NOW
            }
            TokenKind::Identifier => Ok(SourceTerm::Identifier(self.parse_identifier()?)),
            TokenKind::OpenParen => {
                self.advance()?;
                let t = self.parse_term()?;
                self.expect(TokenKind::CloseParen)?;
                Ok(t)
            }
            TokenKind::OpenBracket => {
                self.advance()?;
                let inner = self.parse_term()?;
                self.expect(TokenKind::CloseBracket)?;
                Ok(SourceTerm::Builtin(SourceBuiltinType::Array {
                    dependent: Box::new(inner),
                }))
            }
            _ => Err(ParseError::ExpectedType {
                found: self.current()?.kind,
            }),
        }?;

        while matches!(self.current()?.kind, TokenKind::Arrow) {
            self.advance()?;
            let rhs = self.parse_term()?;
            lhs = SourceTerm::Pi {
                dependent: SourceIdentifier {
                    position: 0,
                    length: 0,
                }, // NOT IDIOMATIC
                from_type: Box::new(lhs),
                to_type: Box::new(rhs),
            }
        }

        Ok(lhs)
    }
}
