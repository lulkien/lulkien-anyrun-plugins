use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub show_description: bool,
    pub max_entries: usize,
    pub highlight_color: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_description: false,
            max_entries: 7,
            highlight_color: String::from("red"),
        }
    }
}
