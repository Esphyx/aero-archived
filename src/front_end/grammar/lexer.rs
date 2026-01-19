use crate::front_end::grammar::token::TokenKind;

#[derive(Debug)]
pub enum LexicalError {
    UnknownToken { string: String },
    ParsingNumberLiteral,
}

pub struct LexicalAnalizer {
    input: Vec<char>,
    position: usize,
}

impl LexicalAnalizer {
    pub fn new(input: &str) -> Self {
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

    fn identifier_or_keyword(&mut self) -> TokenKind {
        let mut id = String::new();

        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match id.as_str() {
            "let" => TokenKind::Let,
            "match" => TokenKind::Match,
            "inductive" => TokenKind::Inductive,
            "partial" => TokenKind::Partial,
            "fn" => TokenKind::Fn,
            // "u8" => Token::U8,
            // "u32" => Token::U32,
            "Type" => TokenKind::Type,
            "do" => TokenKind::Do,
            "while" => TokenKind::While,
            "loop" => TokenKind::Loop,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "external" => TokenKind::External,
            _ => TokenKind::Identifier,
        }
    }

    fn string_literal(&mut self) -> Result<TokenKind, LexicalError> {
        todo!()
    }

    fn number_literal(&mut self) -> Result<TokenKind, LexicalError> {
        let mut literal = String::new();

        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                literal.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // literal
        //     .parse::<u32>()
        //     .map_err(|_| LexicalError::ParsingNumberLiteral)?,

        Ok(TokenKind::NumberLiteral)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    fn comment(&mut self) -> TokenKind {
        let mut comment_text = String::new();

        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            }
            comment_text.push(c);
            self.advance();
        }

        TokenKind::Comment
    }

    pub fn next_token(&mut self) -> Result<TokenKind, LexicalError> {
        self.skip_whitespace();

        let Some(c) = self.current() else {
            return Ok(TokenKind::EndOfFile);
        };

        if self.starts_with("->") {
            self.advance_n(2);
            return Ok(TokenKind::Arrow);
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

        let token = match c {
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
            _ => return Err(LexicalError::UnknownToken { string: c.into() }),
        };

        self.advance();

        Ok(token)
    }
}
