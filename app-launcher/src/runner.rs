use crate::types::{ApplicationDesktopEntry, LaunchFreq};

pub fn start_entry(entry: &ApplicationDesktopEntry, state: &mut LaunchFreq) {
    if std::process::Command::new("sh")
        .arg("-c")
        .arg(&entry.exec)
        .spawn()
        .is_err()
    {
        eprintln!("Failed to run {}.", entry.exec);
        return;
    }

    println!("Start: {}", &entry.entry_name);
    state.update_cache(&entry.entry_name);
}
