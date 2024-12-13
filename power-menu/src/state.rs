use crate::{config::Config, entry::PowerEntry};

#[allow(unused)]
pub struct State {
    pub config: Config,
    pub entries: Vec<PowerEntry>,
}

impl Default for State {
    fn default() -> Self {
        State {
            config: Config::default(),
            entries: vec![
                PowerEntry {
                    name: "Stop UWSM session".into(),
                    command: "uwsm".into(),
                    args: vec!["stop".into()],
                    icon: "system-log-out-symbolic".into(),
                    keywords: "exit quit end".into(),
                    id: 0,
                },
                PowerEntry {
                    name: "Logout".into(),
                    command: "loginctl".into(),
                    args: vec!["terminate-user".into(), whoami::username()],
                    icon: "system-suspend-symbolic".into(),
                    keywords: "terminate".into(),
                    id: 1,
                },
                PowerEntry {
                    name: "Reboot".into(),
                    command: "systemctl".into(),
                    args: vec!["reboot".into()],
                    icon: "system-restart-symbolic".into(),
                    keywords: "restart reset".into(),
                    id: 2,
                },
                PowerEntry {
                    name: "Shutdown".into(),
                    command: "systemctl".into(),
                    args: vec!["poweroff".into()],
                    icon: "system-shutdown-symbolic".into(),
                    keywords: "off".into(),
                    id: 3,
                },
            ],
        }
    }
}
