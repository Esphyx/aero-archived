use crate::front_end::token::Token;

#[derive(Debug)]
pub enum LexicalError {
    UnknownToken,
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

    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
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

        match id.as_str() {
            "let" => Token::Let,
            "match" => Token::Match,
            "inductive" => Token::Inductive,
            _ => Token::Identifier(id),
        }
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

        Ok(Token::NumberLiteral(
            literal
                .parse::<u32>()
                .map_err(|_| LexicalError::ParsingNumberLiteral)?,
        ))
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexicalError> {
        self.skip_whitespace();

        let Some(c) = self.current() else {
            return Ok(Token::EndOfFile);
        };

        if self.starts_with("->") {
            self.advance_n(2);
            return Ok(Token::Arrow);
        }

        if c.is_alphabetic() || c == '_' {
            return Ok(self.identifier_or_keyword());
        }

        if c.is_ascii_digit() {
            return self.number_literal();
        }

        let token = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            ';' => Token::SemiColon,
            '=' => Token::Equals,
            '|' => Token::Pipe,
            ':' => Token::Colon,
            '.' => Token::Period,
            '>' => Token::Arrow,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            _ => return Err(LexicalError::UnknownToken),
        };

        self.advance();

        Ok(token)
    }
}
