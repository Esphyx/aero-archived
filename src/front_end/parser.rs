use std::fmt::Display;

use crate::front_end::{
    ast::{AbstractSyntaxTree, BinaryOperator, Expression},
    lexer::{LexicalAnalizer, LexicalError},
    token::Token,
};

#[derive(Debug)]
pub enum ParseError {
    ExpectedToken { expected: Token, found: Token },
    ExpectedIdentifier { found: Token },
    ExpectedExpression { found: Token },
    ExpectedType { found: Token },
    LexicalError(LexicalError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}

pub struct Parser {
    lexer: LexicalAnalizer,
    current: Token,
}

impl Parser {
    pub fn new(lexer: LexicalAnalizer) -> Result<Self, ParseError> {
        let current = Token::EndOfFile;

        let mut ret = Self { lexer, current };

        ret.advance()?;

        Ok(ret)
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.current = self
            .lexer
            .next_token()
            .map_err(|e| ParseError::LexicalError(e))?;

        Ok(())
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current == expected {
            self.advance()
        } else {
            Err(ParseError::ExpectedToken {
                expected,
                found: self.current.clone(),
            })
        }
    }

    pub fn parse(&mut self) -> Result<AbstractSyntaxTree, ParseError> {
        Ok(AbstractSyntaxTree {
            expression: self.parse_expression()?,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let mut left = match self.current {
            Token::NumberLiteral(n) => {
                self.advance()?;
                Expression::NumberLiteral(n)
            }
            Token::OpenParen => {
                self.advance()?;
                let expression = self.parse_expression()?;
                self.expect(Token::CloseParen)?;
                expression
            }
            _ => panic!("Unmatched token in parse expression: {:?}", self.current),
        };

        while matches!(self.current, Token::Plus | Token::Minus) {
            let op = match self.current {
                Token::Plus => BinaryOperator::Plus,
                Token::Minus => BinaryOperator::Minus,
                _ => unreachable!(),
            };
            
            self.advance()?;

            let right = self.parse_expression()?;
            left = Expression::BinaryOperator {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }

        Ok(left)
    }
}
