use super::{ApplicationDesktopEntry, Config, LaunchFreq};

pub struct State {
    pub config: Config,
    pub entries: Vec<ApplicationDesktopEntry>,
    pub cache: LaunchFreq,
}
