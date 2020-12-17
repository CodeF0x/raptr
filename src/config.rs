use serde_derive::Deserialize;
use chrono::prelude::*;

#[derive(Deserialize)]
pub struct Config {
    pub meta: SitewideMetaData,
}

#[derive(Deserialize)]
pub struct SitewideMetaData {
    pub last_generated_date: String,
    pub keywords: Vec<String>,
    pub description: String,
    pub copyright: String,
    pub page_name: String,
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
    config.meta.last_generated_date = Utc::today().format("%d.%m.%Y").to_string();
    // config.is_blog = false;
    
    Ok(config)
}