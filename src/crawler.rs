use std::{collections::HashSet, env, ffi::OsStr, fs, path::Path};

use crate::{types::ApplicationDesktopEntry, Config};

#[allow(unused)]
pub fn crawler(config: &Config) -> Vec<ApplicationDesktopEntry> {
    let mut application_entries = Vec::new();
    let mut existed_entries: HashSet<String> = HashSet::new();

    let user_path = match env::var("XDG_DATA_HOME") {
        Ok(data_home) => {
            format!("{}/applications/", data_home)
        }
        Err(_) => {
            format!(
                "{}/.local/share/applications/",
                env::var("HOME").expect("Is HOME set?")
            )
        }
    };

    for scan_dir in &[
        &user_path,
        "/usr/local/share/applications",
        "/usr/share/applications",
    ] {
        if let Ok(entries) = load_desktop_entries_from(scan_dir, &mut existed_entries) {
            application_entries.extend(entries);
        }
    }

    application_entries
}

fn load_desktop_entries_from<P: AsRef<Path>>(
    dir: P,
    existed_entries: &mut HashSet<String>,
) -> Result<Vec<ApplicationDesktopEntry>, Box<dyn std::error::Error>> {
    let mut desktop_files: Vec<ApplicationDesktopEntry> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension() != Some(OsStr::new("desktop")) {
            continue;
        }

        if let Some(entry) = ApplicationDesktopEntry::verify_entry(path) {
            if existed_entries.insert(entry.entry_name.clone()) {
                desktop_files.push(entry);
            } else {
                println!("Found before: {}", entry.entry_name);
            }
        }
    }

    Ok(desktop_files)
}
