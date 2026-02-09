use ratatui::{buffer::Buffer, layout::Rect, style::Style};

use super::{super::Theme, Cursor};

pub struct Gutter {
    pub width: usize,
    pub height: usize,
}

impl Gutter {
    pub fn new(height: usize) -> Self {
        Self { width: 4, height }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme, cursor: &Cursor) {
        for y in 0..area.height {
            for x in 0..area.width {
                buf[(area.x + x, area.y + y)].set_bg(theme.gutter.background);
            }
        }

        for i in 0..(area.height as usize).min(self.height) {
            let line_number = format!("{:>width$} ", i + 1, width = self.width - 1);

            let color = if cursor.y == i {
                theme.gutter.line_number_selected
            } else {
                theme.gutter.line_number
            };

            let style = Style::default().fg(color);

            buf.set_stringn(area.x, area.y + i as u16, &line_number, self.width, style);
        }
    }
}
