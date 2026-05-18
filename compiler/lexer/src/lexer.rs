use diagnostics::{Diagnostic, Location, Span};

use super::token::{Token, TokenKind};

#[derive(Debug)]
pub enum LexicalError {
    UnknownToken,
}

pub struct Cursor {
    input: Vec<char>,
    pos: Location,
    diagnostics: Vec<Diagnostic>,
}

impl Cursor {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            pos: Location::default(),
            diagnostics: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos.index + 1).copied()
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos.index).copied()
    }

    fn advance(&mut self) {
        if let Some(c) = self.current() {
            self.pos.index += 1;
            if c == '\n' {
                self.pos.line += 1;
                self.pos.column = 0;
            } else {
                self.pos.column += 1;
            }
        }
    }

    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos.index..]
            .iter()
            .zip(s.chars())
            .all(|(a, b)| *a == b)
    }
}

pub struct Lexer {
    cursor: Cursor,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect();
        Self {
            cursor: Cursor::new(input),
        }
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let start = self.cursor.pos;

        let mut id = String::new();
        while let Some(c) = self.cursor.current() {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
                self.cursor.advance();
            } else {
                break;
            }
        }

        let end = self.cursor.pos;

        let kind = TokenKind::from_str(&id).unwrap_or(TokenKind::Identifier);

        let mut token = Token::new(kind, Span { start, end });
        token.lexeme = Some(id);
        token
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.cursor.current(), Some(c) if c.is_whitespace()) {
            self.cursor.advance();
        }
    }

    fn comment(&mut self) -> Token {
        let start = self.cursor.pos;

        self.cursor.advance();

        while let Some(c) = self.cursor.current() {
            if c == '\n' {
                break;
            }
            self.cursor.advance();
        }

        let end = self.cursor.pos;

        Token::new(TokenKind::Comment, Span { start, end })
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

        let Some(c) = self.cursor.current() else {
            let pos = self.cursor.pos;
            return Ok(Token::new(
                TokenKind::EoF,
                Span {
                    start: pos,
                    end: pos,
                },
            ));
        };

        let multi = [
            ("->", TokenKind::Arrow),
            ("=>", TokenKind::FatArrow),
            (":=", TokenKind::Assign),
        ];

        for (pattern, kind) in multi {
            if self.cursor.starts_with(pattern) {
                let start = self.cursor.pos;
                self.cursor.advance_n(pattern.len());
                let end = self.cursor.pos;
                return Ok(Token::new(kind, Span { start, end }));
            }
        }

        if c == '#' {
            return Ok(self.comment());
        }

        if c.is_alphabetic() || c == '_' {
            return Ok(self.identifier_or_keyword());
        }

        let kind = match c {
            ';' => TokenKind::SemiColon,
            '=' => TokenKind::Assign,
            '|' => TokenKind::Pipe,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '>' => TokenKind::Arrow,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            _ => return Err(LexicalError::UnknownToken),
        };

        let start = self.cursor.pos;
        self.cursor.advance();
        let end = self.cursor.pos;

        Ok(Token::new(kind, Span { start, end }))
    }
}
