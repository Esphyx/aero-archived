mod app;
mod config;
mod editor;

use std::path::Path;

use app::App;
use config::{Config, Theme};
use editor::{Editor, buffer::EditorBuffer};

pub fn run() {
    let project = "example".to_string();

    let main_file_path = Path::new(&project).join("src/main.aero");

    let editor_buffer = if main_file_path.is_file() {
        EditorBuffer::load_from_file(main_file_path).unwrap_or_else(|_| EditorBuffer::new())
    } else {
        EditorBuffer::new()
    };

    let config = Config::default();

    let mut app = App::new(&config, editor_buffer);

    let _ = ratatui::run(|terminal| app.run(terminal));
}
