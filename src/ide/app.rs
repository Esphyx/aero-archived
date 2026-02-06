use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use super::{super::compiler::front_end::grammar::token::TokenKind, Config, Editor, Theme};

pub struct App {
    pub editor: Editor,
    pub config: Option<Config>,
    pub exit: bool,
}

impl App {
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
            .offset(self.editor.cursor_offset());

        frame.set_cursor_position(cursor_position);
    }

    fn reload(&mut self) {
        todo!()
    }

    fn save_and_exit(&mut self) -> io::Result<()> {
        self.editor.save_to_file()?;
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
                } else if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('r') {
                    self.reload();
                } else {
                    match key.code {
                        KeyCode::Char(c) => self.editor.insert_char(c),
                        KeyCode::Enter => self.editor.new_line(),
                        KeyCode::Left => self.editor.left(),
                        KeyCode::Right => self.editor.right(),
                        KeyCode::Up => self.editor.up(),
                        KeyCode::Down => self.editor.down(),
                        KeyCode::Backspace => self.editor.delete_char(),
                        KeyCode::Tab => self.editor.indent(),
                        _ => {}
                    }
                    self.editor.analyze();
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn style_for_token_kind(kind: TokenKind, theme: &Theme) -> Style {
    let style = Style::default();

    let fg = Theme::to_color(match kind {
        TokenKind::Identifier => &theme.identifier,
        TokenKind::Comment => &theme.comment,
        TokenKind::Type
        | TokenKind::U8
        | TokenKind::Unit
        | TokenKind::Fn
        | TokenKind::Let
        | TokenKind::Inductive
        | TokenKind::Assign => &theme.type_name,
        TokenKind::SemiColon
        | TokenKind::Colon
        | TokenKind::Comma
        | TokenKind::Arrow
        | TokenKind::OpenBrace
        | TokenKind::CloseBrace
        | TokenKind::OpenParen
        | TokenKind::CloseParen => &theme.punctuation,
        _ => &theme.default,
    })
    .unwrap_or(Color::default());

    style.fg(fg)
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (y, line) in self.editor.content.iter().enumerate() {
            if y >= area.height as usize {
                break;
            }

            for (x, c) in line.chars().enumerate() {
                if x >= area.width as usize {
                    break;
                }

                let global_position = self.editor.content[..y]
                    .iter()
                    .map(|l| l.len() + 1)
                    .sum::<usize>()
                    + x;
                let style = self
                    .editor
                    .tokens
                    .iter()
                    .find(|t| {
                        global_position >= t.position && global_position < t.position + t.length
                    })
                    .map(|t| {
                        self.config.as_ref().map(|config| {
                            config
                                .theme()
                                .map(|theme| style_for_token_kind(t.kind, theme))
                        })
                    })
                    .flatten()
                    .flatten()
                    .unwrap_or(Style::default());

                buf.set_string(area.x + x as u16, area.y + y as u16, c.to_string(), style);
            }
        }
    }
}
