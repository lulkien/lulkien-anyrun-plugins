use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Config {
    pub disable_notification: bool,
}
