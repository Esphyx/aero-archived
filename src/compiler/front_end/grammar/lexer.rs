use std::fmt::Display;

use super::token::{Token, TokenKind};

#[derive(Debug)]
pub enum LexicalError {
    UnknownToken,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for LexicalError {}

pub struct LexicalAnalyzer {
    input: Vec<char>,
    position: usize,
}

impl LexicalAnalyzer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect();
        Self { input, position: 0 }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn advance_n(&mut self, n: usize) {
        self.position += n;
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..]
            .iter()
            .zip(s.chars())
            .all(|(a, b)| *a == b)
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let mut id = String::new();

        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let kind = match id.as_str() {
            "let" => TokenKind::Let,
            "ind" => TokenKind::Inductive,
            "fn" => TokenKind::Fn,
            "u8" => TokenKind::U8,
            "prop" => TokenKind::Prop,
            "type" => TokenKind::Type,
            "unit" => TokenKind::Unit,
            "write_u8" => TokenKind::WriteU8,
            "read_u8" => TokenKind::ReadU8,
            "add_u8" => TokenKind::AddU8,
            _ => TokenKind::Identifier,
        };
        Token::new(kind, self.position - id.len(), id.len())
    }

    fn number_literal(&mut self) -> Result<Token, LexicalError> {
        let mut literal = String::new();

        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                literal.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Ok(Token::new(
            TokenKind::NumberLiteral,
            self.position - literal.len(),
            literal.len(),
        ))
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    fn comment(&mut self) -> Token {
        let mut comment_text = String::new();

        let saved_position = self.position;

        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            }
            comment_text.push(c);
            self.advance();
        }

        Token::new(
            TokenKind::Comment,
            saved_position - 2,
            comment_text.len() + 2,
        )
    }

    pub fn tokens(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            if matches!(token.kind, TokenKind::EoF) {
                tokens.push(token);
                break;
            }

            tokens.push(token);
        }

        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Result<Token, LexicalError> {
        self.skip_whitespace();

        let Some(c) = self.current() else {
            return Ok(Token::new(TokenKind::EoF, self.position, 0));
        };

        if self.starts_with("->") {
            self.advance_n(2);
            return Ok(Token::new(TokenKind::Arrow, self.position - 2, 2));
        }

        if self.starts_with("<-") {
            self.advance_n(2);
            return Ok(Token::new(TokenKind::Assign, self.position - 2, 2));
        }

        if self.starts_with("//") {
            self.advance_n(2);
            return Ok(self.comment());
        }

        if c.is_alphabetic() || c == '_' {
            return Ok(self.identifier_or_keyword());
        }

        if c.is_ascii_digit() {
            return self.number_literal();
        }

        let kind = match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            ';' => TokenKind::SemiColon,
            '=' => TokenKind::Assign,
            '|' => TokenKind::Pipe,
            ':' => TokenKind::Colon,
            '.' => TokenKind::Period,
            ',' => TokenKind::Comma,
            '>' => TokenKind::Arrow,
            '@' => TokenKind::Snail,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            _ => return Err(LexicalError::UnknownToken),
        };

        self.advance();

        Ok(Token::new(kind, self.position - 1, 1))
    }
}
