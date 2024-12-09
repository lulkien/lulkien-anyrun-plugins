use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::{anyrun_interface::HandleResult, *};
use fuzzy_matcher::FuzzyMatcher;
use serde::Deserialize;
use std::fs;
use types::ApplicationDesktopEntry;

mod crawler;
mod runner;
mod types;

#[derive(Deserialize)]
pub struct Config {
    show_description: bool,
    max_entries: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_description: false,
            max_entries: 10,
        }
    }
}

pub struct State {
    config: Config,
    entries: Vec<ApplicationDesktopEntry>,
}

#[handler]
pub fn handler(selection: Match, state: &State) -> HandleResult {
    if let Some(entry) = state
        .entries
        .iter()
        .find(|entry| entry.title == selection.title)
    {
        runner::run_entry(entry, &state.config);
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

    println!("Found: {} entries.", entries.len());

    State { config, entries }
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

    entries.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.title.cmp(&b.0.title)));

    entries.truncate(state.config.max_entries);
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
