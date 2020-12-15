use serde_derive::Deserialize;
use chrono::prelude::*;

#[derive(Deserialize)]
pub struct Config {
    pub index: Index,
    pub meta: Meta
}

#[derive(Deserialize)]
pub struct Index {
    pub headline: String,
    pub sub_headline: String,
    pub first_line: String,
    pub second_line: String
}

#[derive(Deserialize)]
pub struct Meta {
    pub tab_title: String,
    pub copyright: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub last_edited_date: Option<String>,
    pub author: String
}

pub fn read_config() -> Result<Config, String> {
    let mut config_path = std::env::current_dir().unwrap();
    config_path.push("config.toml");

    let config_str = match std::fs::read_to_string(&config_path) {
        Ok(conf_str) => conf_str,
        Err(_) => return Err(String::from("Error: Could not find config.toml file. Does it exist?")),
    };
    

    let mut config: Config = match toml::from_str(&config_str) {
        Ok(config) => config,
        Err(_) => return Err(String::from("Error: Could not parse config.toml file. Is it valid?")),
    };
    config.meta.last_edited_date = Utc::today().format("%d.%m.%Y").to_string().into();

    Ok(config)
}