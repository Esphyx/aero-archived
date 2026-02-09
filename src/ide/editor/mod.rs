pub mod buffer;
pub mod cursor;
pub mod gutter;

use std::io;

use buffer::EditorBuffer;
use cursor::Cursor;
use gutter::Gutter;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Offset,
};

use super::config::EditorSettings;

pub struct Editor<'a> {
    pub gutter: Gutter,
    pub cursor: Cursor,
    pub buffer: EditorBuffer,
    pub settings: &'a EditorSettings,
}

impl<'a> Editor<'a> {
    pub fn new(buffer: EditorBuffer, settings: &'a EditorSettings) -> Self {
        Self {
            gutter: Gutter::new(buffer.content.len()),
            cursor: Cursor::new(),
            buffer,
            settings,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Enter => self.new_line(),
            KeyCode::Left => self.left(),
            KeyCode::Right => self.right(),
            KeyCode::Up => self.up(),
            KeyCode::Down => self.down(),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Tab => self.indent(),
            _ => {}
        }

        self.gutter.height = self.buffer.content.len();
        self.buffer.analyze();

        Ok(())
    }

    pub fn cursor_offset(&self) -> Offset {
        Offset {
            x: self.cursor.x as i32,
            y: self.cursor.y as i32,
        }
    }

    pub fn new_line(&mut self) {
        if let Some(line) = self.buffer.content.get_mut(self.cursor.y) {
            let new_line = line.split_off(self.cursor.x);
            self.cursor.y += 1;
            self.cursor.x = 0;
            self.buffer.content.insert(self.cursor.y, new_line);
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if let Some(line) = self.buffer.content.get_mut(self.cursor.y) {
            line.insert(self.cursor.x, c);
            self.cursor.x += 1;
        }
    }

    pub fn indent(&mut self) {
        for _ in 0..self.settings.tab_width {
            self.insert_char(' ');
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor.x > 0 {
            if let Some(line) = self.buffer.content.get_mut(self.cursor.y) {
                line.remove(self.cursor.x - 1);
                self.cursor.x -= 1;
            }
        } else if self.cursor.y > 0 {
            let current_line = self.buffer.content.remove(self.cursor.y);
            self.cursor.y -= 1;
            if let Some(previous_line) = self.buffer.content.get_mut(self.cursor.y) {
                let length = previous_line.len();
                previous_line.push_str(&current_line);
                self.cursor.x = length;
            }
        }
    }

    pub fn up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;
            let line_length = self.buffer.content[self.cursor.y].len();
            self.cursor.x = self.cursor.x.min(line_length);
        }
    }

    pub fn down(&mut self) {
        if self.cursor.y + 1 < self.buffer.content.len() {
            self.cursor.y += 1;
            let line_length = self.buffer.content[self.cursor.y].len();
            self.cursor.x = self.cursor.x.min(line_length);
        }
    }

    pub fn left(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        } else if self.cursor.y > 0 {
            self.cursor.y -= 1;
            self.cursor.x = self.buffer.content[self.cursor.y].len()
        }
    }

    pub fn right(&mut self) {
        if let Some(line) = self.buffer.content.get_mut(self.cursor.y) {
            if self.cursor.x < line.len() {
                self.cursor.x += 1;
            } else if self.cursor.y + 1 < self.buffer.content.len() {
                self.cursor.y += 1;
                self.cursor.x = 0;
            }
        }
    }
}
