use std::collections::HashMap;

use ratatui::style::Color;

#[derive(Debug)]
pub struct Config {
    pub selected_theme: String,
    pub editor: EditorSettings,
    pub themes: HashMap<String, Theme>,
}

impl Config {
    pub fn theme(&self) -> Option<&Theme> {
        self.themes.get(&self.selected_theme)
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut themes = HashMap::new();
        themes.insert("default".to_string(), Theme::default());

        Self {
            selected_theme: "default".to_string(),
            editor: EditorSettings::default(),
            themes,
        }
    }
}

#[derive(Debug)]
pub struct EditorSettings {
    pub line_numbers: bool,
    pub auto_indent: bool,
    pub tab_width: usize,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            line_numbers: true,
            auto_indent: true,
            tab_width: 4,
        }
    }
}

#[derive(Debug)]
pub struct Theme {
    pub gutter: GutterTheme,
    pub background: BackgroundTheme,
    pub identifier: Color,
    pub comment: Color,
    pub keyword: Color,
    pub type_name: Color,
    pub builtin: Color,
    pub punctuation: Color,
    pub default: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            gutter: GutterTheme::default(),
            background: BackgroundTheme::default(),
            identifier: Color::Rgb(255, 184, 108),
            comment: Color::Rgb(98, 114, 164),
            keyword: Color::Rgb(255, 85, 85),
            type_name: Color::Rgb(139, 233, 253),
            builtin: Color::Rgb(80, 250, 123),
            punctuation: Color::Rgb(248, 248, 242),
            default: Color::Rgb(224, 224, 224),
        }
    }
}

#[derive(Debug)]
pub struct GutterTheme {
    pub line_number: Color,
    pub line_number_selected: Color,
    pub background: Color,
}

impl Default for GutterTheme {
    fn default() -> Self {
        Self {
            line_number: Color::Rgb(98, 114, 164),
            line_number_selected: Color::Rgb(255, 121, 198),
            background: Color::Rgb(40, 42, 54),
        }
    }
}

#[derive(Debug)]
pub struct BackgroundTheme {
    pub primary: Color,
    pub line_highlight: Color,
    pub selection: Color,
}

impl Default for BackgroundTheme {
    fn default() -> Self {
        Self {
            primary: Color::Rgb(40, 42, 54),
            line_highlight: Color::Rgb(68, 71, 90),
            selection: Color::Rgb(62, 68, 81),
        }
    }
}
