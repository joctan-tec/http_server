use std::env;
use std::path::PathBuf;
use std::collections::HashMap;
use serde_json::Value;

pub fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap()
}

// This function receives a Hashmap value and prints it in a pretty way as JSON
pub fn print_hashmap(hashmap: &HashMap<String, Value>) {
    let pretty_json = serde_json::to_string_pretty(hashmap).unwrap();
    println!("{}", pretty_json);
}

// This function receives a String and returns the same string withouth " and \ characters
pub fn clean_string(string: String) -> String {
    let mut cleaned_string = string.replace("\"", "");
    cleaned_string = cleaned_string.replace("\\", "");
    cleaned_string
}