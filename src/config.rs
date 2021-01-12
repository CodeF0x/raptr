use serde_derive::{Deserialize, Serialize};
use toml;
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub theme: String
}

impl Config {
    pub fn new() -> Self {
        let config_string = fs::read_to_string("./config.toml").unwrap();
        let config: Config = toml::from_str(&config_string).unwrap();

        config
    }
}