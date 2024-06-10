use std::fs;
use std::path::Path;
use std::{fs::File, io::Read};
use serde::{Deserialize, Serialize};

use crate::ParsedConfig;

#[derive(Serialize, Deserialize)]
pub struct Config {
    log_channel_id: String,
    log_channel_enabled: String,
    local_logger_enabled: String,
    stalker_user_id: String,
    stalker_receiver_id: String,
    stalker_enabled: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_channel_id: "".to_string(),
            log_channel_enabled: "false".to_string(),
            local_logger_enabled: "false".to_string(),
            stalker_user_id: "".to_string(),
            stalker_receiver_id: "".to_string(),
            stalker_enabled: "false".to_string(),
        }
    }
}

pub fn load_config() -> ParsedConfig {
    let path = Path::new("config");
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create directory");
    }

    let file_path = path.join("config.json");
    let mut config_file = File::open(&file_path).unwrap_or_else(|_| {
        let file = File::create(&file_path).expect("Failed to create file");
        let config = Config::default(); // assuming Config has a default
        let config_json = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
        fs::write(&file_path, config_json).expect("Unable to write data");
        file
    });

    let mut contents = String::new();
    config_file
        .read_to_string(&mut contents)
        .expect("Unable to read config.json");

    let config: Config = serde_json::from_str(&contents).expect("Unable to parse config.json");

    let parsed_config = config_parser(config);

    return parsed_config;
}

fn config_parser(config: Config) -> ParsedConfig {
    ParsedConfig {
        log_channel_id: parse_field(config.log_channel_id, "log_channel_id"),
        log_channel_enabled: parse_field(config.log_channel_enabled, "log_channel_enabled"),
        local_logger_enabled: parse_field(config.local_logger_enabled, "local_logger_enabled"),
        stalker_user_id: parse_field(config.stalker_user_id, "stalker_user_id"),
        stalker_receiver_id: parse_field(config.stalker_receiver_id, "stalker_receiver_id"),
        stalker_enabled: parse_field(config.stalker_enabled, "stalker_enabled"),
    }
}

fn parse_field<T: std::str::FromStr>(field: String, field_name: &str) -> T {
    field.parse::<T>().unwrap_or_else(|_| panic!("Unable to parse {}", field_name))
}