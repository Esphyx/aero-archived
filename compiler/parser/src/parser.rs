use diagnostics::{Diagnostic, Severity};
use lexer::token::{Token, TokenKind};

use crate::error::ParseError;

pub struct Parser {
    pub input: String,
    pub position: usize,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, input: String) -> Result<Self, ParseError> {
        Ok(Self {
            tokens,
            position: 0,
            input,
            diagnostics: Vec::new(),
        })
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn peek_kind(&self) -> TokenKind {
        self.tokens
            .get(self.position)
            .map(|t| t.kind)
            .unwrap_or(TokenKind::EoF)
    }

    pub fn advance(&mut self) {
        self.position += 1;

        while let Some(tok) = self.tokens.get(self.position) {
            if tok.kind != TokenKind::Comment {
                break;
            }
            self.position += 1;
        }
    }

    pub fn synchronize(&mut self) {
        while let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::Fn
                | TokenKind::Inductive
                | TokenKind::Match
                | TokenKind::Lambda
                | TokenKind::Forall
                | TokenKind::EoF => break,

                _ => self.advance(),
            }
        }
    }

    pub fn error(&mut self, message: impl Into<String>) {
        if let Some(tok) = self.tokens.get(self.position) {
            self.diagnostics.push(Diagnostic {
                message: message.into(),
                primary_span: Some(tok.span),
                labels: Vec::new(),
                severity: Severity::Error,
            });
        } else {
            self.diagnostics.push(Diagnostic {
                message: message.into(),
                primary_span: None,
                labels: Vec::new(),
                severity: Severity::Error,
            });
        }
    }

    pub fn expect_token(&mut self, expected: TokenKind) {
        match self.peek() {
            Some(tok) if tok.kind == expected => {
                self.advance();
            }
            Some(tok) => {
                self.diagnostics.push(Diagnostic {
                    message: format!(
                        "expected `{}`, found `{}`",
                        expected.to_str(),
                        tok.kind.to_str()
                    ),
                    primary_span: Some(tok.span),
                    labels: Vec::new(),
                    severity: Severity::Error,
                });
            }
            None => {
                self.diagnostics.push(Diagnostic {
                    message: "End of file".into(),
                    primary_span: None,
                    labels: Vec::new(),
                    severity: Severity::Error,
                });
            }
        }
    }
}
