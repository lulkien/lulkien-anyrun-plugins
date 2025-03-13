use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, metadata, File},
    io::{BufReader, BufWriter, Read},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static CACHE_PATH: Lazy<String> = Lazy::new(|| {
    let cache_dir = env::var("XDG_CACHE_HOME")
        .unwrap_or_else(|_| format!("{}/.cache", env::var("HOME").expect("Is HOME set?")));

    if !metadata(&cache_dir).map(|m| m.is_dir()).unwrap_or(false) {
        if let Err(e) = create_dir_all(&cache_dir) {
            eprintln!("Cannot create directory: {cache_dir} | Error: {e}");
        }
    }

    format!("{}/uwsm-launcher-cache.ron", cache_dir)
});

const DURATION_30_DAYS: Duration = Duration::from_secs(30 * 24 * 60 * 60);

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct LaunchInfo {
    pub launch_count: u32,
    pub last_launch: SystemTime,
}

impl Default for LaunchInfo {
    fn default() -> Self {
        Self {
            launch_count: 0,
            last_launch: UNIX_EPOCH,
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct LaunchFreq(HashMap<String, LaunchInfo>);

impl LaunchFreq {
    pub fn parse_cache_file() -> Self {
        let cache_path = &*CACHE_PATH;

        let file = match File::open(cache_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Cannot open file {} | Error: {}", cache_path, e);
                return Self::default();
            }
        };

        let mut reader = BufReader::new(file);
        let mut data = String::new();

        if let Err(e) = reader.read_to_string(&mut data) {
            eprintln!("Cannot read file content: {cache_path} | Error: {e}");
            return Self::default();
        }

        match ron::from_str(&data) {
            Ok(cache) => cache,
            Err(e) => {
                eprintln!("Cannot parse cache file: {cache_path} | Error: {e}");
                Self::default()
            }
        }
    }

    pub fn update_cache(&mut self, key: &str) {
        let now = SystemTime::now();
        let remove_time = now - DURATION_30_DAYS;

        let entry = self.0.entry(key.to_string()).or_default();

        entry.launch_count += 1;
        entry.last_launch = now;

        self.as_mut()
            .retain(|_, info| info.last_launch.duration_since(remove_time).is_ok());

        let cache_path = &*CACHE_PATH;

        let file = match File::create(cache_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Cannot create file {} | Error: {}", cache_path, e);
                return;
            }
        };

        let writer = BufWriter::new(file);

        if let Err(e) = ron::ser::to_writer(writer, &self) {
            eprintln!("Cannot write to cache file: {cache_path} | Error: {e}");
        }
    }

    pub fn as_ref(&self) -> &HashMap<String, LaunchInfo> {
        &self.0
    }

    fn as_mut(&mut self) -> &mut HashMap<String, LaunchInfo> {
        &mut self.0
    }
}
