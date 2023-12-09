use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub height: u32,
    pub width: u32,
    pub fps: u32,
}

impl Config {
    pub fn new(filename: &str) -> Self {
        let content = fs::read_to_string(filename).unwrap();
        toml::from_str(&content).unwrap()
    }
}
