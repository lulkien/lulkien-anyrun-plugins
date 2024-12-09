use std::process::Command;

use crate::{types::ApplicationDesktopEntry, Config};

#[allow(unused)]
pub fn run_entry(entry: &ApplicationDesktopEntry, config: &Config) {
    if Command::new("uwsm")
        .arg("app")
        .arg("--")
        .arg(&entry.entry_name)
        .spawn()
        .is_err()
    {
        eprintln!("Failed to run {}.", entry.entry_name);
    }
}
