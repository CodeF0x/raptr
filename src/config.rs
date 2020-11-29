use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub title: String,
    pub theme: String,
    pub copyright: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub index: Index
}
#[derive(Deserialize)]
pub struct Index {
    pub indextitle: String,
    pub hometitle: String,
    pub homesubtitle: String
}

pub fn read_config() -> Result<Config, std::io::Error> {
    let mut config_path = std::env::current_dir()?;
    config_path.push("config.toml");

    let config_str = std::fs::read_to_string(&config_path)?;

    Ok(toml::from_str(&config_str).unwrap())
}