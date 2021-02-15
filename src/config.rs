//! # config.rs
//!
//! Contains the code for parsing the config.

use serde_derive::{Deserialize, Serialize};
use toml;
use std::fs;
use crate::errors;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub theme: String,
    pub copyright: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub index_title: String,
    pub home_title: String,
    pub home_sub_title: String,
    pub custom_string_one: String,
    pub custom_string_two: String,
}

impl Config {
    /// Returns a new config by parsing the config.toml file
    ///
    /// # Arguments
    ///
    /// * `verbose` - boolean if verbose mode is on
    pub fn new(verbose: bool) -> Self {
        let config_string = match fs::read_to_string("./config.toml") {
            Ok(config_str) => config_str,
            Err(err) => {
                errors::display_io_error(err, "config.toml", verbose);
                std::process::exit(1);
            }
        };
        let config: Config = toml::from_str(&config_string).unwrap();

        config
    }
}