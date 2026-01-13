use std::fmt::Display;

use crate::{
    ast::{AbstractSyntaxTree, Expression},
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

        

        todo!()
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_arrow()
    }

    pub fn parse_arrow(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_application()?;

        Ok(if self.current == Token::Arrow {
            self.advance()?;

            let right = self.parse_arrow()?;

            Expression::Arrow {
                input: Box::new(left),
                output: Box::new(right),
            }
        } else {
            left
        })
    }

    pub fn parse_lambda(&mut self) -> Result<Expression, ParseError> {
        self.expect(Token::Pipe)?;
        let parameter = if let Token::Identifier(name) = &self.current {
            name.clone()
        } else {
            println!("In lambda");
            return Err(ParseError::ExpectedIdentifier {
                found: self.current.clone(),
            });
        };
        self.advance()?;
        println!("Parsed '{}'", parameter);

        self.expect(Token::Colon)?;
        let of_type = Box::new(self.parse_expression()?);

        self.expect(Token::Pipe)?;

        self.expect(Token::OpenBrace)?;
        let body = Box::new(self.parse_expression()?);
        self.expect(Token::CloseBrace)?;

        Ok(Expression::Lambda {
            parameter,
            of_type,
            body,
        })
    }

    pub fn parse_application(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_atom()?;

        println!("Parsing application!");

        while let Token::Identifier(_) | Token::OpenParen = self.current {
            let right = self.parse_atom()?;
            left = Expression::Application {
                function: Box::new(left),
                argument: Box::new(right),
            };
        }

        Ok(left)
    }

    pub fn parse_atom(&mut self) -> Result<Expression, ParseError> {
        println!("Parsing atom! {:?}", self.current);
        match &self.current {
            Token::Identifier(name) => {
                let variable = Expression::Variable(name.clone());
                self.advance()?;
                Ok(variable)
            }
            Token::OpenParen => {
                self.advance()?;
                let expression = self.parse_expression()?;
                self.expect(Token::CloseParen)?;
                Ok(expression)
            }
            Token::Pipe => self.parse_lambda(),
            _ => {
                println!("while parsing atom");
                Err(ParseError::ExpectedExpression {
                    found: self.current.clone(),
                })
            }
        }
    }
}
