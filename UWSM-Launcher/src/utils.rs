use std::{cmp::Ordering, time::SystemTime};

use crate::types::{ApplicationDesktopEntry, LaunchInfo, State};

const DURATION_1_DAY: std::time::Duration = std::time::Duration::from_secs(24 * 60 * 60);

pub fn sort_entries_and_truncate(
    entries: &mut Vec<(&ApplicationDesktopEntry, i64, Vec<usize>)>,
    state: &State,
) {
    let one_day_ago = SystemTime::now() - DURATION_1_DAY;

    entries.sort_by(|first, second| {
        let first_score = first.1;
        let second_score = second.1;

        let first_entry = &first.0;
        let second_entry = &second.0;

        first_score
            .cmp(&second_score) // Sort by score
            .then_with(|| {
                let first = state
                    .cache
                    .as_ref()
                    .get(&first_entry.entry_name)
                    .cloned()
                    .unwrap_or_default();

                let second = state
                    .cache
                    .as_ref()
                    .get(&second_entry.entry_name)
                    .cloned()
                    .unwrap_or_default();

                cmp_optimized(&first, &second, one_day_ago)
            }) // Sort by launch info
            .then_with(|| first_entry.title.cmp(&second_entry.title).reverse()) // Sort by name
            .reverse() // Reverse everything to get priority list
    });

    entries.truncate(state.config.max_entries);
}

pub fn cmp_optimized(first: &LaunchInfo, second: &LaunchInfo, time_point: SystemTime) -> Ordering {
    let first_recent = first.last_launch.duration_since(time_point).is_ok();
    let second_recent = second.last_launch.duration_since(time_point).is_ok();

    match (first_recent, second_recent) {
        (true, true) => first.last_launch.cmp(&second.last_launch), // in the last 24 hours, latest launch -> Greater
        (false, false) => first.launch_count.cmp(&second.launch_count), // out of the last 24 hours, greater launch count -> Greater
        (true, false) => Ordering::Greater,                             // no comment
        (false, true) => Ordering::Less,                                // no comment
    }
}

pub fn get_exec_name(input: &str) -> Option<String> {
    if input.trim().is_empty() {
        None
    } else if input.contains('/') {
        input
            .split('/')
            .last()
            .and_then(|s| s.split_whitespace().next())
            .map(|s| s.to_string())
    } else {
        input.split_whitespace().next().map(|s| s.to_string())
    }
}
