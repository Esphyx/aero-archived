use std::{fs, io, path::PathBuf};

use ratatui::{
    buffer::Buffer,
    layout::{Offset, Rect},
    style::{Color, Style},
};

use super::{
    super::{
        super::compiler::front_end::grammar::{
            lexer::LexicalAnalyzer,
            token::{Token, TokenKind},
        },
        Theme,
    },
    Cursor,
};

pub struct EditorBuffer {
    pub content: Vec<String>,
    file_path: Option<PathBuf>,
    pub tokens: Vec<Token>,
}

impl EditorBuffer {
    pub fn new() -> Self {
        Self {
            content: vec![String::new()],
            file_path: None,
            tokens: Vec::new(),
        }
    }

    pub fn analyze(&mut self) {
        self.tokens = LexicalAnalyzer::new(self.content.join("\n"))
            .tokens()
            .unwrap_or(Vec::new());
    }

    pub fn load_from_file(path: PathBuf) -> io::Result<Self> {
        let content = fs::read_to_string(&path)?
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut out = Self {
            content,
            file_path: Some(path),
            tokens: Vec::new(),
        };

        out.analyze();

        Ok(out)
    }

    pub fn save_to_file(&self) -> io::Result<()> {
        if let Some(path) = &self.file_path {
            let text = self.content.join("\n");
            fs::write(path, text)?;
        }
        Ok(())
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme, cursor: &Cursor) {
        for y in 0..area.height {
            for x in 0..area.width {
                buf[(area.x + x, area.y + y)].set_bg(theme.background.primary);
            }
        }

        if cursor.y < area.height as usize {
            for x in 0..area.width {
                buf[(area.x + x, area.y + cursor.y as u16)].set_bg(theme.background.line_highlight);
            }
        }

        for (y, line) in self.content.iter().enumerate() {
            if y >= area.height as usize {
                break;
            }

            for (x, c) in line.chars().enumerate() {
                if x >= area.width as usize {
                    break;
                }

                let global_position =
                    self.content[..y].iter().map(|l| l.len() + 1).sum::<usize>() + x;
                let style = self
                    .tokens
                    .iter()
                    .find(|t| {
                        global_position >= t.position && global_position < t.position + t.length
                    })
                    .map(|t| style_for_token_kind(t.kind, theme))
                    .unwrap_or(Style::default());

                buf.set_string(area.x + x as u16, area.y + y as u16, c.to_string(), style);
            }
        }
    }
}

fn style_for_token_kind(kind: TokenKind, theme: &Theme) -> Style {
    let style = Style::default();

    let fg = match kind {
        TokenKind::Identifier => theme.identifier,
        TokenKind::Comment => theme.comment,
        TokenKind::Let | TokenKind::Inductive | TokenKind::Fn | TokenKind::Assign => theme.keyword,
        TokenKind::Type | TokenKind::U8 | TokenKind::Unit => theme.type_name,
        TokenKind::SemiColon
        | TokenKind::Colon
        | TokenKind::Comma
        | TokenKind::Arrow
        | TokenKind::OpenBrace
        | TokenKind::CloseBrace
        | TokenKind::OpenParen
        | TokenKind::CloseParen => theme.punctuation,
        TokenKind::AddU8 | TokenKind::ReadU8 | TokenKind::WriteU8 => theme.builtin,
        _ => theme.default,
    };

    style.fg(fg)
}
