use dashmap::DashMap;
use crate::statics::get_whitelist;
use ferrumc_general_purpose::paths::get_root_path;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Write};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct WhitelistEntry {
    uuid: String,
    name: String,
}

pub fn read_whitelist_file() -> DashMap<u128, String> {
    let whitelist_map = DashMap::new();
    let whitelist_location = get_root_path().join("whitelist.json");

    if !whitelist_location.exists() {
        info!("Whitelist file not found. Creating a new one.");
        save_blank_whitelist();
        return whitelist_map;
    }

    let mut file = match File::open(&whitelist_location) {
        Ok(file) => file,
        Err(e) => {
            error!("Could not open whitelist file: {e}");
            return whitelist_map;
        }
    };

    let mut whitelist_str = String::new();
    if let Err(e) = file.read_to_string(&mut whitelist_str) {
        error!("Could not read whitelist file: {e}");
        return whitelist_map;
    }

    if whitelist_str.is_empty() {
        return whitelist_map;
    }

    let entries: Vec<WhitelistEntry> = match serde_json::from_str(&whitelist_str) {
        Ok(entries) => entries,
        Err(e) => {
            error!("Failed to parse whitelist JSON: {e}");
            return whitelist_map;
        }
    };

    for entry in entries {
        if let Ok(uuid) = Uuid::parse_str(&entry.uuid) {
            whitelist_map.insert(uuid.as_u128(), entry.name);
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
    let whitelist_location = get_root_path().join("whitelist.json");

    let entries: Vec<WhitelistEntry> = whitelist
        .iter()
        .map(|entry| WhitelistEntry {
            uuid: Uuid::from_u128(*entry.key()).hyphenated().to_string(),
            name: entry.value().clone(),
        })
        .collect();

    let json_data = match serde_json::to_string_pretty(&entries) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to serialize whitelist to JSON: {e}");
            return;
        }
    };

    if let Err(e) = File::create(&whitelist_location).and_then(|mut file| file.write_all(json_data.as_bytes())) {
        error!("Failed to save whitelist: {e}");
    }
}


pub fn save_blank_whitelist() {
    let whitelist_location = get_root_path().join("whitelist.json");

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
