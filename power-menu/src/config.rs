use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub disable_notification: bool,
    pub highlight_color: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            disable_notification: false,
            highlight_color: String::from("red"),
        }
    }
}
