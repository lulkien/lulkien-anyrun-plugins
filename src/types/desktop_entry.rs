use crate::utils;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct ApplicationDesktopEntry {
    pub entry_name: String,
    pub title: String,
    pub exec: String,
    pub icon: Option<String>,
    pub desc: Option<String>,
}

impl ApplicationDesktopEntry {
    pub fn verify_entry<P: AsRef<Path>>(path: P) -> Option<ApplicationDesktopEntry> {
        let file = File::open(&path).ok()?;

        let reader = BufReader::new(file);
        let mut found_tag = false;

        let mut title: Option<String> = None;
        let mut exec: Option<String> = None;
        let mut icon: Option<String> = None;
        let mut desc: Option<String> = None;

        let entry_name = path.as_ref().file_name()?.to_string_lossy().into_owned();

        for line in reader.lines() {
            let line = line.ok()?;

            if &line == "[Desktop Entry]" {
                found_tag = true;
                continue;
            }

            if !found_tag {
                continue;
            }

            if line.starts_with('[') {
                break;
            }

            if let Some((key, value)) = line.split_once("=") {
                match key {
                    "Type" if value != "Application" => return None,
                    "Terminal" if value != "false" => return None,
                    "NoDisplay" | "Hidden" if value != "false" => return None,
                    "Exec" => exec = utils::get_exec_name(value),
                    "Name" => title = Some(value.to_string()),
                    "Icon" => icon = Some(value.to_string()),
                    "Comment" => desc = Some(value.to_string()),
                    _ => {}
                }
            }
        }

        title.as_ref()?;
        exec.as_ref()?;

        Some(ApplicationDesktopEntry {
            entry_name,
            title: title.unwrap(),
            exec: exec.unwrap(),
            icon,
            desc,
        })
    }
}
