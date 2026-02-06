use std::{fs, io, path::PathBuf};

use ratatui::layout::Offset;

use super::super::compiler::front_end::grammar::{lexer::LexicalAnalyzer, token::Token};

struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub struct Editor {
    pub content: Vec<String>,
    cursor: Cursor,
    file_path: Option<PathBuf>,
    pub tokens: Vec<Token>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            content: vec![String::new()],
            cursor: Cursor::new(),
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
            cursor: Cursor::new(),
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

    pub fn cursor_offset(&self) -> Offset {
        Offset {
            x: self.cursor.x as i32,
            y: self.cursor.y as i32,
        }
    }

    pub fn new_line(&mut self) {
        if let Some(line) = self.content.get_mut(self.cursor.y) {
            let new_line = line.split_off(self.cursor.x);
            self.cursor.y += 1;
            self.cursor.x = 0;
            self.content.insert(self.cursor.y, new_line);
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some(line) = self.content.get_mut(self.cursor.y) {
            line.insert(self.cursor.x, c);
            self.cursor.x += 1;
        }
    }

    pub fn indent(&mut self) {
        for _ in 0..4 {
            self.insert_char(' ');
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor.x > 0 {
            if let Some(line) = self.content.get_mut(self.cursor.y) {
                line.remove(self.cursor.x - 1);
                self.cursor.x -= 1;
            }
        } else if self.cursor.y > 0 {
            let current_line = self.content.remove(self.cursor.y);
            self.cursor.y -= 1;
            if let Some(previous_line) = self.content.get_mut(self.cursor.y) {
                let length = previous_line.len();
                previous_line.push_str(&current_line);
                self.cursor.x = length;
            }
        }
    }

    pub fn up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;
            let line_length = self.content[self.cursor.y].len();
            self.cursor.x = self.cursor.x.min(line_length);
        }
    }

    pub fn down(&mut self) {
        if self.cursor.y + 1 < self.content.len() {
            self.cursor.y += 1;
            let line_length = self.content[self.cursor.y].len();
            self.cursor.x = self.cursor.x.min(line_length);
        }
    }

    pub fn left(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        } else if self.cursor.y > 0 {
            self.cursor.y -= 1;
            self.cursor.x = self.content[self.cursor.y].len()
        }
    }

    pub fn right(&mut self) {
        if let Some(line) = self.content.get_mut(self.cursor.y) {
            if self.cursor.x < line.len() {
                self.cursor.x += 1;
            } else if self.cursor.y + 1 < self.content.len() {
                self.cursor.y += 1;
                self.cursor.x = 0;
            }
        }
    }
}
