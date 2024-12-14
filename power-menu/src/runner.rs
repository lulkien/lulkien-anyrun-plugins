use notify_rust::{Notification, Timeout};

use crate::{config::Config, entry::PowerEntry};

pub fn run(entry: &PowerEntry, config: &Config) {
    if std::process::Command::new(&entry.command)
        .args(&entry.args)
        .spawn()
        .is_err()
    {
        eprintln!("Failed to run {}.", entry.name);
    }

    println!("Run {} with args {:?}", entry.command, entry.args);

    if config.disable_notification {
        return;
    }

    if let Some(notify) = &entry.notify {
        if Notification::new()
            .summary(&notify.summary)
            .body(&notify.body)
            .icon(&notify.icon)
            .timeout(Timeout::from(&notify.timeout * 1000))
            .show()
            .is_err()
        {
            eprintln!("Cannot send notification");
        }
    }
}
