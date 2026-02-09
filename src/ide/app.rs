use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::{Offset, Rect},
    widgets::Widget,
};

use super::{Config, Editor, EditorBuffer};

pub struct App<'a> {
    pub editor: Editor<'a>,
    pub config: &'a Config,
    pub exit: bool,
}

impl<'a> App<'a> {
    pub fn new(config: &'a Config, buffer: EditorBuffer) -> Self {
        Self {
            editor: Editor::new(buffer, &config.editor),
            config,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        let cursor_position = frame
            .area()
            .as_position()
            .offset(self.editor.cursor_offset())
            .offset(Offset {
                x: self.editor.gutter.width as i32,
                y: 0,
            });

        frame.set_cursor_position(cursor_position);
    }

    fn save_and_exit(&mut self) -> io::Result<()> {
        self.editor.buffer.save_to_file()?;
        self.exit = true;
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Esc {
                    self.exit = true;
                } else if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('s') {
                    self.save_and_exit()?;
                } else {
                    self.editor.handle_key(key)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'a> Widget for &App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let gutter_area = Rect {
            x: area.x,
            y: area.y,
            width: self.editor.gutter.width as u16,
            height: area.height,
        };

        let editor_area = Rect {
            x: area.x + self.editor.gutter.width as u16,
            y: area.y,
            width: area.width.saturating_sub(self.editor.gutter.width as u16),
            height: area.height,
        };

        let theme = self.config.theme().expect("Theme is None");

        self.editor
            .gutter
            .render(gutter_area, buf, theme, &self.editor.cursor);
        self.editor
            .buffer
            .render(editor_area, buf, theme, &self.editor.cursor);
    }
}
