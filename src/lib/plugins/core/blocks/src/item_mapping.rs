//! Item to block state ID mapping
//!
//! This module loads the item-to-block mapping from the JSON data file.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;

const ITEM_TO_BLOCK_MAPPING_FILE: &str =
    include_str!("../../../../../../assets/data/item_to_block_mapping.json");

pub static ITEM_TO_BLOCK_MAPPING: Lazy<HashMap<i32, i32>> = Lazy::new(|| {
    let str_form: HashMap<String, String> = serde_json::from_str(ITEM_TO_BLOCK_MAPPING_FILE)
        .expect("Failed to parse item_to_block_mapping.json");
    str_form
        .into_iter()
        .map(|(k, v)| (i32::from_str(&k).unwrap(), i32::from_str(&v).unwrap()))
        .collect()
});
