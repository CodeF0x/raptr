use serde_derive::Deserialize;
use chrono::prelude::*;
use std::fs::File;
use std::fs;

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

pub fn change_theme(theme_name: &str) -> Result<(), String> {
    let theme_file_path = std::path::Path::new(".theme");
    let theme_file_exists = theme_file_path.exists();
    let theme_dir = std::path::Path::new("./templates");
    let err_message = "Could not change theme:";

    if theme_file_exists {
        let theme = match fs::read_to_string(theme_file_path) {
            Ok(theme) => theme,
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    return Err(format!("{}: {}", err_message, err));
                } else {
                    return Err(String::from("whatever"));
                }
            }
        };

        match fs::write(theme_file_path, theme_name) {
            Ok(_) => {},
            Err(err) => return Err(format!("{}: {}", err_message, err))
        };
    } else {
        match fs::write(theme_file_path, theme_name.as_bytes()) {
            Ok(_) => {},
            Err(err) => return Err(format!("{}: {}", err_message, err))
        }

        let available_templates = fs::read_dir(theme_dir).unwrap().count();
        match available_templates {
            0 | 1 => {
                return Err(String::from("Please add at least two themes."));
            },
            _ => {
                fs::rename(theme_name, "default");
            }
        }

    }
    Ok(())
}