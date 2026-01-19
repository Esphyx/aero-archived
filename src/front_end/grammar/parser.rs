// use std::fmt::Display;

// use crate::front_end::{
//     grammar::{
//         lexer::{LexicalAnalizer, LexicalError},
//         token::TokenKind,
//     },
//     kernel::{
//         cic::{AST, ConstName, Environment, Identifier, Universe},
//         inductive::{Constructor, Inductive},
//         term::Term,
//     },
// };

// #[derive(Debug)]
// pub enum ParseError {
//     ExpectedToken { expected: TokenKind, found: TokenKind },
//     ExpectedIdentifier { found: TokenKind },
//     ExpectedExpression { found: TokenKind },
//     ExpectedType { found: TokenKind },
//     LexicalError(LexicalError),
// }

// impl Display for ParseError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl std::error::Error for ParseError {}

// pub struct Parser {
//     lexer: LexicalAnalizer,
//     current: TokenKind,
// }

// impl Parser {
//     pub fn new(lexer: LexicalAnalizer) -> Result<Self, ParseError> {
//         let current = TokenKind::EndOfFile;
//         let mut ret = Self { lexer, current };
//         ret.advance()?;
//         Ok(ret)
//     }

//     fn advance(&mut self) -> Result<(), ParseError> {
//         loop {
//             self.current = self
//                 .lexer
//                 .next_token()
//                 .map_err(|e| ParseError::LexicalError(e))?;

//             if !matches!(self.current, TokenKind::Comment(_)) {
//                 break;
//             }
//         }

//         Ok(())
//     }

//     fn expect(&mut self, expected: TokenKind) -> Result<(), ParseError> {
//         if self.current == expected {
//             self.advance()
//         } else {
//             Err(ParseError::ExpectedToken {
//                 expected,
//                 found: self.current.clone(),
//             })
//         }
//     }

//     fn expect_identifier(&mut self) -> Result<String, ParseError> {
//         if let TokenKind::Identifier(n) = &self.current {
//             let out = n.clone();
//             self.advance()?;
//             Ok(out)
//         } else {
//             Err(ParseError::ExpectedIdentifier {
//                 found: self.current.clone(),
//             })
//         }
//     }

//     pub fn parse(&mut self) -> Result<AST, ParseError> {
//         Ok(AST {
//             environment: self.parse_environment()?,
//         })
//     }

//     fn parse_environment(&mut self) -> Result<Environment, ParseError> {
//         let mut inductives = Vec::new();
//         let mut constants = Vec::new();

//         while !matches!(self.current, TokenKind::EndOfFile) {
//             match &self.current {
//                 TokenKind::Inductive => {
//                     inductives.push(self.parse_inductive()?);
//                 }
//                 TokenKind::Fn => {
//                     let (name, ty) = self.parse_fn()?;
//                     constants.push((ConstName(name), ty));
//                 }
//                 _ => {
//                     return Err(ParseError::ExpectedToken {
//                         expected: TokenKind::Inductive, // or function
//                         found: self.current.clone(),
//                     });
//                 }
//             }
//         }

//         Ok(Environment {
//             inductives,
//             constants,
//         })
//     }

//     fn parse_fn(&mut self) -> Result<(String, Term), ParseError> {
//         // DEFINITION
//         self.expect(TokenKind::Fn)?;
//         let name = self.expect_identifier()?;

//         // PARAMETERS
//         let mut params = Vec::new();
//         while matches!(self.current, TokenKind::OpenParen) {
//             self.advance()?;
//             let param_name = self.expect_identifier()?;
//             self.expect(TokenKind::Colon)?;
//             let param_type = self.parse_term()?;
//             self.expect(TokenKind::CloseParen)?;
//             params.push((param_name, param_type));
//         }

//         // RETURN TYPE

//         self.expect(TokenKind::Colon)?;
//         let _ty = self.parse_term()?;

//         // BODY
//         self.expect(TokenKind::OpenBrace)?;
//         let body = self.parse_term()?;
//         self.expect(TokenKind::CloseBrace)?;

//         let mut lambda = body;
//         for (param_name, param_type) in params.into_iter().rev() {
//             lambda = Term::Lambda {
//                 parameter: Identifier { name: param_name },
//                 r#type: Box::new(param_type),
//                 body: Box::new(lambda),
//             }
//         }

//         Ok((name, lambda))
//     }

//     fn parse_inductive(&mut self) -> Result<Inductive, ParseError> {
//         self.expect(TokenKind::Inductive)?;
//         let name = self.expect_identifier()?;

//         self.expect(TokenKind::Colon)?;
//         let ty = self.parse_term()?;

//         self.expect(TokenKind::OpenBrace)?;

//         let mut constructors = Vec::new();
//         let mut index = 0;
//         while !matches!(self.current, TokenKind::CloseBrace) {
//             let cname = self.expect_identifier()?;
//             self.expect(TokenKind::Colon)?;
//             let cty = self.parse_term()?;
//             constructors.push(Constructor {
//                 name: cname,
//                 r#type: cty,
//                 index,
//             });
//             index += 1;

//             if matches!(self.current, TokenKind::Comma) {
//                 self.advance()?;
//             }
//         }
//         self.expect(TokenKind::CloseBrace)?;

//         let eliminator = ConstName(format!("elim_{}", name));
//         Ok(Inductive {
//             name,
//             r#type: ty,
//             constructors,
//             eliminator,
//         })
//     }

//     fn parse_term(&mut self) -> Result<Term, ParseError> {
//         let mut lhs = match &self.current {
//             TokenKind::Type => {
//                 self.advance()?;
//                 Ok(Term::Universe(Universe::Type(0)))
//             }
//             TokenKind::Identifier(name) => {
//                 let id = Identifier { name: name.clone() };
//                 self.advance()?;
//                 Ok(Term::Identifier(id))
//             }
//             TokenKind::OpenParen => {
//                 self.advance()?;
//                 let t = self.parse_term()?;
//                 self.expect(TokenKind::CloseParen)?;
//                 Ok(t)
//             }
//             TokenKind::OpenBracket => {
//                 self.advance()?;
//                 let inner = self.parse_term()?;
//                 self.expect(TokenKind::CloseBracket)?;

//                 Ok(Term::App {
//                     function: Box::new(Term::Const(ConstName("Array".into()))),
//                     argument: Box::new(inner),
//                 })
//             }
//             _ => Err(ParseError::ExpectedType {
//                 found: self.current.clone(),
//             }),
//         }?;

//         while matches!(self.current, TokenKind::Arrow) {
//             self.advance()?;
//             let rhs = self.parse_term()?;
//             lhs = Term::Pi {
//                 dependent: Identifier { name: "_".into() },
//                 from_type: Box::new(lhs),
//                 to_type: Box::new(rhs),
//             }
//         }

//         Ok(lhs)
//     }
// }
