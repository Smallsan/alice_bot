use std::{fs::File, io::Read};

use super::create_directory::create_directory;
use crate::{Config, ParsedConfig};

pub fn load_config() -> ParsedConfig {
    create_directory("config/config.json");
    let mut config_file = File::open("config/config.json").expect("Unable to find config.json");
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
        log_channel_id: (config
            .log_channel_id
            .parse::<u64>()
            .expect("Unable to parse log_channel_id")),
        log_channel_enabled: (config
            .log_channel_enabled
            .parse::<bool>()
            .expect("Unable to parse log_channel_enabled")),
        local_logger_enabled: (config
            .local_logger_enabled
            .parse::<bool>()
            .expect("Unable to parse local_logger_enabled")),
        stalker_user_id: (config
            .stalker_user_id
            .parse::<u64>()
            .expect("Unable to parse stalker_user_id")),
        stalker_receiver_id: (config
            .stalker_receiver_id
            .parse::<u64>()
            .expect("Unable to parse stalker_receiver_id")),
        stalker_enabled: (config
            .stalker_enabled
            .parse::<bool>()
            .expect("Unable to parse stalker_enabled")),
    }
}
