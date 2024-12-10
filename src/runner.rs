use crate::types::{ApplicationDesktopEntry, LaunchFreq};

pub fn start_entry(entry: &ApplicationDesktopEntry, state: &mut LaunchFreq) {
    if std::process::Command::new("uwsm")
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
