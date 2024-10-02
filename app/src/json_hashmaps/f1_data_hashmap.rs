use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use std::error::Error;
use crate::utils::get_current_dir;



pub fn get_f1_data() -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let current_dir = get_current_dir();
    let f1_data_path = current_dir.join("data/f1_data.json");
    let f1_data = fs::read_to_string(f1_data_path)?;
    let f1_data: HashMap<String, Value> = serde_json::from_str(&f1_data)?;
    Ok(f1_data)
}

// This function receives a Hashmap and writes it to a file with this name: f1_data.json
pub fn write_json_to_file(hashmap: &HashMap<String, Value>) {
    let pretty_json = serde_json::to_string_pretty(hashmap).unwrap();
    let current_dir = get_current_dir();
    let f1_data_path = current_dir.join("data/f1_data.json");
    std::fs::write(f1_data_path, pretty_json).unwrap();
}
