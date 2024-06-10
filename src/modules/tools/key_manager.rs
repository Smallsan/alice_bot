use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

use crate::Keys;

pub fn get_key_from_json() -> Keys {
    let path = Path::new("config");
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create directory");
    }

    let file_path = path.join("keys.json");
    let mut key_file = File::open(&file_path).unwrap_or_else(|_| File::create(&file_path).expect("Failed to create file"));

    let mut contents = String::new();
    key_file
        .read_to_string(&mut contents)
        .expect("Unable to read keys.json");

    let keys: Keys = serde_json::from_str(&contents).expect("Unable to parse keys.json");

    return keys;
}