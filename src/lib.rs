use crate::types::{Config, State};

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::{anyrun_interface::HandleResult, *};
use fuzzy_matcher::FuzzyMatcher;
use std::fs;
use types::LaunchFreq;

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
        runner::run_entry(entry, &mut state.cache);
    }

    HandleResult::Close
}

#[init]
pub fn init(config_dir: RString) -> State {
    let config: Config = match fs::read_to_string(format!("{}/applications.ron", config_dir)) {
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

    let cache = LaunchFreq::parse_cache_file();

    println!("Found: {} entries.", entries.len());
    println!("Cache: {:?}.", cache);

    State {
        config,
        entries,
        cache,
    }
}

#[get_matches]
pub fn get_matches(input: RString, state: &State) -> RVec<Match> {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().smart_case();
    let mut entries = if input.is_empty() {
        println!("All");
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
                let desc_score = match &entry.desc {
                    None => matcher.fuzzy_match(&entry.title, &input).unwrap_or(0),
                    Some(val) => matcher
                        .fuzzy_match(&format!("{} {}", &val, &entry.title).to_string(), &input)
                        .unwrap_or(0),
                };

                let mut score = desc_score;

                if entry.desc.is_some() {
                    score *= 2;
                }

                if score > 0 {
                    Some((entry, score))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    utils::prepare_display_entries(&mut entries, state);

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
        icon: "distributor-logo-archlinux".into(),
    }
}
