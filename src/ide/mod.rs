mod app;
mod config;
mod editor;
mod project;

use std::path::Path;

use app::App;
use config::{Config, Theme};
use editor::Editor;

pub fn run() {
    let project = "example".to_string();

    let main_file_path = Path::new(&project).join("src/main.aero");
    let config_file_path = Path::new(&project).join("config.json");

    let editor = if main_file_path.exists() && main_file_path.is_file() {
        Editor::load_from_file(main_file_path).unwrap_or_else(|_| Editor::new())
    } else {
        Editor::new()
    };

    let config = if config_file_path.exists() && config_file_path.is_file() {
        Config::load(config_file_path)
    } else {
        None
    };

    let mut app = App {
        editor,
        config,
        exit: false,
    };

    let _ = ratatui::run(|terminal| app.run(terminal));
}
