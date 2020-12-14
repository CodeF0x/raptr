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

pub fn read_config() -> Result<Config, std::io::Error> {
    let mut config_path = std::env::current_dir()?;
    config_path.push("config.toml");

    let config_str = std::fs::read_to_string(&config_path)?;

    let mut config: Config = toml::from_str(&config_str).unwrap();
    config.meta.last_edited_date = Utc::today().format("%d.%m.%Y").to_string().into();
    Ok(config)
}