use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub show_description: bool,
    pub max_entries: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_description: false,
            max_entries: 7,
        }
    }
}
