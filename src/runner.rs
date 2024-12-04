use std::{env, path::PathBuf, process::Command};

use crate::{scrubber::DesktopEntry, Config};

const SENSIBLE_TERMINALS: &[&str] = &["kitty", "foot", "alacritty", "wezterm", "wterm"];

pub fn run_entry(entry: &DesktopEntry, config: &Config) {
    if entry.term {
        run_terminal_entry(entry, config);
    } else {
        run_normal_entry(entry);
    }
}

fn run_terminal_entry(entry: &DesktopEntry, config: &Config) {
    if let Some(term) = &config.terminal {
        if run_in_terminal(entry.exec.trim(), term) {
            return;
        }
        eprintln!("Failed to run {} in {}", &entry.exec, term);
    }

    for term in SENSIBLE_TERMINALS {
        if run_in_terminal(entry.exec.trim(), term) {
            return;
        }
    }
    eprintln!("Failed to run {} in list terminals", &entry.exec);
}

fn run_in_terminal(execute: &str, terminal: &str) -> bool {
    Command::new("uwsm")
        .arg("app")
        .arg("--")
        .arg(terminal)
        .arg("-e")
        .arg(execute)
        .spawn()
        .is_ok()
}

fn run_normal_entry(entry: &DesktopEntry) {
    let current_dir = env::current_dir().unwrap_or(PathBuf::from("~"));
    if Command::new("uwsm")
        .arg("app")
        .arg("--")
        .arg(entry.exec.trim())
        .current_dir(
            entry
                .path
                .as_deref()
                .filter(|p| p.exists())
                .unwrap_or(&current_dir),
        )
        .spawn()
        .is_err()
    {
        eprintln!("Failed to run {}.", entry.exec);
    }
}
