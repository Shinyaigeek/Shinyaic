use std::fs;

pub fn load_default_css() -> String {
    fs::read_to_string("src/paint/default.css").unwrap_or("".to_string())
}
