use crate::types::{Config, LaunchFreq, State};

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::{anyrun_interface::HandleResult, *};
use fuzzy_matcher::FuzzyMatcher;

mod crawler;
mod runner;
mod types;
mod utils;

#[handler]
pub fn handler(selection: Match, state: &mut State) -> HandleResult {
    if let Some(entry) = state
        .entries
        .iter()
        .find(|entry| entry.title == selection.title)
    {
        runner::start_entry(entry, &mut state.cache);
    }

    HandleResult::Close
}

#[init]
pub fn init(config_dir: RString) -> State {
    let config: Config = match std::fs::read_to_string(format!("{}/applications.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_else(|why| {
            eprintln!("Error parsing applications plugin config: {}", why);
            Config::default()
        }),
        Err(why) => {
            eprintln!("Error reading applications plugin config: {}", why);
            Config::default()
        }
    };

    let entries = crawler::crawler(&config);

    State {
        config,
        entries,
        cache: LaunchFreq::parse_cache_file(),
    }
}

#[get_matches]
pub fn get_matches(input: RString, state: &State) -> RVec<Match> {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().smart_case();
    let mut entries = if input.is_empty() {
        state
            .entries
            .iter()
            .map(|entry| (entry, 0))
            .collect::<Vec<_>>()
    } else {
        state
            .entries
            .iter()
            .filter_map(|entry| {
                let title_score: i64 = matcher.fuzzy_match(&entry.title, &input).unwrap_or(0);

                let exec_score: i64 = matcher.fuzzy_match(&entry.exec, &input).unwrap_or(0);

                let desc_score: i64 = if state.config.show_description {
                    entry
                        .desc
                        .as_ref()
                        .map_or(0, |desc| matcher.fuzzy_match(desc, &input).unwrap_or(0))
                } else {
                    0
                };

                let score: i64 = title_score * 3 + exec_score * 2 + desc_score;

                if score > 0 {
                    Some((entry, score))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    utils::sort_entries_and_truncate(&mut entries, state);

    entries
        .into_iter()
        .map(|(entry, _)| Match {
            title: entry.title.clone().into(),
            description: if state.config.show_description {
                entry.desc.clone().map(|desc| desc.into()).into()
            } else {
                ROption::RNone
            },
            use_pango: false,
            icon: entry.icon.clone().map(|icon| icon.into()).into(),
            id: ROption::RNone,
        })
        .collect()
}

#[info]
pub fn info() -> PluginInfo {
    PluginInfo {
        name: "Uwsm launcher".into(),
        icon: "app-launcher".into(),
    }
}
