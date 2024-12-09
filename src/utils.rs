use crate::types::{ApplicationDesktopEntry, State};

pub fn prepare_display_entries(entries: &mut Vec<(&ApplicationDesktopEntry, i64)>, state: &State) {
    entries.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| {
                let freq_a = state
                    .cache
                    .data()
                    .get(&a.0.entry_name)
                    .cloned()
                    .unwrap_or(0);

                let freq_b = state
                    .cache
                    .data()
                    .get(&b.0.entry_name)
                    .cloned()
                    .unwrap_or(0);

                freq_b.cmp(&freq_a)
            })
            .then_with(|| a.0.title.cmp(&b.0.title))
    });

    entries.truncate(state.config.max_entries);
}
