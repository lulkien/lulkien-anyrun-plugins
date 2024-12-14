use crate::{config::Config, entry::PowerEntry, notify::Notify};

#[allow(unused)]
pub struct State {
    pub config: Config,
    pub entries: Vec<PowerEntry>,
}

impl State {
    pub fn new(config: Config) -> Self {
        State {
            config,
            entries: vec![
                PowerEntry {
                    name: "Stop UWSM".into(),
                    command: "uwsm".into(),
                    args: vec!["stop".into()],
                    icon: "system-log-out-symbolic".into(),
                    keywords: "exit quit".into(),
                    notify: None,
                    id: 0,
                },
                PowerEntry {
                    name: "Logout".into(),
                    command: "loginctl".into(),
                    args: vec!["terminate-user".into(), whoami::username()],
                    icon: "system-suspend-symbolic".into(),
                    keywords: "terminate".into(),
                    notify: None,
                    id: 1,
                },
                PowerEntry {
                    name: "Reboot".into(),
                    command: "systemctl".into(),
                    args: vec!["reboot".into()],
                    icon: "system-restart-symbolic".into(),
                    keywords: "restart reset".into(),
                    notify: Some(Notify {
                        summary: "System".into(),
                        body: "Rebooting...".into(),
                        icon: "system-reboot".into(),
                        timeout: 10,
                    }),
                    id: 2,
                },
                PowerEntry {
                    name: "Shutdown".into(),
                    command: "systemctl".into(),
                    args: vec!["poweroff".into()],
                    icon: "system-shutdown-symbolic".into(),
                    keywords: "off".into(),
                    notify: Some(Notify {
                        summary: "System".into(),
                        body: "Shutting down...".into(),
                        icon: "system-shutdown".into(),
                        timeout: 10,
                    }),
                    id: 3,
                },
            ],
        }
    }
}
