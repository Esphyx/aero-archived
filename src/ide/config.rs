use std::{collections::HashMap, fs, path::PathBuf};

use ratatui::style::Color;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub editor: EditorSettings,
    pub themes: HashMap<String, Theme>,
}

impl Config {
    pub fn theme(&self) -> Option<&Theme> {
        self.themes.get(&self.editor.theme)
    }

    pub fn load(path: PathBuf) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }
}

#[derive(Deserialize, Debug)]
pub struct EditorSettings {
    pub theme: String,
    pub line_numbers: bool,
    pub auto_indent: bool,
    pub tab_width: usize,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub identifier: String,
    pub comment: String,
    pub keyword: String,
    pub type_name: String,
    pub punctuation: String,
    pub default: String,
}

impl Theme {
    pub fn to_color(hex: &String) -> Option<Color> {
        hex.strip_prefix('#')
            .filter(|h| h.len() == 6)
            .and_then(|h| {
                u32::from_str_radix(h, 16)
                    .map(|rgb| {
                        let r = ((rgb >> 16) & 0xFF) as u8;
                        let g = ((rgb >> 8) & 0xFF) as u8;
                        let b = (rgb & 0xFF) as u8;
                        Color::Rgb(r, g, b)
                    })
                    .ok()
            })
    }
}
