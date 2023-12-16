use std::{fs::File, io::Read};

use super::create_directory::create_directory;
use crate::Keys;

/// Gets The Discord Bot Token From The Config File.
pub fn get_key_from_json() -> Keys {
    create_directory("config/keys.json");

    let mut key_file = File::open("config/keys.json").expect("Unable to find keys.json");

    let mut contents = String::new();
    key_file
        .read_to_string(&mut contents)
        .expect("Unable to read keys.json");

    let keys: Keys = serde_json::from_str(&contents).expect("Unable to parse keys.json");

    return keys;
}
