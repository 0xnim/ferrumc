use dashmap::DashMap;
use crate::statics::get_whitelist;
use ferrumc_general_purpose::paths::get_root_path;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct WhitelistEntry {
    uuid: String,
    name: String,
}

pub fn read_whitelist_file() -> DashMap<u128, String> {
    let whitelist_map = DashMap::new();
    let whitelist_location = get_root_path().join("whitelist.dsv");

    if !whitelist_location.exists() {
        info!("Whitelist file not found. Creating a new one.");
        save_blank_whitelist();
        return whitelist_map;
    }

    let file = match File::open(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open whitelist file: {e}");
            return whitelist_map;
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                let parts: Vec<&str> = line_content.split(',').collect(); // Assuming comma as the delimiter
                if parts.len() == 2 {
                    if let Ok(uuid) = Uuid::parse_str(parts[0]) {
                        whitelist_map.insert(uuid.as_u128(), parts[1].to_string());
                    }
                } else {
                    error!("Invalid line format in whitelist file: {line_content}");
                }
            }
            Err(e) => {
                error!("Error reading line from whitelist file: {e}");
                break;
            }
        }
    }

    whitelist_map
}

pub fn load_whitelist() {
    let whitelist_map = read_whitelist_file();
    let whitelist = get_whitelist();
    whitelist.clear();
    for entry in whitelist_map.into_iter() {
        whitelist.insert(entry.0, entry.1);
    }
}

pub fn save_whitelist_to_file(whitelist: DashMap<u128, String>) {
    let whitelist_location = get_root_path().join("whitelist.dsv");

    let mut file = match File::create(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to create whitelist file: {e}");
            return;
        }
    };

    for entry in whitelist.iter() {
        let line = format!("{},{}\n", Uuid::from_u128(*entry.key()).hyphenated(), entry.value());
        if let Err(e) = file.write_all(line.as_bytes()) {
            error!("Failed to write to whitelist file: {e}");
            break;
        }
    }
}

pub fn save_blank_whitelist() {
    let whitelist_location = get_root_path().join("whitelist.dsv");

    if let Err(e) = File::create(&whitelist_location) {
        error!("Failed to save whitelist: {e}");
    }

    let whitelist_map: DashMap<u128, String> = DashMap::new();
    whitelist_map.insert(0, "Player".to_string());
    save_whitelist_to_file(whitelist_map);
}

pub fn list_whitelist() -> Vec<(u128, String)> {
    get_whitelist()
        .iter()
        .map(|entry| (*entry.key(), entry.value().clone()))
        .collect()
}

/// This function will both add a player to the whitelist if it doesn't exist and update the name if it does.
pub fn update_whitelist(uuid: Uuid, name: String) -> String {
    let whitelist = get_whitelist();
    let new_name = name.clone();
    let old_name = whitelist.insert(uuid.as_u128(), name);
    save_whitelist_to_file(whitelist.clone());
    old_name.unwrap_or_else(|| new_name)
}

pub fn remove_from_whitelist(uuid: Uuid) {
    let whitelist = get_whitelist();
    whitelist.remove(&uuid.as_u128());
    save_whitelist_to_file(whitelist.clone());
}

pub fn reload_whitelist() {
    load_whitelist();
}

pub fn check_whitelist(uuid: Uuid) -> bool {
    let whitelist = get_whitelist();
    whitelist.contains_key(&uuid.as_u128())
}
