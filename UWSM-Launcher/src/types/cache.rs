use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufReader, BufWriter, Read},
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LaunchFreq(HashMap<String, u32>);

static CACHE_PATH: Lazy<String> = Lazy::new(|| {
    let cache_dir = env::var("XDG_CACHE_HOME")
        .unwrap_or_else(|_| format!("{}/.cache", env::var("HOME").expect("Is HOME set?")));

    if !fs::metadata(&cache_dir)
        .map(|m| m.is_dir())
        .unwrap_or(false)
    {
        if let Err(e) = fs::create_dir_all(&cache_dir) {
            eprintln!("Cannot create directory: {cache_dir} | Error: {e}");
        }
    }

    format!("{}/uwsm-launcher-cache.ron", cache_dir)
});

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
        let entry = self.0.entry(key.to_string()).or_insert(0);
        *entry += 1;

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

    pub fn data(&self) -> &HashMap<String, u32> {
        &self.0
    }
}
