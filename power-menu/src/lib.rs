use crate::{config::Config, entry::PowerEntry, state::State};

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::{anyrun_interface::HandleResult, *};
use fuzzy_matcher::FuzzyMatcher;
use std::fs;

mod config;
mod entry;
mod notify;
mod runner;
mod state;

const PLUGIN_NAME: &str = "Power Settings";
const PLUGIN_ICON: &str = "power-symbolic";

#[info]
pub fn info() -> PluginInfo {
    PluginInfo {
        name: PLUGIN_NAME.into(),
        icon: PLUGIN_ICON.into(),
    }
}

#[init]
pub fn init(config_dir: RString) -> State {
    let config_path = format!("{}/power-menu.ron", config_dir);

    let config: Config = match fs::read_to_string(&config_path) {
        Ok(content) => ron::from_str(&content).unwrap_or_else(|why| {
            eprintln!("Error parsing config: Path: {config_path} | Why: {why}");
            Config::default()
        }),
        Err(why) => {
            eprintln!("Error reading config: Path: {config_path} | Why: {why}");
            Config::default()
        }
    };

    State::new(config)
}

#[get_matches]
pub fn get_matches(input: RString, state: &State) -> RVec<Match> {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().smart_case();
    let mut entries: Vec<(&PowerEntry, i64)> = state
        .entries
        .iter()
        .filter_map(|entry| {
            let title_score: i64 = matcher.fuzzy_match(&entry.name, &input).unwrap_or(0);

            let keywords_score: i64 = matcher
                .fuzzy_match(
                    format!("{} {}", PLUGIN_NAME, &entry.keywords).as_str(),
                    &input,
                )
                .unwrap_or(0);

            let score: i64 = title_score * 3 + keywords_score;

            if score > 0 {
                Some((entry, score))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.id.cmp(&b.0.id)));

    entries
        .into_iter()
        .map(|(entry, _)| Match {
            title: entry.name.clone().into(),
            description: ROption::RNone,
            use_pango: false,
            icon: ROption::RSome(entry.icon.clone().into()),
            id: ROption::RSome(entry.id),
        })
        .collect()
}

#[handler]
pub fn handler(selection: Match, state: &mut State) -> HandleResult {
    println!("Selection id: {}", selection.id.unwrap_or(99));
    if let Some(entry) = state
        .entries
        .iter()
        .find(|entry| Some(entry.id) == selection.id.into())
    {
        runner::run(entry, &state.config);
    }
    HandleResult::Close
}
