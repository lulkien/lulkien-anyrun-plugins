use crate::types::{ApplicationDesktopEntry, LaunchFreq};

use std::process::Command;

pub fn run_entry(entry: &ApplicationDesktopEntry, state: &mut LaunchFreq) {
    if Command::new("uwsm")
        .arg("app")
        .arg("--")
        .arg(&entry.entry_name)
        .spawn()
        .is_err()
    {
        eprintln!("Failed to run {}.", entry.entry_name);
        return;
    }

    state.update_cache(&entry.entry_name);
}
